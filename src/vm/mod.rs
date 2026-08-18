use std::collections::HashMap;

pub struct ZVirtualMachine {
    pub memory: HashMap<String, String>,
}

impl Default for ZVirtualMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl ZVirtualMachine {
    pub fn new() -> Self {
        ZVirtualMachine {
            memory: HashMap::new(),
        }
    }

    pub fn execute(&mut self, bytecode: Vec<String>) -> String {
        let mut output = String::new();
        for instr in bytecode {
            if let Some(stripped) = instr.strip_prefix("PRINT_STDOUT ") {
                output.push_str(&format!("[ZLang VM] {}\n", stripped));
            } else if let Some(stripped) = instr.strip_prefix("ZCHAIN_SIGN ") {
                output.push_str(&format!("[Z-Chain Ledger] Firma payload: {}\n", stripped));
            } else if instr == "SYSTEM_SCAN_ORBITAL" {
                output.push_str("[LEO Tracker] Scansione costellazione attiva...\n");
            } else if instr == "NET_SYNC_LEO" {
                output.push_str("[Network] Gateway sincronizzato.\n");
            } else {
                output.push_str(&format!("[Errore VM] Istruzione non valida: {}\n", instr));
            }
        }
        output
    }
}
