#[derive(Debug, Clone)]
pub enum AstNode {
    Let { name: String, value: String },
    Print(String),
    SpaceCommand(String),
}

pub struct Compiler { pub instructions: Vec<String> }

impl Compiler {
    pub fn new() -> Self { Compiler { instructions: Vec::new() } }
    pub fn compile(&mut self, source: &str) -> Vec<String> {
        for line in source.lines() {
            let line = line.trim();
            if line.starts_with("emit ") { 
                self.instructions.push(format!("PRINT_STDOUT {}", &line[5..])); 
            } else if line == "orbit_sync" { 
                self.instructions.push(String::from("SPACEX_LEO_HANDSHAKE")); 
            } else if line.starts_with("zchain ") { 
                self.instructions.push(format!("ZCHAIN_SIGN {}", &line[7..])); 
            }
        }
        self.instructions.clone()
    }
}
