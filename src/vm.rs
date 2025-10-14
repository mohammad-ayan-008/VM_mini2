const CARRY_FLAG: u8 = 0b0000_0010;
const ZERO_FLAG: u8 = 0b0000_0001;
const GRETER_FLAG: u8 = 0b0001_0000;
const LESSER_FLAG: u8 = 0b0010_0000;
type Flags = u8;

const MEMORY_SIZE: usize = 8 * 1024;

pub struct VM {
    flag: Flags,
    pc: u32,
    sp: u16,
    pub reg: [i32; 8],
    stack: [i32; 16],
    pub memory: [u8; MEMORY_SIZE],
}
impl Default for VM {
    fn default() -> Self {
        Self {
            flag: 0,
            pc: 0,
            sp: 0,
            reg: [0; 8],
            stack: [0; 16],
            memory: [0; MEMORY_SIZE],
        }
    }
}

impl VM {
    pub fn push(&mut self, value: i32) {
        assert!((self.sp as usize) < self.stack.len(), "stack overflow ");
        self.stack[self.sp as usize] = value;
        self.sp += 1;
    }

    pub fn pop(&mut self) -> i32 {
        assert!(self.sp > 0, "stack underflow");
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    pub fn extract_u32(&mut self) -> u32 {
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
            let [inst1, inst2, inst3, inst4] = self.extract_u32().to_be_bytes();
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

                    if res == 0 {
                        self.flag |= ZERO_FLAG;
                    }
                    if res > 0 {
                        self.flag |= GRETER_FLAG;
                    } else {
                        self.flag |= LESSER_FLAG;
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
                },
                //cmp rn imm
                0x18 => {
                    self.flag &= !(ZERO_FLAG | GRETER_FLAG | LESSER_FLAG);

                    let n = inst2 as usize;
                    let val = i16::from_be_bytes([inst3,inst4]) as i32;
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

    pub fn copy(&mut self, program: &[u8]) {
        self.memory[(self.pc as usize)..program.len()].copy_from_slice(program);
    }
}
