#![allow(dead_code)]

use std::{fs::File, io::Read};

use crate::{backend::CodeGen, parser::Parser, scanner::Scanner};

mod backend;
mod parser;
mod scanner;
mod vm;

fn main() {
    let mut file = File::open("asm1.mm").unwrap();
    let mut buff = String::new();

    file.read_to_string(&mut buff).unwrap();

    let mut vm = vm::VM::default();

    let mut scanner = Scanner::new(&buff);
    let p = scanner.parse();

    let parser = Parser::new(p.to_vec());

    let mut code_back = CodeGen::new(parser);
    let code = code_back.gen_();

    let loop_program = [
        0x05, 0x00, 0x01, 0x01, // MOV r1,5
        0x00, 0x00, 0x02, 0x01, // MOV r2,0
        0x00, 0x00, 0x01, 0x11, // print r1
        0x01, 0x00, 0x01, 0x05, // SUB r1,1
        0x00, 0x02, 0x01, 0x06, // CMP r1,r2
        0x02, 0x00, 0x00, 0x09, // JMPGE
        0x00, 0x00, 0x00, 0xFF, // HALT
    ];

    vm.copy(code);

    vm.execute();
    println!("<<reg -> {:?}>>", vm.reg);
}

/*
*  let _program = [
        0xF9, 0xFF, 0x01, 0x01, // mov r1, -7
        0x01, 0x00, 0x02, 0x01, // mov r2, 1
        0x00, 0x01, 0x02, 0x02, // add r2,r1
        0x04, 0x00, 0x03, 0x03, // add r3,4
        0x00, 0x00, 0x00, 0xFF, // halt
    ];

    // a = 10
    // while a > 0 {
    //    print(a)
    //    a--;
    // }

    let loop_program =[
      0x05, 0x00, 0x01 ,0x01,   // MOV r1,5
      0x00, 0x00, 0x02, 0x01,   // MOV r2,0
      0x00, 0x00, 0x01, 0x11,   // print r1
      0x01, 0x00, 0x01, 0x05,   // SUB r1,1
      0x00, 0x02, 0x01, 0x06,   // CMP r1,r2
      0x02, 0x00, 0x00, 0x09,   // JMPGE
      0x00, 0x00, 0x00, 0xFF    // HALT
    ];

*/
