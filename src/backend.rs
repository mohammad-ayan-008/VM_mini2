use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    parser::{Parser, Stmt},
    scanner::{Token, TokenType},
};
const I24_MIN: i32 = -8_388_608;
const I24_MAX: i32 = 8_388_607;

pub struct CodeGen {
    code: Vec<u8>,
    statements: Rc <Vec<Stmt>>,
    table: HashMap<String, usize>,
}
impl CodeGen {
    pub fn new(mut parser: Parser) -> Self {
        let code = parser.parse();

        Self {
            code: vec![],
            statements: Rc::new(code.to_vec()),
            table: parser.get_table().clone(),
        }
    }
    pub fn helper_reg(&mut self,op1:u8,op2:u8, lhs_reg: &Token, right_reg_imm: &Token) {
        let reg = lhs_reg.token_type.get_reg();
        let mut command: [u8; 4] = [0; 4];
        if right_reg_imm.token_type != TokenType::INT {
            let reg_2 = right_reg_imm.token_type.get_reg();
            command[3] = op1;
            command[2] = reg.0 as u8;
            command[1] = reg_2.0 as u8;
        } else {
            let reg_2 = right_reg_imm
                .literal
                .as_ref()
                .unwrap()
                .parse::<i16>()
                .expect("parsing error");

            let [high, low] = reg_2.to_be_bytes();

            command[3] = op2;
            command[2] = reg.0 as u8;
            command[1] = high;
            command[0] = low;
        }
        self.code.extend_from_slice(&command);
    }
    pub fn gen_(&mut self) -> &[u8] {

        for i in self.statements
            .clone()
            .iter(){
            match i {
                Stmt::MOD{
                    lhs_reg,
                    right_reg_imm,
                } => {
                    let reg = lhs_reg.token_type.get_reg();
                    let mut command: [u8; 4] = [0; 4];
                    if right_reg_imm.token_type != TokenType::INT {
                        let reg_2 = right_reg_imm.token_type.get_reg();
                        command[3] = 0x33;
                        command[2] = reg.0 as u8;
                        command[1] = reg_2.0 as u8;
                    } else {
                        let reg_2 = right_reg_imm
                            .literal
                            .as_ref()
                            .unwrap()
                            .parse::<i16>()
                            .expect("parsing error");

                        let [high, low] = reg_2.to_be_bytes();

                        command[3] = 0x34;
                        command[2] = reg.0 as u8;
                        command[1] = high;
                        command[0] = low;
                    }
                    self.code.extend_from_slice(&command);
                }, 
                Stmt::DIV{
                    lhs_reg,
                    right_reg_imm,
                } => {
                    let reg = lhs_reg.token_type.get_reg();
                    let mut command: [u8; 4] = [0; 4];
                    if right_reg_imm.token_type != TokenType::INT {
                        let reg_2 = right_reg_imm.token_type.get_reg();
                        command[3] = 0x13;
                        command[2] = reg.0 as u8;
                        command[1] = reg_2.0 as u8;
                    } else {
                        let reg_2 = right_reg_imm
                            .literal
                            .as_ref()
                            .unwrap()
                            .parse::<i16>()
                            .expect("parsing error");

                        let [high, low] = reg_2.to_be_bytes();

                        command[3] = 0x32;
                        command[2] = reg.0 as u8;
                        command[1] = high;
                        command[0] = low;
                    }
                    self.code.extend_from_slice(&command);
                },
                Stmt::MUL{
                    lhs_reg,
                    right_reg_imm,
                } => {
                    let reg = lhs_reg.token_type.get_reg();
                    let mut command: [u8; 4] = [0; 4];
                    if right_reg_imm.token_type != TokenType::INT {
                        let reg_2 = right_reg_imm.token_type.get_reg();
                        command[3] = 0x12;
                        command[2] = reg.0 as u8;
                        command[1] = reg_2.0 as u8;
                    } else {
                        let reg_2 = right_reg_imm
                            .literal
                            .as_ref()
                            .unwrap()
                            .parse::<i16>()
                            .expect("parsing error");

                        let [high, low] = reg_2.to_be_bytes();

                        command[3] = 0x31;
                        command[2] = reg.0 as u8;
                        command[1] = high;
                        command[0] = low;
                    }
                    self.code.extend_from_slice(&command);
                },
                Stmt::AND_OR_XOR {
                    type_op,
                    reg,
                    register_or_imm,
                } => {
                    if *type_op == TokenType::AND{
                        self.helper_reg(0x24, 0x25, reg, register_or_imm);
                    }else if *type_op == TokenType::OR {
                        self.helper_reg(0x26, 0x27, reg, register_or_imm);
                    }else if *type_op == TokenType::XOR {
                        self.helper_reg(0x28, 0x29, reg, register_or_imm);
                    }
                },
                Stmt::JMPLE{ to }=>{
                    let val = to.literal.as_ref().unwrap();
                    let val = (*self.table.get(val).unwrap()) as u32;
                    let mut command: [u8; 4] = [0; 4];
                    let [_, u2, u3, u4] = val.to_be_bytes();
                    command[3] = 0x10;
                    command[2] = u2;
                    command[1] = u3;
                    command[0] = u4;
                    self.code.extend_from_slice(&command);

                },
                Stmt::JMPGE{ to }=>{
                    let val = to.literal.as_ref().unwrap();
                    let val = (*self.table.get(val).unwrap()) as u32;
                    let mut command: [u8; 4] = [0; 4];
                    let [_, u2, u3, u4] = val.to_be_bytes();
                    command[3] = 0x09;
                    command[2] = u2;
                    command[1] = u3;
                    command[0] = u4;
                    self.code.extend_from_slice(&command);

                },
                Stmt::JMPZ { to }=>{
                    let val = to.literal.as_ref().unwrap();
                    let val = (*self.table.get(val).unwrap()) as u32;
                    let mut command: [u8; 4] = [0; 4];
                    let [_, u2, u3, u4] = val.to_be_bytes();
                    command[3] = 0x14;
                    command[2] = u2;
                    command[1] = u3;
                    command[0] = u4;
                    self.code.extend_from_slice(&command);

                },
                Stmt::PUSH { register_or_imm } => {
                    let mut command: [u8; 4] = [0; 4];
                    if register_or_imm.token_type != TokenType::INT {
                        let reg_2 = register_or_imm.token_type.get_reg();
                        command[3] = 0x23;
                        command[2] = reg_2.0 as u8;
                    } else {
                        let reg_2 = register_or_imm
                            .literal
                            .as_ref()
                            .unwrap()
                            .parse::<i32>()
                            .expect("parsing error");
                        if !(I24_MIN..=I24_MAX).contains(&reg_2) {
                            panic!("size overflow");
                        }
                        let [_, u2, u3, u4] = reg_2.to_be_bytes();

                        command[3] = 0x21;
                        command[2] = u2;
                        command[1] = u3;
                        command[0] = u4;
                    }
                    self.code.extend_from_slice(&command);
                }
                Stmt::POP { reg } => {
                    let reg = reg.token_type.get_reg();
                    let mut command: [u8; 4] = [0; 4];
                    command[3] = 0x22;
                    command[2] = reg.0 as u8;
                    self.code.extend_from_slice(&command);
                }
                Stmt::JMP { to } => {
                    let val = to.literal.as_ref().unwrap();
                    let val = (*self.table.get(val).unwrap()) as u32;
                    let mut command: [u8; 4] = [0; 4];
                    let [_, u2, u3, u4] = val.to_be_bytes();
                    command[3] = 0x16;
                    command[2] = u2;
                    command[1] = u3;
                    command[0] = u4;
                    self.code.extend_from_slice(&command);
                }
                Stmt::Call { to } => {
                    if to.token_type == TokenType::LabelCall {
                        let addr = (*self.table.get(to.literal.as_ref().unwrap()).unwrap()) as u32;
                        let mut command: [u8; 4] = [0; 4];
                        let [_, u1, u2, u3] = addr.to_be_bytes();
                        command[3] = 0x19;
                        command[2] = u1;
                        command[1] = u2;
                        command[0] = u3;
                        self.code.extend_from_slice(&command);
                    } else {
                        panic!("Expected a label");
                    }
                }
                Stmt::RET => {
                    let mut command: [u8; 4] = [0; 4];
                    command[3] = 0x20;
                    self.code.extend_from_slice(&command);
                }
                Stmt::NOP => {
                    let command: [u8; 4] = [0; 4];
                    self.code.extend_from_slice(&command);
                }
                Stmt::SUB {
                    lhs_reg,
                    right_reg_imm,
                } => {
                    let reg = lhs_reg.token_type.get_reg();
                    let mut command: [u8; 4] = [0; 4];
                    if right_reg_imm.token_type != TokenType::INT {
                        let reg_2 = right_reg_imm.token_type.get_reg();
                        command[3] = 0x04;
                        command[2] = reg.0 as u8;
                        command[1] = reg_2.0 as u8;
                    } else {
                        let reg_2 = right_reg_imm
                            .literal
                            .as_ref()
                            .unwrap()
                            .parse::<i16>()
                            .expect("parsing error");

                        let [high, low] = reg_2.to_be_bytes();

                        command[3] = 0x05;
                        command[2] = reg.0 as u8;
                        command[1] = high;
                        command[0] = low;
                    }
                    self.code.extend_from_slice(&command);
                }
                Stmt::Print { reg } => {
                    let reg = reg.token_type.get_reg();
                    let mut commad: [u8; 4] = [0; 4];
                    commad[3] = 0x11;
                    commad[2] = reg.0 as u8;
                    commad[1] = 0;
                    commad[0] = 0;
                    self.code.extend_from_slice(&commad);
                }
                Stmt::ADD {
                    lhs_reg,
                    right_reg_imm,
                } => {
                    let reg = lhs_reg.token_type.get_reg();
                    let mut command: [u8; 4] = [0; 4];
                    if right_reg_imm.token_type != TokenType::INT {
                        let reg_2 = right_reg_imm.token_type.get_reg();
                        command[3] = 0x02;
                        command[2] = reg.0 as u8;
                        command[1] = reg_2.0 as u8;
                    } else {
                        let reg_2 = right_reg_imm
                            .literal
                            .as_ref()
                            .unwrap()
                            .parse::<i16>()
                            .expect("parsing error");

                        let [high, low] = reg_2.to_be_bytes();

                        command[3] = 0x03;
                        command[2] = reg.0 as u8;
                        command[1] = high;
                        command[0] = low;
                    }
                    self.code.extend_from_slice(&command);
                }
                Stmt::CMP {
                    from_reg,
                    register_or_imm,
                } => {
                    let reg = from_reg.token_type.get_reg();
                    let mut command: [u8; 4] = [0; 4];
                    if register_or_imm.token_type != TokenType::INT {
                        let reg_2 = register_or_imm.token_type.get_reg();
                        command[3] = 0x06;
                        command[2] = reg.0 as u8;
                        command[1] = reg_2.0 as u8;
                    } else {
                        let reg_2 = register_or_imm
                            .literal
                            .as_ref()
                            .unwrap()
                            .parse::<u16>()
                            .expect("parsing error");

                        let [high, low] = reg_2.to_be_bytes();

                        command[3] = 0x18;
                        command[2] = reg.0 as u8;
                        command[1] = high;
                        command[0] = low;
                    }
                    self.code.extend_from_slice(&command);
                }
                Stmt::JMPG { to } => {
                    let val = to.literal.as_ref().unwrap();
                    let val = (*self.table.get(val).unwrap()) as u32;
                    let mut command: [u8; 4] = [0; 4];
                    let [_, u2, u3, u4] = val.to_be_bytes();
                    command[3] = 0x07;
                    command[2] = u2;
                    command[1] = u3;
                    command[0] = u4;
                    self.code.extend_from_slice(&command);
                }
                Stmt::JMPL { to } => {
                    let val = to.literal.as_ref().unwrap();
                    let val = (*self.table.get(val).unwrap()) as u32;
                    let mut command: [u8; 4] = [0; 4];
                    let [_, u2, u3, u4] = val.to_be_bytes();
                    command[3] = 0x08;
                    command[2] = u2;
                    command[1] = u3;
                    command[0] = u4;
                    self.code.extend_from_slice(&command);
                }

                Stmt::MovLit {
                    from,
                    register_or_imm,
                } => {
                    // 0x01 mov r1,10
                    // 0x17 mov r2, r1
                    let mut command: [u8; 4] = [0; 4];
                    let reg = from.token_type.get_reg();
                    if register_or_imm.token_type != TokenType::INT {
                        // 0x17 0xn 0xm 0x00
                        let to = register_or_imm.token_type.get_reg();
                        command[3] = 0x17;
                        command[2] = reg.0 as u8;
                        command[1] = to.0 as u8;
                        command[0] = 0x00;
                    } else {
                        let num = register_or_imm
                            .literal
                            .as_ref()
                            .unwrap()
                            .parse::<i16>()
                            .expect("number parsing error");
                        let [high, low] = num.to_be_bytes();
                        command[3] = 0x01;
                        command[2] = reg.0 as u8;
                        command[1] = high;
                        command[0] = low;
                    }
                    self.code.extend_from_slice(&command);
                }
                Stmt::Halt { token: _ } => {
                    self.code.extend_from_slice(&[0x00, 0x00, 0x00, 0xFF]);
                }
            }
        }
        &self.code
    }
}
