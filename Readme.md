---

# Tiny Virtual Machine — Assembly-Like Language in Rust

This project is a minimal **virtual machine** built from scratch in Rust.
It executes a custom **assembly-like instruction set**, supporting arithmetic, control flow, comparison, and basic I/O — all designed to simulate how a CPU fetches, decodes, and executes instructions.

---

## ⚙️ Overview

This tiny VM is a learning-oriented project that mimics the core of a real processor:

* **8 General-purpose registers**
* **8KB memory**
* **Stack-based execution**
* **Program counter & flag register**
* **Little-endian instruction storage**
* **Custom bytecode instruction set**

You can write small assembly programs, assemble them into bytecode, and execute them directly in the virtual machine.

---

## 🧮 Supported Instructions

| Instruction   | Description                          |
| ------------- | ------------------------------------ |
| `MOV Rn, imm` | Move immediate value into a register |
| `MOV Rn, Rm`  | Copy value between registers         |
| `ADD Rn, Rm`  | Add two registers                    |
| `ADD Rn, imm` | Add immediate to register            |
| `SUB Rn, Rm`  | Subtract two registers               |
| `SUB Rn, imm` | Subtract immediate from register     |
| `CMP Rn, Rm`  | Compare two registers (sets flags)   |
| `CMP Rn, imm` | Compare register with immediate      |
| `MUL Rn, imm` | Multiply register with immediate     |
| `DIV Rn, Rm`  | Divide one register by another       |
| `PRINT Rn`    | Print the value of a register        |
| `JMP label`   | Unconditional jump                   |
| `JMPG label`  | Jump if greater flag is set          |
| `JMPL label`  | Jump if lesser flag is set           |
| `JMPGE label` | Jump if greater or equal             |
| `JMPLE label` | Jump if lesser or equal              |
| `JMPZ label`  | Jump if zero flag is set             |
| `JMPNZ label` | Jump if zero flag is not set         |
| `HALT`        | Stop execution                       |

---

## ⚑ Flags

The VM maintains a flag register for comparisons and arithmetic operations:

| Flag                 | Description                         |
| -------------------- | ----------------------------------- |
| **Zero Flag (Z)**    | Set when a result is zero           |
| **Carry Flag (C)**   | Set when arithmetic overflow occurs |
| **Greater Flag (G)** | Set when a comparison is greater    |
| **Lesser Flag (L)**  | Set when a comparison is lesser     |

These flags allow branching instructions like `JMPG`, `JMPL`, `JMPZ`, etc.

---

## 🔁 Example Program

### Assembly

```asm
mov r1, 5
loop:
print r1
sub r1, 1
cmp r1, 0
jmpg loop
exit:
halt
```

### Output

```
5
4
3
2
1
Halt
```

### Explanation

* Load `5` into register `r1`
* Print it and subtract `1`
* Compare with `0`
* Jump back to label `loop` while greater than zero
* Halt when finished

---

## 🧱 Memory Layout

Each instruction is encoded into **4 bytes**:

```
[ opcode | operand1 | operand2 | operand3 ]
```

* Stored in **little-endian** order
* The **program counter (PC)** increments by 4 after each instruction
* The VM fetches, decodes, and executes instructions one by one

---

## 📦 Features

✅ Stack operations (push/pop)
✅ Manual flag management
✅ Label-based jumps
✅ Compact 8KB memory model
✅ Zero-dependency runtime
✅ Easily extensible instruction set

---

## 🚀 Example Use Cases

* Learning how CPU instruction decoding works
* Experimenting with custom bytecode formats
* Building toy compilers or assemblers
* Teaching low-level programming and flag logic

---

## 🧩 Future Improvements

* Function calls (`CALL` / `RET`)
* Memory load/store (`LDR` / `STR`)
* Better debugging and disassembly tools
* REPL-like interface for live instruction execution
* Binary assembler to compile `.asm` → `.bin`

---

## 📜 License

This project is open-source and licensed under the **MIT License**.
Use it freely for learning, research, or integration in educational tools.

---


