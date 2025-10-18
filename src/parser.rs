use std::{collections::HashMap, panic};

use crate::scanner::{Token, TokenType};
macro_rules! INSERT {
    ($s:expr,$name:ident) => {
        let reg = $s.consume_2(&[R0, R1, R2, R3, R4, R5, R6, R7]);
        $s.consume(Comma);
        let register_or_imm = $s.consume_2(&[R0, R1, R2, R3, R4, R5, R6, R7, INT]);
        $s.statements.push(Stmt::$name {
            lhs_reg: reg,
            right_reg_imm: register_or_imm,
        });
    };
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone)]
pub enum Stmt {
    MovLit {
        from: Token,
        register_or_imm: Token,
    },
    Halt {
        token: Token,
    },
    CMP {
        from_reg: Token,
        register_or_imm: Token,
    },
    JMPZ {
        to: Token,
    },
    JMPLE {
        to: Token,
    },
    JMPGE {
        to: Token,
    },
    JMPG {
        to: Token,
    },
    JMPL {
        to: Token,
    },
    JMP {
        to: Token,
    },
    Call {
        to: Token,
    },
    RET,
    ADD {
        lhs_reg: Token,
        right_reg_imm: Token,
    },
    MOD {
        lhs_reg: Token,
        right_reg_imm: Token,
    },

    SUB {
        lhs_reg: Token,
        right_reg_imm: Token,
    },
    DIV {
        lhs_reg: Token,
        right_reg_imm: Token,
    },
    MUL {
        lhs_reg: Token,
        right_reg_imm: Token,
    },

    NOP,
    Print {
        reg: Token,
    },
    PUSH {
        register_or_imm: Token,
    },
    POP {
        reg: Token,
    },
    AND_OR_XOR {
        type_op: TokenType,
        reg: Token,
        register_or_imm: Token,
    },
}

