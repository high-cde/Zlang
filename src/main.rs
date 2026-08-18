use std::env;
use std::fs;
use std::process;
use zdos_zlang::{Compiler, ZVirtualMachine};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("[ZDOS] Errore: Nessun file sorgente fornito.");
        eprintln!("Uso: zlang <file.zlang>");
        process::exit(1);
    }

    let filename = &args[1];

    // Gestione sicura degli errori I/O senza panic!
    let source = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!(
                "[ZDOS] Errore di I/O nella lettura di '{}': {}",
                filename, e
            );
            process::exit(1);
        }
    };

    // 1. Fase di Compilazione
    let mut compiler = Compiler::new();
    let bytecode = compiler.compile(&source);

    // 2. Fase di Esecuzione (VM)
    let mut vm = ZVirtualMachine::new();
    let output = vm.execute(bytecode);

    print!("{}", output);
}
