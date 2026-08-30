use std::env;
use std::fs;
use std::process;
use zdos_zlang::{Compiler, ZVirtualMachine};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Uso: zlang <file.zlang>");
        process::exit(2);
    }

    let filename = &args[1];
    let source = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("[Zlang] errore di lettura per {filename:?}: {error}");
            process::exit(1);
        }
    };

    let compiler = Compiler::new();
    let bytecode = match compiler.compile(&source) {
        Ok(bytecode) => bytecode,
        Err(error) => {
            eprintln!("[Zlang] errore di compilazione: {error}");
            process::exit(1);
        }
    };

    let output = match ZVirtualMachine::new().execute(&bytecode) {
        Ok(output) => output,
        Err(error) => {
            eprintln!("[Zlang] errore della VM: {error}");
            process::exit(1);
        }
    };
    print!("{output}");
}
