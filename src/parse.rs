use std::collections::HashMap;

use crate::value::Value;
use crate::bytecode::ByteCode;
use crate::lex::{Lex, Token};

pub struct ParseProto {
    pub constants: Vec<Value>,
    pub instructions: Vec<ByteCode>,
    constants_pos: HashMap<Value, usize>,  // 键字符串（字符串型Value），值常量表位置

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
                    let next_token = self.lex.peek();
                    if let Token::Assign = next_token {
                        self.assignment(name);
                    } else {
                        self.function_call(name);
                    }
                },

                // local Name = exp
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

    fn add_const(&mut self, const_var: Value) -> usize {  // Value添加到constants_pos中
        if let Some(i) = self.constants_pos.get(&const_var) {
            return *i;
        } else {
            let i = self.constants.len();
            self.constants_pos.insert(const_var.clone(), i);
            self.constants.push(const_var);
            i
        }
    }

    // local a = ...
    fn load_exp(&mut self, dst: usize) {
        let token = self.lex.next();

        let byte_code = match token {
            Token::String(s) => {
                let i = self.add_const(Value::String(s));
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

    // 加载变量： local a = b
    fn load_var(&mut self, name: String, dst: usize) -> ByteCode {
        if let Some(i) = self.locals.iter().rposition(|x| *x==name) {
            ByteCode::Move(dst as u8, i as u8)
        } else {
            let i = self.add_const(Value::String(name));
            ByteCode::GetGlobal(dst as u8, i as u8)
        }
        

    }

    fn function_call(&mut self, name: String) {
        let constant_idx = self.add_const(Value::String(name));
        let func_idx = self.locals.len();
        self.instructions.push(ByteCode::GetGlobal(func_idx as u8, constant_idx as u8));

        let next_token = self.lex.next();
        match next_token {
            Token::String(s) => {
                let i = self.add_const(Value::String(s));
                self.instructions.push(ByteCode::LoadConstant((func_idx+1) as u8, i as u8));
            },
            Token::Lp => {  // '('
                self.load_exp(func_idx+1);

                if let Token::Rp = self.lex.next() {

                } else {
                    panic!("expect ')'");
                }
            },

            _ => panic!("expect constant")
        }

        self.instructions.push(ByteCode::Call(func_idx as u8, 1));
    }

    fn assignment(&mut self, var_name: String) {
        self.lex.next();  // '='

        if let Some(i) = self.locals.iter().rposition(|x| *x == var_name) {
            // 局部变量赋值
            self.load_exp(i);
        } else {
            // 添加全局变量
            let dst = self.add_const(Value::String(var_name)) as u8;

            let next_token = self.lex.next();
            let code = match next_token {
                // 常数
                Token::String(s) => ByteCode::SetGlobalConst(dst, self.add_const(Value::String(s)) as u8),
                Token::Integer(i) => ByteCode::SetGlobalConst(dst, self.add_const(Value::Integer(i)) as u8),
                Token::Float(f) => ByteCode::SetGlobalConst(dst, self.add_const(Value::Float(f)) as u8),
                Token::True => ByteCode::SetGlobalConst(dst, self.add_const(Value::Bool(true)) as u8),
                Token::False => ByteCode::SetGlobalConst(dst, self.add_const(Value::Bool(false)) as u8),
                Token::Nil => ByteCode::SetGlobalConst(dst, self.add_const(Value::Nil) as u8),

                // 变量
                Token::Name(name) => {
                    if let Some(i) = self.locals.iter().rposition(|x| *x == name) {
                        ByteCode::SetGlobal(dst, i as u8)
                    } else {
                        ByteCode::SetGlobalGlobal(dst, self.add_const(Value::String(name)) as u8)
                    }
                },

                _ => panic!("Unexpected token: {:?}", next_token),
            };

            self.instructions.push(code);
        }

    }
}

