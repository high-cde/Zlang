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
            } else if instr.starts_with("UNIMPLEMENTED_EXTERNAL_OPERATION ") {
                output.push_str(&format!("[Errore VM] Operazione esterna non implementata: {}\n", instr));
            } else {
                output.push_str(&format!("[Errore VM] Istruzione non valida: {}\n", instr));
            }
        }
        output
    }
}
