pub struct Compiler {
    pub instructions: Vec<String>,
}

impl Default for Compiler {
    fn default() -> Self { Self::new() }
}

impl Compiler {
    pub fn new() -> Self { Compiler { instructions: Vec::new() } }

    pub fn compile(&mut self, source: &str) -> Vec<String> {
        self.instructions.clear();
        for line in source.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') { continue; }
            if let Some(stripped) = line.strip_prefix("emit ") {
                self.instructions.push(format!("PRINT_STDOUT {}", stripped));
            } else if let Some(stripped) = line.strip_prefix("let ") {
                self.instructions.push(format!("LET_ASSIGN {}", stripped));
            } else {
                self.instructions.push(format!("UNKNOWN_OPCODE {}", line));
            }
        }
        self.instructions.clone()
    }
}
