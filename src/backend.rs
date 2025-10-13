use crate::{parser::Stmt, scanner::{Token, TokenType}};

pub struct CodeGen{
    code:Vec<u8>,
    statements:Vec<Stmt>,
}
impl CodeGen{
    pub fn new(stmts:&[Stmt])->Self{
        Self { code: vec![], statements: stmts.to_vec()}
    }

    pub fn gen_(&mut self,)->&[u8]{
        for i in &self.statements{
            match i {
                Stmt::MovLit { from, register_or_imm }=>{
                    // 0x01 mov r1,10
                    // 0x17 mov r2, r1
                    let mut command:[u8;4] =[0;4];
                    let reg = from.token_type.get_reg();
                    if register_or_imm.token_type != TokenType::INT{
                        // 0x17 0xn 0xm 0x00
                        let to = register_or_imm.token_type.get_reg();
                        command[3]=0x17;
                        command[2]= reg.0  as u8;
                        command[1]= to.0 as u8;
                        command[0]= 0x00;
                    } else {
                       let num = register_or_imm
                            .literal
                            .as_ref()
                            .unwrap()
                            .parse::<i16>().expect("number parsing error");
                        let [high,low]= num.to_be_bytes();
                        command[3]= 0x01;
                        command[2]= reg.0 as u8;
                        command[1]= high;
                        command[0]= low;
                    }
                    self.code.extend_from_slice(&command);
                },
                Stmt::Halt { token }=>{
                    self.code.extend_from_slice(&[0x00,0x00,0x00,0xFF]);
                }
            }
        }
        &self.code
    }
}
