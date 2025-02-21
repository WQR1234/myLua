use std::{fs, io::{Read, Seek, SeekFrom}};

#[derive(Debug)]
pub enum Token {

    // lua关键字
    And, Break, Do, Else, ElseIf, End, 
    False, For, Function, Goto, If, In, 
    Local, Nil, Not, Or, Repeat, Return, 
    Then, True, Until, While,

    // lua符号
    // +  -    *    /    %    ^    #
    Add, Sub, Mul, Div, Mod, Pow, Len,
    // ==    ~=    <=    <    >=    >
    Eq,     Ne,    Le,   Lt,  Ge,  Gt,
    // =    (    )  {    }  [    ]
    Assign, Lp, Rp, Lb, Rb, Ls, Rs,
    //  ;        :      ,     .    ..      ...
    Semicolon, Colon, Comma, Dot, Concat, Vararg,

    // lua常量
    Integer(i64),
    Float(f64),
    String(String),

    // lua标识符
    Name(String),
    
    // 文件结束
    Eos,
}

pub struct Lex {
    // input: fs::File,
    code: Vec<u8>, 
    idx: usize,
    ahead: Option<Token>
}

impl Lex {
    pub fn new(filename: &str) -> Self {
        let mut input = fs::File::open(filename).expect("Could not open file");
        let mut code = Vec::new();
        input.read_to_end(&mut code).expect("Could not read file");

        Lex { code, idx: 0, ahead: None }
    }

    pub fn peek(&mut self) -> &Token {
        if self.ahead.is_none()  {
            let t = self.next();
            self.ahead = Some(t);
            
        } 

        self.ahead.as_ref().unwrap()
    }

    pub fn next(&mut self) -> Token {
        let ahead = self.ahead.take();
        if let Some(t) = ahead {
            return t;
        } 

        let c = self.read_char();

        match c {
            ' ' | '\n' | '\r' | '\t' => self.next(),
            '\0' => Token::Eos,
            '"' => {
                let mut s = String::new();
                loop {
                    let c = self.read_char();
                    match c {
                        '"' => break,
                        '\0' => panic!("Unterminated string"),
                        _ => s.push(c),
                        
                    }
                }
                Token::String(s)
            },
            'a'..='z' | 'A'..='Z' | '_' => {
                self.read_name(c)
            },

            '0'..='9' => {
                let mut s = String::new();
                s.push(c);
                let mut is_integer = true;
                loop {
                    let c = self.read_char();
                    match c {
                        '0'..='9' => s.push(c),
                        '.' => {
                            s.push(c);
                            is_integer = false;
                        },
                        '\0' => break,
                        _ => {
                            // self.input.seek(SeekFrom::Current(-1)).unwrap();
                            self.idx -= 1;
                            break;
                        }
                        
                    }
                }
                if is_integer {
                    Token::Integer(s.parse().unwrap())
                } else {
                    Token::Float(s.parse().unwrap())
                }
            },

            '(' => Token::Lp,
            ')' => Token::Rp,
            '{' => Token::Lb,
            '}' => Token::Rb,
            '[' => Token::Ls,
            ']' => Token::Rs,

            '+' => Token::Add,
            '-' => Token::Sub,
            '*' => Token::Mul,
            '/' => Token::Div,
            '%' => Token::Mod,
            '^' => Token::Pow,
            '#' => Token::Len,

            ';' => Token::Semicolon,
            ':' => Token::Colon,
            ',' => Token::Comma,
            
            '.' => {
                let c = self.read_char();
                if c == '.' {
                    let c = self.read_char();
                    if c == '.' {
                        Token::Vararg
                    } else {
                        // self.input.seek(SeekFrom::Current(-1)).unwrap();
                        self.idx -= 1;
                        Token::Concat
                    }
                } else {
                    // self.input.seek(SeekFrom::Current(-1)).unwrap();
                    self.idx -= 1;
                    Token::Dot
                }
            },
            '=' => {
                let c = self.read_char();
                if c == '=' {
                    Token::Eq
                } else {
                    self.idx -= 1;
                    Token::Assign

                }
            }

            _ => panic!("Unexpected character: {}", c),
        }
        
        
    }

    fn read_char(&mut self) -> char {
        // let mut buf = [0u8; 1];
        // match self.input.read(&mut buf) {
        //     Ok(1) => buf[0] as char,
        //     _ => '\0',
            
        // }

        if let Some(&ch) = self.code.get(self.idx) {
            self.idx += 1;
            return ch as char;
        } else {
            return '\0';
        }
    }

    fn read_name(&mut self, ch: char) -> Token {
        let mut s = String::new();
        s.push(ch);
        loop {
            let c = self.read_char();
            match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => s.push(c),
                '\0' => break,
                _ => {
                    // self.input.seek(SeekFrom::Current(-1)).unwrap();
                    self.idx -= 1;
                    break;
                }
                
            }
        }
        match &s as &str {     
            "and"      => Token::And,
            "break"    => Token::Break,
            "do"       => Token::Do,
            "else"     => Token::Else,
            "elseif"   => Token::ElseIf,
            "end"      => Token::End,
            "false"    => Token::False,
            "for"      => Token::For,
            "function" => Token::Function,
            "goto"     => Token::Goto,
            "if"       => Token::If,
            "in"       => Token::In,
            "local"    => Token::Local,
            "nil"      => Token::Nil,
            "not"      => Token::Not,
            "or"       => Token::Or,
            "repeat"   => Token::Repeat,
            "return"   => Token::Return,
            "then"     => Token::Then,
            "true"     => Token::True,
            "until"    => Token::Until,
            "while"    => Token::While,
            _ => Token::Name(s)
        }
    }
    
}