use crate::zpm;
use std::collections::HashMap;

pub struct ZVirtualMachine {
    pub memory: HashMap<String, String>,
}

impl ZVirtualMachine {
    pub fn new() -> Self {
        ZVirtualMachine {
            memory: HashMap::new(),
        }
    }

    pub fn execute(&mut self, bytecode: Vec<String>) {
        println!("\n[Z-LANG VM 2026] Esecuzione Bytecode...");
        for instruction in bytecode {
            if instruction.starts_with("PRINT_STDOUT") {
                println!("[Z-LANG OUTPUT] {}", &instruction[13..]);
            } else if instruction == "SPACEX_LEO_HANDSHAKE" {
                println!("[Z-LANG ORBITAL] Handshake LEO completato.");
            } else if instruction.starts_with("ZCHAIN_SIGN") {
                zpm::sign_transaction_zchain(&instruction[12..]);
            }
        }
        println!("[Z-LANG VM 2026] Esecuzione completata.");
    }
}
