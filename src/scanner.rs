use std::{collections::HashMap, iter::Peekable, str::Chars};


#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    MOV,
    Print,
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    MUL,
    DIV,
    ADD,
    SUB,
    CMP,
    JMPG,
    JMPL,
    JMPGE,
    JMPLE,
    JMPZ,
    JMPNZ,
    JUMP,
    INT,
    Comma,
    HALT,
    EOF,
    LabelDef,
    LabelCall,
    Call,
    Ret
}
impl TokenType {
    pub fn get_reg(&self) -> (u32, TokenType) {
        match self {
            TokenType::R0 => (0, TokenType::R0),
            TokenType::R1 => (1, TokenType::R1),
            TokenType::R2 => (2, TokenType::R2),
            TokenType::R3 => (3, TokenType::R3),
            TokenType::R4 => (4, TokenType::R4),
            TokenType::R5 => (5, TokenType::R5),
            TokenType::R6 => (6, TokenType::R6),
            TokenType::R7 => (7, TokenType::R7),
            _ => panic!("Not a register"),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: Option<String>,
    pub line_number: usize,
}
pub struct Scanner<'a> {
    data: Peekable<Chars<'a>>,
    tokens: Vec<Token>,
    line: usize,
    keywords: HashMap<String, TokenType>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut map = HashMap::new();
        map.insert("mov".to_string(), TokenType::MOV);
        map.insert("r0".to_string(), TokenType::R0);
        map.insert("r1".to_string(), TokenType::R1);
        map.insert("r2".to_string(), TokenType::R2);
        map.insert("r3".to_string(), TokenType::R3);
        map.insert("r4".to_string(), TokenType::R4);
        map.insert("r5".to_string(), TokenType::R5);
        map.insert("r6".to_string(), TokenType::R6);
        map.insert("r7".to_string(), TokenType::R7);
        map.insert("halt".to_string(), TokenType::HALT);
        map.insert("cmp".to_string(), TokenType::CMP);
        map.insert("jmpg".to_string(), TokenType::JMPG);
        map.insert("jmpl".to_string(), TokenType::JMPL);
        map.insert("jmp".to_string(), TokenType::JUMP);
        map.insert("add".to_string(), TokenType::ADD);
        map.insert("print".to_string(), TokenType::Print);
        map.insert("sub".to_string(), TokenType::SUB);
        map.insert("call".to_string(), TokenType::Call);
        map.insert("ret".to_string(), TokenType::Ret);


        Self {
            data: source.chars().peekable(),
            tokens: vec![],
            line: 1,
            keywords: map,
        }
    }
    pub fn push_token(&mut self, literal: Option<String>, token_type: TokenType) {
        let token = Token {
            token_type,
            literal,
            line_number: self.line,
        };
        self.tokens.push(token);
    }
    pub fn parse(&'a mut self) -> &'a [Token] {
        while let Some(a) = self.data.next() {
            match a {
                '\n' => {
                    self.line += 1;
                }
                a if a.is_ascii_whitespace() || a == '\t'  => {}
                ',' => {
                    self.push_token(Some(a.to_string()), TokenType::Comma);
                }
                a if a.is_ascii_alphabetic() => {
                    let mut str = String::new();
                    str.push(a);
                    while let Some(a) = self.data.peek()
                        && (a.is_ascii_alphanumeric()|| *a == ':' || *a == '_')
                    {
                        str.push(self.data.next().unwrap());
                    }
                    str = str.trim().to_string();
                    if let Some(a) = self.keywords.get(&str.to_lowercase()) {
                        self.push_token(None, *a);
                    } else if str.contains(":"){
                        let str = str.replace(":", "").to_string();
                        self.push_token(Some(str), TokenType::LabelDef);
                    }else  {
                        self.push_token(Some(str), TokenType::LabelCall);
                    }
                }
                a if a.is_ascii_digit() || a == '-' => {
                    let mut dig = String::new();
                    dig.push(a);
                    while let Some(a) = self.data.peek()
                        && a.is_ascii_digit()
                    {
                        dig.push(self.data.next().unwrap());
                    }
                    self.push_token(Some(dig), TokenType::INT);
                }
                a => panic!("uknown token {a}"),
            }
        }
        self.push_token(None, TokenType::EOF);
        &self.tokens
    }
}
