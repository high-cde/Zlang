pub struct Compiler {
    pub instructions: Vec<String>,
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            instructions: Vec::new(),
        }
    }

    pub fn compile(&mut self, source: &str) -> Vec<String> {
        for line in source.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if let Some(stripped) = line.strip_prefix("emit ") {
                self.instructions.push(format!("PRINT_STDOUT {}", stripped));
            } else if let Some(stripped) = line.strip_prefix("zchain ") {
                self.instructions.push(format!("ZCHAIN_SIGN {}", stripped));
            } else if line == "SCAN_LEO_CONSTELLATION" {
                self.instructions.push("SYSTEM_SCAN_ORBITAL".to_string());
            } else if line == "orbit_sync" {
                self.instructions.push("NET_SYNC_LEO".to_string());
            } else {
                self.instructions.push(format!("UNKNOWN_OPCODE {}", line));
            }
        }
        self.instructions.clone()
    }
}
