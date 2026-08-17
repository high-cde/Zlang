use std::{env, fs};
use zdos_zlang::runtime;
use zdos_zlang::{Compiler, ZVirtualMachine};

fn main() {
    println!("=========================================");
    println!("        Z-LANG SOVEREIGN v2026.2.0       ");
    println!("=========================================");

    runtime::init_runtime();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("[Z-LANG] Utilizzo: zlang <file.zl>");
        return;
    }

    let source =
        fs::read_to_string(&args[1]).expect("[ERROR] Impossibile leggere il file sorgente.");
    let mut compiler = Compiler::new();
    let bytecode = compiler.compile(&source);

    let mut vm = ZVirtualMachine::new();
    vm.execute(bytecode);
}
