use crate::parser::Data;

const CARRY_FLAG: u8 = 0b0000_0010;
const ZERO_FLAG: u8 = 0b0000_0001;
const GRETER_FLAG: u8 = 0b0001_0000;
const LESSER_FLAG: u8 = 0b0010_0000;
type Flags = u8;

const CODE_START: usize = 0x0000;
const DATA_START: usize = 0x2000;
const BSS_START: usize = 0x3000;
const HEAP_START: usize = 0x4000;
const STACK_START: usize = MEMORY_SIZE - 1;
const STACK_END: usize = HEAP_START;
const MEMORY_SIZE: usize = 64 * 1024;

pub struct VM {
    flag: Flags,
    pc: u32,
    sp: usize,
    pub reg: [i32; 8],
    pub memory: [u8; MEMORY_SIZE],
}
impl Default for VM {
    fn default() -> Self {
        Self {
            flag: 0,
            pc: CODE_START as u32,
            sp: STACK_START,
            reg: [0; 8],
            memory: [0; MEMORY_SIZE],
        }
    }
}

impl VM {
    pub fn push(&mut self, value: i32) {
        assert!(self.sp >= STACK_END, "stack overflow");
        self.sp -= 4;
        self.memory[self.sp..self.sp + 4].copy_from_slice(&value.to_be_bytes());
    }

    pub fn pop(&mut self) -> i32 {
        assert!(self.sp < STACK_START, "stack underflow");

        let val: [u8; 4] = self.memory[self.sp..self.sp + 4].try_into().unwrap();
        self.sp += 4;
        i32::from_be_bytes(val)
    }

    pub fn extract_u32(&mut self) -> u32 {
        assert!(self.pc + 3 < DATA_START as u32, "PC out of bounds");
        let lsb0 = self.memory[self.pc as usize];
        let lsb1 = self.memory[(self.pc + 1) as usize];
        let lsb2 = self.memory[(self.pc + 2) as usize];
        let lsb3 = self.memory[(self.pc + 3) as usize];
        self.pc += 4;
        u32::from_le_bytes([lsb0, lsb1, lsb2, lsb3])
    }

