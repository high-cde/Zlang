use std::fs;
use std::process::Command;

use zdos_zlang::{
    BytecodeModule, CapabilityPolicy, Compiler, VmLimits, ZVirtualMachine, ZlangError,
};

fn compile(source: &str) -> BytecodeModule {
    let mut compiler = Compiler::new();
    compiler.compile(source).expect("source should compile")
}

#[test]
fn compiler_produces_valid_deterministic_register_bytecode() {
    let module = compile("let answer = (2 + 3) * 4\nprint answer\n");
    let first = module.to_bytes().expect("encode first module");
    let second = module.to_bytes().expect("encode second module");
    assert_eq!(first, second, "bytecode encoding must be deterministic");

    let decoded = BytecodeModule::from_bytes(&first).expect("decode encoded module");
    assert_eq!(module, decoded);
}

#[test]
fn vm_executes_register_program_and_records_audit() {
    let module = compile("let answer = (2 + 3) * 4\nprint answer\n");
    let mut vm = ZVirtualMachine::new(CapabilityPolicy::console_only(), VmLimits::default());
    let report = vm.execute(&module).expect("vm should execute valid module");

    assert_eq!(report.outputs, vec!["20"]);
    assert_eq!(report.instructions_executed, module.instructions.len());
    assert!(report.audit.iter().any(|event| event.instruction == "EMIT"));
    assert_eq!(
        report.audit.last().map(|event| event.instruction),
        Some("HALT")
    );
}

#[test]
fn vm_denies_undelegated_console_capability_and_audits_decision() {
    let module = compile("print 7\n");
    let mut vm = ZVirtualMachine::new(CapabilityPolicy::deny_all(), VmLimits::default());
    let error = vm
        .execute(&module)
        .expect_err("console must be denied by default");

    assert!(matches!(error, ZlangError::CapabilityDenied(_)));
    assert!(matches!(
        vm.audit_log().last().map(|event| &event.outcome),
        Some(zdos_zlang::AuditOutcome::Denied(_))
    ));
}

#[test]
fn tampered_module_is_rejected_before_execution() {
    let module = compile("print 9\n");
    let mut bytes = module.to_bytes().expect("encode module");
    bytes[12] ^= 0x01;

    let error = BytecodeModule::from_bytes(&bytes).expect_err("tampering must fail checksum");
    assert!(matches!(error, ZlangError::Bytecode(_)));
}

#[test]
fn division_by_zero_is_controlled_and_audited() {
    let module = compile("let invalid = 4 / 0\nprint invalid\n");
    let mut vm = ZVirtualMachine::new(CapabilityPolicy::console_only(), VmLimits::default());
    let error = vm
        .execute(&module)
        .expect_err("division by zero must fail safely");

    assert!(matches!(error, ZlangError::Runtime(_)));
    assert!(vm.audit_log().last().is_some());
}

#[test]
fn cli_compiles_and_executes_verified_module() {
    let root = std::env::temp_dir().join(format!("zlang-register-vm-{}", std::process::id()));
    let source_path = root.with_extension("zl");
    let module_path = root.with_extension("zreg");
    fs::write(&source_path, "let telemetry = 40 + 2\nprint telemetry\n").expect("write source");

    let compile_output = Command::new(env!("CARGO_BIN_EXE_zlang"))
        .args([
            "compile",
            source_path.to_str().unwrap(),
            module_path.to_str().unwrap(),
        ])
        .output()
        .expect("compile command starts");
    assert!(compile_output.status.success(), "{:?}", compile_output);

    let execute_output = Command::new(env!("CARGO_BIN_EXE_zlang"))
        .args(["exec", module_path.to_str().unwrap(), "--audit"])
        .output()
        .expect("exec command starts");
    fs::remove_file(&source_path).ok();
    fs::remove_file(&module_path).ok();

    assert!(execute_output.status.success(), "{:?}", execute_output);
    let stdout = String::from_utf8_lossy(&execute_output.stdout);
    assert!(stdout.contains("42"));
    assert!(stdout.contains("AUDIT"));
}

#[test]
fn checked_in_telemetry_example_runs() {
    let module = compile(include_str!("../examples/telemetry.zl"));
    let mut vm = ZVirtualMachine::new(CapabilityPolicy::console_only(), VmLimits::default());
    let report = vm
        .execute(&module)
        .expect("checked-in example should execute");

    assert_eq!(report.outputs, vec!["72"]);
}