pub struct Parser {
    tokens: Vec<Token>,
    statements: Vec<Stmt>,
    current: usize,
    mapping_table: HashMap<String, usize>,
}
impl Parser {
    pub fn get_table(&self) -> &HashMap<String, usize> {
        &self.mapping_table
    }
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            statements: vec![],
            current: 0,
            mapping_table: HashMap::new(),
        }
    }

    pub fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    pub fn advance(&mut self) -> Token {
        let token = self.tokens[self.current].clone();
        self.current += 1;
        token
    }

    #[inline]
    fn is_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    pub fn parse(&mut self) -> &[Stmt] {
        while !self.is_end() {
            self.statements();
        }
        &self.statements
    }
    pub fn match_(&mut self, token: &[TokenType]) -> bool {
        for i in token {
            if self.peek().token_type == *i {
                self.advance();
                return true;
            }
        }
        false
    }
    pub fn statements(&mut self) {
        if self.match_(&[TokenType::MOV]) {
            self.mov_statement();
        } else if self.match_(&[TokenType::HALT]) {
            self.halt();
        } else if self.match_(&[TokenType::CMP]) {
            self.compare_stmt();
        } else if self.match_(&[TokenType::JMPG]) {
            self.jump_stmt();
        } else if self.match_(&[TokenType::ADD]) {
            use TokenType::*;
            INSERT!(self, ADD);
        } else if self.match_(&[TokenType::SUB]) {
            use TokenType::*;
            INSERT!(self, SUB);
        } else if self.match_(&[TokenType::MUL]) {
            use TokenType::*;
            INSERT!(self, MUL);
        } else if self.match_(&[TokenType::DIV]) {
            use TokenType::*;
            INSERT!(self, DIV);
        } else if self.match_(&[TokenType::MOD]) {
            use TokenType::*;
            INSERT!(self, MOD);
        }
        else if self.match_(&[TokenType::JUMP]) {
              let token = self.consume_2(&[TokenType::LabelCall]);
            self.statements.push(Stmt::JMP{ to: token });

        }
        else if self.match_(&[TokenType::Print]) {
            self.print_st();
        } else if self.match_(&[TokenType::LabelDef]) {
            self.label_def();
        } else if self.match_(&[TokenType::Call]) {
            self.call();
        } else if self.match_(&[TokenType::Ret]) {
            self.statements.push(Stmt::RET);
        } else if self.match_(&[TokenType::JMPL]) {
            self.jump_stmt_2();
        } else if self.match_(&[TokenType::PUSH]) {
            self.push();
        } else if self.match_(&[TokenType::POP]) {
            self.pop();
        } else if self.match_(&[TokenType::AND, TokenType::OR, TokenType::XOR]) {
            self.bit_wise();
        }else if self.match_(&[TokenType::JMPZ]) {
            let token = self.consume_2(&[TokenType::LabelCall]);
            self.statements.push(Stmt::JMPZ { to: token });
        }else if self.match_(&[TokenType::JMPLE]) {
            let token = self.consume_2(&[TokenType::LabelCall]);
            self.statements.push(Stmt::JMPLE { to: token });
        }else if self.match_(&[TokenType::JMPGE]) {
            let token = self.consume_2(&[TokenType::LabelCall]);
            self.statements.push(Stmt::JMPGE { to: token });
        }
        else {
            panic!("uknown token {:?}", self.peek());
        }
    }
    fn bit_wise(&mut self) {
        use TokenType::*;
        let type_op = self.previous().token_type;
        let register = self.consume_2(&[R0, R1, R2, R3, R4, R5, R6, R7]);
        self.consume(Comma);
        let register_or_imm = self.consume_2(&[R0, R1, R2, R3, R4, R5, R6, R7, INT]);
        self.statements.push(Stmt::AND_OR_XOR {
            type_op,
            reg: register,
            register_or_imm,
        });
    }

    pub fn pop(&mut self) {
        use TokenType::*;
        let register_or_imm = self.consume_2(&[R0, R1, R2, R3, R4, R5, R6, R7]);
        self.statements.push(Stmt::POP {
            reg: register_or_imm,
        });
    }
    pub fn push(&mut self) {
        use TokenType::*;
        let register_or_imm = self.consume_2(&[R0, R1, R2, R3, R4, R5, R6, R7, INT]);
        self.statements.push(Stmt::PUSH { register_or_imm });
    }

    pub fn call(&mut self) {
        let int_token = self.consume_2(&[TokenType::LabelCall]);
        self.statements.push(Stmt::Call { to: int_token });
    }

    pub fn label_def(&mut self) {
        let token = self.previous();
        self.statements.push(Stmt::NOP);
        let index = self.statements.len() - 1;
        self.mapping_table.insert(token.literal.unwrap(), index);
    }

    pub fn print_st(&mut self) {
        use TokenType::*;
        let reg = self.consume_2(&[R0, R1, R2, R3, R4, R5, R6, R7]);
        self.statements.push(Stmt::Print { reg });
    }
    pub fn jump_stmt(&mut self) {
        let token = self.consume_2(&[TokenType::LabelCall]);
        self.statements.push(Stmt::JMPG { to: token });
    }
    pub fn jump_stmt_2(&mut self) {
        let token = self.consume_2(&[TokenType::LabelCall]);
        self.statements.push(Stmt::JMPL { to: token });
    }

    pub fn compare_stmt(&mut self) {
        use TokenType::*;
        let reg = self.consume_2(&[R0, R1, R2, R3, R4, R5, R6, R7]);
        self.consume(Comma);
        let register_or_imm = self.consume_2(&[R0, R1, R2, R3, R4, R5, R6, R7, INT]);
        self.statements.push(Stmt::CMP {
            from_reg: reg,
            register_or_imm,
        });
    }

    fn consume(&mut self, expected: TokenType) {
        let prev = self.previous();
        if self.peek().token_type == expected {
            self.current += 1;
        } else {
            panic!(
                "expeted {:?} at line {} after {:?}",
                expected,
                self.peek().line_number - 1,
                prev.token_type
            );
        }
    }

    fn consume_2(&mut self, expected: &[TokenType]) -> Token {
        let tk = self.peek().clone();
        for i in expected {
            if tk.token_type == *i {
                self.current += 1;
                return tk;
            }
        }
        panic!("Expected {:?}, found {:?}", expected, tk);
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn halt(&mut self) {
        let token = self.previous();
        self.statements.push(Stmt::Halt { token });
    }
    fn mov_statement(&mut self) {
        use TokenType::*;
        let register = self.consume_2(&[R0, R1, R2, R3, R4, R5, R6, R7]);
        self.consume(Comma);
        let register_or_imm = self.consume_2(&[R0, R1, R2, R3, R4, R5, R6, R7, INT]);
        self.statements.push(Stmt::MovLit {
            from: register,
            register_or_imm,
        });
    }
}
