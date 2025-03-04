use std::collections::HashMap;

use crate::value::Value;
use crate::bytecode::ByteCode;
use crate::parse::ParseProto;

pub struct ExeState {
    stack: Vec<Value>,
    globals: HashMap<String, Value>,
}

impl ExeState {
    pub fn new() -> Self {
        let mut globals = HashMap::new();

        globals.insert("print".to_string(), Value::Function(|state| {
            let value = state.stack.pop().unwrap();
            println!("{}", value);
            0
        }));
        

        ExeState {
            stack: Vec::new(),
            globals,
        }
    }

    pub fn run(&mut self, proto: &ParseProto) {
        for instruction in &proto.instructions {
            match *instruction {
                ByteCode::GetGlobal(index, name) => {
                    let name = &proto.constants[name as usize];
                    if let Value::String(name) = name {
                        let value = self.globals.get(name)
                                        .unwrap_or(&Value::Nil)
                                        .clone();
                        self.set_stack(index, value);
                        
                    } else {
                        panic!("Expected string, got {:?}", name);
                    }
                },
                ByteCode::LoadConstant(index, c) => {
                    let value = proto.constants[c as usize].clone();
                    self.set_stack(index, value);
                }
                ByteCode::LoadInt(index, i) => {
                    self.set_stack(index, Value::Integer(i as i64));
                }
                
                ByteCode::LoadBool(index, b) => {
                    self.set_stack(index, Value::Bool(b));
                }
                ByteCode::LoadNil(index) => {
                    self.set_stack(index, Value::Nil);
                }

                ByteCode::SetGlobal(dst, src) => {
                    let name = proto.constants[dst as usize].clone();
                    if let Value::String(key) = name {
                        let value = self.stack[src as usize].clone();
                        self.globals.insert(key, value);
                    } else {
                        panic!("Expected string, got {:?}", name);
                    }
                }

                ByteCode::SetGlobalConst(dst, src) => {
                    let name = proto.constants[dst as usize].clone();
                    if let Value::String(name) = name {
                        let value = proto.constants[src as usize].clone();
                        self.globals.insert(name, value);
                    } else {
                        panic!("Expected string, got {:?}", name);
                    }
                }

                ByteCode::SetGlobalGlobal(dst, src) => {
                    let name = proto.constants[dst as usize].clone();
                    if let Value::String(name) = name {
                        let src = &proto.constants[src as usize];
                        if let Value::String(src) = src {
                            let value = self.globals.get(src)
                                .unwrap_or(&Value::Nil)
                                .clone();
                            self.globals.insert(name, value);
                        } else {
                            panic!("Expected string, got {:?}", src);
                        }

                    } else {
                        panic!("Expected string, got {:?}", name);
                    }
                }

                ByteCode::Move(dst, src) => {
                    self.set_stack(dst, self.stack[src as usize].clone());
                }

                ByteCode::Call(index, _) => {
                    // println!("{}", index);
                    // dbg!(&self.stack);
                    let function = &self.stack[index as usize];
                    if let Value::Function(f) = function {
                        let result = f(self);
                        if result != 0 {
                            panic!("Function returned error code: {}", result);
                        }
                    } else {
                        panic!("Expected function, got {:?}", function);
                    }
                }

                _ => panic!("Unimplemented instruction: {:?}", instruction),
            }
        }
    }

    fn set_stack(&mut self, index: u8, value: Value) {
        let index = index as usize;
        match index.cmp(&self.stack.len()) {
            std::cmp::Ordering::Equal => self.stack.push(value),
            std::cmp::Ordering::Less => self.stack[index] = value,
            std::cmp::Ordering::Greater => {
                dbg!(&self.stack);
                panic!("Invalid stack index: {}", index)
            },
            
        }
    }
}