    pub fn execute(&mut self) {
        // [opcode (8 bits ) | rest ----]
        // little endian bytes for memory structure MSB at last and LSB first
        'lp: loop {
            let ins = self.extract_u32();
            let [inst1, inst2, inst3, inst4] = ins.to_be_bytes();
            // println!("{:?}",self.reg);
            //println!("i1 {inst1} i2 {inst2} i3 {inst3} i4 {inst4}");
            let op_code = inst1;
            match op_code {
                0x00 => {}
                0xFF => {
                    println!("Halt");
                    break 'lp;
                }
                // Mov rn,i16
                0x01 => {
                    let reg = inst2 as usize;
                    let value = i16::from_be_bytes([inst3, inst4]) as i32;
                    self.reg[reg] = value;
                }
                // Add rn + rm
                0x02 => {
                    self.flag &= !CARRY_FLAG;
                    self.flag &= !ZERO_FLAG;
                    let n = inst2 as usize;
                    let m = inst3 as usize;
                    let overflow = self.reg[n].overflowing_add(self.reg[m]);
                    if overflow.1 {
                        self.flag |= CARRY_FLAG;
                    }
                    if overflow.0 == 0 {
                        self.flag |= ZERO_FLAG;
                    }
                    self.reg[n] = overflow.0;
                }
                // Add rn + imm
                0x03 => {
                    self.flag &= !CARRY_FLAG;
                    self.flag &= !ZERO_FLAG;
                    let n = inst2 as usize;
                    let m = i16::from_be_bytes([inst3, inst4]) as i32;
                    let overflow = self.reg[n].overflowing_add(m);
                    if overflow.1 {
                        self.flag |= CARRY_FLAG;
                    }
                    if overflow.0 == 0 {
                        self.flag |= ZERO_FLAG;
                    }
                    self.reg[n] = overflow.0;
                }
                // sub rn + rm
                0x04 => {
                    self.flag &= !CARRY_FLAG;
                    self.flag &= !ZERO_FLAG;
                    let n = inst2 as usize;
                    let m = inst3 as usize;
                    let overflow = self.reg[n].overflowing_sub(self.reg[m]);
                    if overflow.1 {
                        self.flag |= CARRY_FLAG;
                    }
                    if overflow.0 == 0 {
                        self.flag |= ZERO_FLAG;
                    }
                    self.reg[n] = overflow.0;
                }
                // sub rn + imm
                0x05 => {
                    self.flag &= !CARRY_FLAG;
                    self.flag &= !ZERO_FLAG;
                    let n = inst2 as usize;
                    let m = i16::from_be_bytes([inst3, inst4]) as i32;
                    let overflow = self.reg[n].overflowing_sub(m);
                    if overflow.1 {
                        self.flag |= CARRY_FLAG;
                    }
                    if overflow.0 == 0 {
                        self.flag |= ZERO_FLAG;
                    }
                    self.reg[n] = overflow.0;
                }

                //cmp rn rm
                0x06 => {
                    self.flag &= !(ZERO_FLAG | GRETER_FLAG | LESSER_FLAG);

                    let n = inst2 as usize;
                    let m = inst3 as usize;

                    let res = self.reg[n].overflowing_sub(self.reg[m]).0;
                    // println!("{:?}",res);
                    if res == 0 {
                        self.flag |= ZERO_FLAG;
                    }
                    if res > 0 {
                        self.flag |= GRETER_FLAG;
                        // println!("G flag set");
                    } else {
                        self.flag |= LESSER_FLAG;
                        // println!("L flag set");
                    }
                }
                // JMPG
                0x07 => {
                    let n = u32::from_be_bytes([0x00, inst2, inst3, inst4]);

                    if (self.flag & GRETER_FLAG) != 0 {
                        self.pc = n * 4;
                    }
                }
                // JMPL
                0x08 => {
                    let n = u32::from_be_bytes([0x00, inst2, inst3, inst4]);
                    if (self.flag & LESSER_FLAG) != 0 {
                        self.pc = n * 4;
                    }
                }
                // JMPGE
                0x09 => {
                    let n = u32::from_be_bytes([0x00, inst2, inst3, inst4]);
                    if (self.flag & GRETER_FLAG) != 0 || (self.flag & ZERO_FLAG) != 0 {
                        self.pc = n * 4;
                    }
                }
                // JMPLE
                0x10 => {
                    let n = u32::from_be_bytes([0x00, inst2, inst3, inst4]);
                    if (self.flag & LESSER_FLAG) != 0 || (self.flag & ZERO_FLAG) != 0 {
                        self.pc = n * 4;
                    }
                }
                //print reg
                0x11 => {
                    let n = inst2 as usize;
                    println!("{:?}", self.reg[n]);
                }
                //mul rn, imm
                0x12 => {
                    self.flag &= !(CARRY_FLAG | ZERO_FLAG);
                    let n = inst2 as usize;
                    let num = i16::from_be_bytes([inst3, inst4]) as i32;
                    let res = self.reg[n].overflowing_mul(num);
                    self.reg[n] = res.0;
                    if res.1 {
                        self.flag |= CARRY_FLAG
                    }
                    if res.0 == 0 {
                        self.flag |= ZERO_FLAG
                    }
                }
                // Div rn ,rm
                0x13 => {
                    self.flag &= !ZERO_FLAG;
                    let n = inst2 as usize;
                    let m = inst3 as usize;
                    if self.reg[m] == 0 {
                        println!("Divide by zero");
                        return;
                    }
                    self.reg[n] /= self.reg[m];
                    if self.reg[n] == 0 {
                        self.flag |= ZERO_FLAG
                    }
                }
                // JMPZ
                0x14 => {
                    let n = u32::from_be_bytes([0x00, inst2, inst3, inst4]);
                    if self.flag & ZERO_FLAG != 0 {
                        self.pc = n * 4;
                    }
                }
                // JMPNZ
                0x15 => {
                    let n = u32::from_be_bytes([0x00, inst2, inst3, inst4]);
                    if self.flag & ZERO_FLAG == 0 {
                        self.pc = n * 4;
                    }
                }
                //JUMP
                0x16 => {
                    let n = u32::from_be_bytes([0x00, inst2, inst3, inst4]);
                    self.pc = n * 4;
                }
                // MOV rn,rm
                0x17 => {
                    let n = inst2 as usize;
                    let m = inst3 as usize;
                    self.reg[n] = self.reg[m];
                }
                //cmp rn imm
                0x18 => {
                    self.flag &= !(ZERO_FLAG | GRETER_FLAG | LESSER_FLAG);

                    let n = inst2 as usize;
                    let val = i16::from_be_bytes([inst3, inst4]) as i32;
                    let res = self.reg[n].overflowing_sub(val).0;

                    if res == 0 {
                        self.flag |= ZERO_FLAG;
                    }
                    if res > 0 {
                        self.flag |= GRETER_FLAG;
                    } else {
                        self.flag |= LESSER_FLAG;
                    }
                }
                //call addr
                0x19 => {
                    let addr = u32::from_be_bytes([0x00, inst2, inst3, inst4]);
                    self.push(self.pc as i32);
                    self.pc = addr * 4;
                }
                //ret
                0x20 => {
                    self.pc = self.pop() as u32;
                }
                //Push imm
                0x21 => {
                    let sign = if inst2 & 0x80 != 0 { 0xFF } else { 0x00 };
                    let val = i32::from_be_bytes([sign, inst2, inst3, inst4]);
                    self.push(val);
                }
                //pop
                0x22 => {
                    let reg = inst2 as usize;
                    self.reg[reg] = self.pop();
                }
                //Push reg
                0x23 => {
                    let reg = inst2 as usize;
                    self.push(self.reg[reg]);
                }
                //AND rn , rm
                0x24 => {
                    let n = inst2 as usize;
                    let m = inst3 as usize;
                    self.flag &= !ZERO_FLAG;
                    self.reg[n] &= self.reg[m];
                    if self.reg[n] == 0 {
                        self.flag |= ZERO_FLAG;
                    }
                }
                //AND rn , imm
                0x25 => {
                    let n = inst2 as usize;
                    let imm = i16::from_be_bytes([inst3, inst4]) as i32;
                    self.flag &= !ZERO_FLAG;
                    self.reg[n] &= imm;
                    if self.reg[n] == 0 {
                        self.flag |= ZERO_FLAG;
                    }
                }
                //OR rn , rm
                0x26 => {
                    let n = inst2 as usize;
                    let m = inst3 as usize;
                    self.flag &= !ZERO_FLAG;
                    self.reg[n] |= self.reg[m];
                    if self.reg[n] == 0 {
                        self.flag |= ZERO_FLAG;
                    }
                }
                //OR rn , imm
                0x27 => {
                    let n = inst2 as usize;
                    let imm = i16::from_be_bytes([inst3, inst4]) as i32;
                    self.flag &= !ZERO_FLAG;
                    self.reg[n] |= imm;
                    if self.reg[n] == 0 {
                        self.flag |= ZERO_FLAG;
                    }
                }
                //XOR rn , rm
                0x28 => {
                    let n = inst2 as usize;
                    let m = inst3 as usize;
                    self.flag &= !ZERO_FLAG;
                    self.reg[n] ^= self.reg[m];
                    if self.reg[n] == 0 {
                        self.flag |= ZERO_FLAG;
                    }
                }
                //XOR rn , imm
                0x29 => {
                    let n = inst2 as usize;
                    let imm = i16::from_be_bytes([inst3, inst4]) as i32;
                    self.flag &= !ZERO_FLAG;
                    self.reg[n] ^= imm;
                    if self.reg[n] == 0 {
                        self.flag |= ZERO_FLAG;
                    }
                }
                //NOT rn
                0x30 => {
                    let n = inst2 as usize;
                    self.flag &= !ZERO_FLAG;
                    self.reg[n] = !self.reg[n];
                    if self.reg[n] == 0 {
                        self.flag |= ZERO_FLAG;
                    }
                }
                //mul rn, imm 0x12
                //mul rn,rm
                0x31 => {
                    self.flag &= !(CARRY_FLAG | ZERO_FLAG);
                    let n = inst2 as usize;
                    let m = inst3 as usize;
                    let res = self.reg[n].overflowing_mul(self.reg[m]);
                    self.reg[n] = res.0;
                    if res.1 {
                        self.flag |= CARRY_FLAG
                    }
                    if res.0 == 0 {
                        self.flag |= ZERO_FLAG
                    }
                }
                // Div rn ,rm 0x13
                // Div rn,imm
                0x32 => {
                    self.flag &= !ZERO_FLAG;
                    let n = inst2 as usize;
                    let val = i16::from_be_bytes([inst3, inst4]) as i32;
                    if val == 0 {
                        println!("Divide by zero");
                        return;
                    }
                    self.reg[n] /= val;
                    if self.reg[n] == 0 {
                        self.flag |= ZERO_FLAG
                    }
                }
                //mod rn,rm
                0x33 => {
                    self.flag &= !(CARRY_FLAG | ZERO_FLAG);
                    let n = inst2 as usize;
                    let m = inst3 as usize;
                    let res = self.reg[n] % (self.reg[m]);
                    self.reg[n] = res;
                    if res == 0 {
                        self.flag |= ZERO_FLAG
                    }
                }
                //mod rn,imm
                0x34 => {
                    self.flag &= !ZERO_FLAG;
                    let n = inst2 as usize;
                    let val = i16::from_be_bytes([inst3, inst4]) as i32;
                    self.reg[n] %= val;
                    if self.reg[n] == 0 {
                        self.flag |= ZERO_FLAG
                    }
                }
                //mov rn , [addr] // value of u8
                0x35 => {
                    let reg = inst2 as usize;
                    let offset = DATA_START + u16::from_be_bytes([inst3, inst4]) as usize;
                    assert!(
                        (DATA_START..BSS_START).contains(&offset),
                        "seg fault {:?}",
                        offset
                    );
                    self.reg[reg] = self.memory[offset] as i8 as i16 as i32;
                },
                 //cmp rn addr // value u8
                0x36 => {
                    self.flag &= !(ZERO_FLAG | GRETER_FLAG | LESSER_FLAG);
                    let offset = DATA_START + u16::from_be_bytes([inst3, inst4]) as usize;
                    assert!(
                    (DATA_START..BSS_START).contains(&offset),
                        "seg fault {:?}",
                        offset
                    );
                    let n = inst2 as usize;
                    let val = self.memory[offset] as i8 as i16 as i32;
                    let res = self.reg[n].overflowing_sub(val).0;

                    if res == 0 {
                        self.flag |= ZERO_FLAG;
                    }
                    if res > 0 {
                        self.flag |= GRETER_FLAG;
                    } else {
                        self.flag |= LESSER_FLAG;
                    }
                }

                _ => todo!(),
            }
        }
    }

    pub fn copy(&mut self, program: &[u8], data: &[u8]) {
        let start = CODE_START;
        let end = program.len() + start;
        assert!(end < DATA_START, "program is too large");
        self.memory[start..program.len()].copy_from_slice(program);

        let start = DATA_START;
        let end = start + data.len();
        assert!(end <= self.memory.len(), "data too big for memory");
        assert!(end < BSS_START, "seg fault");
        self.memory[start..end].copy_from_slice(data);
    }
}
