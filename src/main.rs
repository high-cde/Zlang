use std::env;
use std::fs;
use std::process::ExitCode;

use zdos_zlang::{
    BytecodeModule, CapabilityPolicy, Compiler, VmLimits, ZVirtualMachine, ZlangError, ZlangResult,
};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("[ZLANG] {error}");
            ExitCode::from(error.exit_code() as u8)
        }
    }
}

fn run() -> ZlangResult<()> {
    let arguments: Vec<String> = env::args().skip(1).collect();
    if arguments.is_empty()
        || matches!(arguments.first().map(String::as_str), Some("--help" | "-h"))
    {
        print_help();
        return Ok(());
    }

    match arguments[0].as_str() {
        "run" => {
            let source_path = required(&arguments, 1, "zlang run <source.zl> [--audit]")?;
            let source = fs::read_to_string(source_path)?;
            let mut compiler = Compiler::new();
            let module = compiler.compile(&source)?;
            execute(
                module,
                arguments.iter().any(|argument| argument == "--audit"),
            )
        }
        "compile" => {
            let source_path = required(&arguments, 1, "zlang compile <source.zl> <module.zreg>")?;
            let output_path = required(&arguments, 2, "zlang compile <source.zl> <module.zreg>")?;
            let source = fs::read_to_string(source_path)?;
            let mut compiler = Compiler::new();
            let module = compiler.compile(&source)?;
            fs::write(output_path, module.to_bytes()?)?;
            println!("[ZLANG] compiled {source_path} -> {output_path}");
            Ok(())
        }
        "exec" => {
            let module_path = required(&arguments, 1, "zlang exec <module.zreg> [--audit]")?;
            let bytes = fs::read(module_path)?;
            let module = BytecodeModule::from_bytes(&bytes)?;
            execute(
                module,
                arguments.iter().any(|argument| argument == "--audit"),
            )
        }
        "inspect" => {
            let module_path = required(&arguments, 1, "zlang inspect <module.zreg>")?;
            let bytes = fs::read(module_path)?;
            let module = BytecodeModule::from_bytes(&bytes)?;
            println!("version: {}", module.version);
            println!("registers: {}", module.register_count);
            println!("capabilities: {:?}", module.capabilities);
            println!("instructions: {}", module.instructions.len());
            Ok(())
        }
        path if !path.starts_with('-') && arguments.len() == 1 => {
            let source = fs::read_to_string(path)?;
            let mut compiler = Compiler::new();
            execute(compiler.compile(&source)?, false)
        }
        _ => Err(ZlangError::Usage(
            "unknown command; run `zlang --help` for the stable CLI".to_string(),
        )),
    }
}

fn execute(module: BytecodeModule, print_audit: bool) -> ZlangResult<()> {
    let mut vm = ZVirtualMachine::new(CapabilityPolicy::console_only(), VmLimits::default());
    let report = vm.execute(&module)?;
    for output in &report.outputs {
        println!("{output}");
    }
    if print_audit {
        for event in &report.audit {
            println!(
                "AUDIT seq={} ip={} op={} outcome={:?}",
                event.sequence, event.instruction_pointer, event.instruction, event.outcome
            );
        }
    }
    Ok(())
}

fn required<'a>(arguments: &'a [String], index: usize, usage: &str) -> ZlangResult<&'a str> {
    arguments
        .get(index)
        .map(String::as_str)
        .ok_or_else(|| ZlangError::Usage(usage.to_string()))
}

fn print_help() {
    println!("ZLang register VM v2026.2.0");
    println!("\nUsage:");
    println!("  zlang run <source.zl> [--audit]");
    println!("  zlang compile <source.zl> <module.zreg>");
    println!("  zlang exec <module.zreg> [--audit]");
    println!("  zlang inspect <module.zreg>");
    println!("\nStable core: integer expressions, let bindings, print, verified ZREG bytecode, deterministic register VM.");
}
