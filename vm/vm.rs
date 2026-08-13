use crate::vm::bytecode::OpCode;
use crate::vm::value::Value;
use std::collections::HashMap;
use std::process::Command;

pub struct VM {
    pub stack: Vec<Value>, pub ip: usize, pub code: Vec<u8>, 
    pub constants: Vec<Value>, pub globals: HashMap<String, Value>
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
                        let val = self.stack.pop().expect("Stack vuoto");
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
                OpCode::Sub => { let b = self.stack.pop().unwrap(); let a = self.stack.pop().unwrap(); if let (Value::Int(i1), Value::Int(i2)) = (a, b) { self.stack.push(Value::Int(i1 - i2)); } }
                OpCode::Less => { let b = self.stack.pop().unwrap(); let a = self.stack.pop().unwrap(); if let (Value::Int(i1), Value::Int(i2)) = (a, b) { self.stack.push(Value::Int(if i1 < i2 { 1 } else { 0 })); } }
                OpCode::Greater => { let b = self.stack.pop().unwrap(); let a = self.stack.pop().unwrap(); if let (Value::Int(i1), Value::Int(i2)) = (a, b) { self.stack.push(Value::Int(if i1 > i2 { 1 } else { 0 })); } }
                OpCode::Eq => { let b = self.stack.pop().unwrap(); let a = self.stack.pop().unwrap(); if let (Value::Int(i1), Value::Int(i2)) = (a, b) { self.stack.push(Value::Int(if i1 == i2 { 1 } else { 0 })); } }
                OpCode::JmpIfFalse => {
                    let addr = self.code[self.ip] as usize; self.ip += 1;
                    if let Some(Value::Int(cond)) = self.stack.pop() { if cond == 0 { self.ip = addr; } }
                }
                OpCode::Jmp => { self.ip = self.code[self.ip] as usize; }
                OpCode::SysCall => {
                    let id = self.code[self.ip]; self.ip += 1; let _argc = self.code[self.ip]; self.ip += 1;
                    match id {
                        1 => { if let Some(val) = self.stack.pop() { println!("\x1B[1;36m[Z-LANG LOG]\x1B[0m {:?}", val); } }
                        2 => {
                            if let Some(Value::Str(s)) = self.stack.pop() {
                                let h = format!("{:x}", md5::compute(s));
                                self.stack.push(Value::Str(h)); // Il fix è qui!
                            }
                        }
                        3 => { if let Some(val) = self.stack.pop() { println!("\x1B[1;35m[Z-CHAIN P2P]\x1B[0m Blocco Neurale Trasnesso: {:?}", val); } }
                        10 => {
                            if let Some(Value::Str(cmd)) = self.stack.pop() {
                                println!("\x1B[1;31m[ZDOS EXEC]\x1B[0m Esecuzione comando: {}", cmd);
                                let output = Command::new("sh").arg("-c").arg(&cmd).output();
                                if let Ok(out) = output { self.stack.push(Value::Str(String::from_utf8_lossy(&out.stdout).trim().to_string())); } 
                                else { self.stack.push(Value::Str("Err".to_string())); }
                            }
                        }
                        _ => {}
                    }
                }
                OpCode::Ret => break,
                _ => { self.ip += 1; }
            }
        }
        println!("\x1B[1;32m[ZDOS VM] Ciclo terminato.\x1B[0m");
    }
}
