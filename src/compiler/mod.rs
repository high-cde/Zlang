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
            } else if line.strip_prefix("zchain ").is_some()
                || line == "SCAN_LEO_CONSTELLATION"
                || line == "orbit_sync"
            {
                self.instructions
                    .push(format!("UNIMPLEMENTED_EXTERNAL_OPERATION {}", line));
            } else {
                self.instructions.push(format!("UNKNOWN_OPCODE {}", line));
            }
        }
        self.instructions.clone()
    }
}
