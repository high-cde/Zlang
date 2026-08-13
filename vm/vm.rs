use crate::vm::bytecode::OpCode;
use crate::vm::value::Value;
use std::collections::HashMap;
use std::process::Command; // <-- IL PONTE CON IL SISTEMA OPERATIVO

pub struct VM { 
    pub stack: Vec<Value>, 
    pub ip: usize, 
    pub code: Vec<u8>, 
    pub constants: Vec<Value>, 
    pub globals: HashMap<String, Value> 
}

impl VM {
    pub fn new(code: Vec<u8>, constants: Vec<Value>) -> Self { 
        VM { stack: Vec::new(), ip: 0, code, constants, globals: HashMap::new() } 
    }
    
    pub fn run(&mut self) {
        println!("\x1B[1;34m[ZDOS VM] Collegamento al Kernel Stabilito...\x1B[0m");
        while self.ip < self.code.len() {
            let opcode = OpCode::from(self.code[self.ip]);
            self.ip += 1;
            match opcode {
                OpCode::PushConst => { let idx = self.code[self.ip] as usize; self.ip += 1; self.stack.push(self.constants[idx].clone()); }
                OpCode::Pop => { self.stack.pop(); }
                OpCode::StoreGlobal => {
                    let idx = self.code[self.ip] as usize; self.ip += 1;
                    if let Value::Str(name) = &self.constants[idx] {
                        let val = self.stack.pop().unwrap();
                        self.globals.insert(name.clone(), val);
                    }
                }
                OpCode::LoadGlobal => {
                    let idx = self.code[self.ip] as usize; self.ip += 1;
                    if let Value::Str(name) = &self.constants[idx] {
                        let val = self.globals.get(name).cloned().unwrap_or(Value::Str("nil".into()));
                        self.stack.push(val);
                    }
                }
                OpCode::Add => {
                    let b = self.stack.pop().unwrap(); let a = self.stack.pop().unwrap();
                    match (a, b) {
                        (Value::Int(i1), Value::Int(i2)) => self.stack.push(Value::Int(i1 + i2)),
                        (Value::Str(s1), Value::Str(s2)) => self.stack.push(Value::Str(format!("{}{}", s1, s2))),
                        _ => {}
                    }
                }
                OpCode::SysCall => {
                    let id = self.code[self.ip]; self.ip += 1; let _argc = self.code[self.ip]; self.ip += 1;
                    match id {
                        // SysCall 1: Stampa a terminale
                        1 => { if let Some(val) = self.stack.pop() { println!("\x1B[1;36m[Z-LANG LOG]\x1B[0m {:?}", val); } }
                        // SysCall 10: ESECUZIONE COMANDI DI SISTEMA (sys.exec)
                        10 => { 
                            if let Some(Value::Str(cmd)) = self.stack.pop() {
                                println!("\x1B[1;31m[ZDOS EXEC]\x1B[0m Esecuzione comando: {}", cmd);
                                let output = Command::new("sh").arg("-c").arg(&cmd).output();
                                match output {
                                    Ok(out) => {
                                        let res = String::from_utf8_lossy(&out.stdout).to_string();
                                        self.stack.push(Value::Str(res));
                                    },
                                    Err(_) => self.stack.push(Value::Str("Errore esecuzione".to_string())),
                                }
                            }
                        }
                        4 => { if let Some(Value::Int(ms)) = self.stack.pop() { std::thread::sleep(std::time::Duration::from_millis(ms as u64)); } }
                        _ => { if let Some(val) = self.stack.pop() { println!("\x1B[1;33m[ZDOS SYSCALL {}]\x1B[0m {:?}", id, val); } }
                    }
                }
                OpCode::Ret => break,
                _ => { self.ip += 1; }
            }
        }
        println!("\x1B[1;32m[ZDOS VM] Ciclo terminato.\x1B[0m");
    }
}
