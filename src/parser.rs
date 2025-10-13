use std::panic;

use crate::scanner::{Token, TokenType};

#[derive(Debug,Clone)]
pub enum Stmt{
    MovLit{
        from:Token,
        register_or_imm:Token,
    },
    Halt{
        token:Token
    }
}

pub struct Parser{
    tokens:Vec<Token>,
    statements:Vec<Stmt>,
    current:usize
}
impl Parser{
    pub fn new(tokens:Vec<Token>)->Self{
        Self { tokens , statements: vec![],current:0 }
    }

    pub fn peek(&self)->&Token{
        &self.tokens[self.current]
    }
   
    pub fn advance(&mut self)->Token{
        let token = self.tokens[self.current].clone();
        self.current +=1;
        token
    }
    
    #[inline]
    fn is_end(&self)->bool{
        self.peek().token_type == TokenType::EOF
    }

    pub fn parse(&mut self)->&[Stmt]{
        while !self.is_end(){
            self.statements();
        }
        &self.statements
    }
    pub fn match_(&mut self,token:&[TokenType])->bool{
        for i in token{
            if self.peek().token_type == *i{
                self.advance();
                return true;
            }
        }
        false
    }
    pub fn statements(&mut self){
        if self.match_( &[TokenType::MOV]){
            self.mov_statement();
        }else if self.match_(&[TokenType::HALT]) {
            self.halt();
        }
    }
    
    fn consume(&mut self,expected:TokenType){
        if self.peek().token_type == expected{
            self.current +=1;
        }else {
            panic!("expeted {:?} ",expected);
        }
    }

    
    fn consume_2(&mut self,expected:&[TokenType])->Token{
        let tk =self.peek().clone();
        for i in expected{
           if  tk.token_type == *i{
               self.current +=1;
               return tk;
            }
        }
        panic!("Expected {:?}, found {:?}",expected,tk);
    }

    fn previous(&self)->Token{
        self.tokens[self.current -1].clone()
    }

    fn halt(&mut self){
        let token  = self.previous();
        self.statements.push(Stmt::Halt { token });
    }
    fn mov_statement(&mut self){
       use TokenType::*;
       let register = self.consume_2(&[R0,R1,R2,R3,R4,R5,R6,R7]); 
       self.consume(Comma);
       let register_or_imm = self.consume_2(&[R0,R1,R2,R3,R4,R5,R6,R7,INT]);
       self.statements.push(Stmt::MovLit { from: register, register_or_imm });
    } 
}
