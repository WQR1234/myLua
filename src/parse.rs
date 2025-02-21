use std::collections::HashMap;

use crate::value::Value;
use crate::bytecode::ByteCode;
use crate::lex::{Lex, Token};

pub struct ParseProto {
    pub constants: Vec<Value>,
    pub instructions: Vec<ByteCode>,
    constants_pos: HashMap<String, usize>,  // 键字符串（字符串型Value），值常量表位置

    locals: Vec<String>,
    lex: Lex
}

impl ParseProto {
    pub fn new(lex: Lex) -> Self {
        ParseProto {
            constants: Vec::new(),
            instructions: Vec::new(),
            constants_pos: HashMap::new(),
            locals: Vec::new(),
            lex
        }
    } 

    pub fn compile(&mut self) {
        loop {
            let token = self.lex.next();
            match token {
                Token::Eos => break,
                Token::Name(name) => {
 
                    let constant_idx = self.add_const_string(name);
                    let func_idx = self.locals.len();
                    self.instructions.push(ByteCode::GetGlobal(func_idx as u8, constant_idx as u8));
    
                    let next_token = self.lex.next();
                    match next_token {
                        Token::String(s) => {
                            let i = self.add_const_string(s);
                            self.instructions.push(ByteCode::LoadConstant((func_idx+1) as u8, i as u8));
                        },
                        Token::Lp => {  // '('
                            self.load_exp(func_idx+1);
    
                            if let Token::Rp = self.lex.next() {
                                
                            } else {
                                panic!("expect ')'");
                            }
                        },
                        Token::Assign => {
                            // self.assignment(name);
                        }
    
                        _ => panic!("expect constant")
                    } 
    
                    self.instructions.push(ByteCode::Call(func_idx as u8, 1));
                },
                Token::Local => {
                    if let Token::Name(name) = self.lex.next() {
                        if let Token::Assign = self.lex.next() {
                            self.load_exp(self.locals.len());
                            self.locals.push(name);
                        } else {
                            panic!("expected `=`");
                        }
                    } else {
                        panic!("Expect variable name")
                    }
                }
    
                _ => panic!("Unexpected token: {:?}", token),
            }
        }

        dbg!(&self.constants);
        dbg!(&self.instructions);
    }

    fn add_const_string(&mut self, str: String) -> usize {  // 字符型Value添加到constants_pos中
        if let Some(i) = self.constants_pos.get(&str) {
            return *i;
        } else {
            let i = self.constants.len();
            self.constants.push(Value::String(str.clone()));
            self.constants_pos.insert(str, i);
            i
        }
    }

    fn load_exp(&mut self, dst: usize) {
        let token = self.lex.next();

        let byte_code = match token {
            Token::String(s) => {
                let i = self.add_const_string(s);
                ByteCode::LoadConstant(dst as u8, i as u8)
            },
            Token::Integer(i) => {
                if let Ok(ii) = i16::try_from(i) {
                    ByteCode::LoadInt(dst as u8, ii)
                } else {
                    let constant_index = self.constants.len();
                    self.constants.push(Value::Integer(i));
                    ByteCode::LoadConstant(dst as u8, constant_index as u8)
                }
            },
            Token::Float(f) => {
                let constant_index = self.constants.len();
                self.constants.push(Value::Float(f));
                ByteCode::LoadConstant(dst as u8, constant_index as u8)
            },
            Token::True => {
                ByteCode::LoadBool(dst as u8, true)
            },
            Token::False => {
                ByteCode::LoadBool(dst as u8, false)
            },
            Token::Nil => {
                ByteCode::LoadNil(dst as u8)
            },
            Token::Name(name) => self.load_var(name, dst),
            _ => panic!("Unexpected token: {:?}", token),
            
        };

        self.instructions.push(byte_code);
    }

    fn load_var(&mut self, name: String, dst: usize) -> ByteCode {
        if let Some(i) = self.locals.iter().rposition(|x| *x==name) {
            ByteCode::Move(dst as u8, i as u8)
        } else {
            let i = self.add_const_string(name);
            ByteCode::GetGlobal(dst as u8, i as u8)
        }
        

    }

    fn assignment(&mut self, var_name: String) {

    }
}

