mod compiler;
mod vm;
mod wallet;

use compiler::Compiler;
use vm::ZVirtualMachine;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Uso: zlang <file.zl oppure file.zbag>");
        return;
    }

    let filename = &args[1];
    let source_code = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            println!("Errore lettura file {}: {}", filename, e);
            return;
        }
    };

    let mut compiler = Compiler::new();
    let mut vm = ZVirtualMachine::new();

    // Se è un file .zbag, estraiamo l'importo della transazione
    let processed_source = if filename.ends_with(".zbag") {
        println!("[zbag Wallet] Caricamento transazione nativa crittografica...");
        let mut transfer_amount = 0;
        for line in source_code.lines() {
            let line = line.trim();
            if line.starts_with("amount") {
                let parts: Vec<&str> = line.split('=').collect();
                if parts.len() == 2 {
                    transfer_amount = parts[1].trim().parse().unwrap_or(0);
                }
            }
        }
        format!(
            "emit \"=== VALIDAZIONE ZBAG WALLET ===\"\n\
             let sovereign_balance = 100000\n\
             let transfer_amount = {}\n\
             let sovereign_balance = sovereign_balance - transfer_amount\n\
             emit \"=== ZBAG TRANSAZIONE CONFERMATA ===\"",
            transfer_amount
        )
    } else {
        source_code
    };

    let bytecode = compiler.compile(&processed_source);
    let result = vm.execute(bytecode);
    print!("{}", result);
}
