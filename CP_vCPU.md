# CP vCPU (Commander Proto Virtual CPU)
## Official Architecture Specification

---

## 1. Overview

CP vCPU (Commander Proto Virtual CPU) is a minimal, lightweight, and deterministic virtual processor designed for simplicity, portability, and theoretical completeness.

It is built to:
- Run efficiently on low-end and modern devices
- Support a minimal operating system
- Execute bare-metal programs written in CP language
- Enable simple 2D game development
- Be compiled instead of interpreted for performance

---

## 2. Core Architecture

### 2.1 Execution Model

- Deterministic
- Instruction Counter (IC) driven
- Two virtual cores
- Compiled execution preferred (AOT/JIT possible)
- No hardware-level simulation

---

## 3. Memory Model

### 3.1 Memory Structure

- Program consists of multiple lines
- Each line contains exactly 16 boxes
- Each box stores a numeric value
- All boxes initialize to `0`

Memory is represented internally as:
Memory[line][box]

Example:
Line 0 → [0][0][0]...[0] (16 boxes) 
Line 1 → [0][0][0]...[0] ...

---

## 4. Core Components

### 4.1 Pointer Register

Tracks:
- Current Line
- Current Box Index (0–15)

### 4.2 Instruction Counter (IC)

Tracks:
- Current executing instruction

### 4.3 Virtual Cores

- Total: 2 cores
- Cooperative scheduling
- No hardware threading
- No pipelining

---

## 5. Instruction Set

### 5.1 Pointer Movement

| Instruction | Description |
|------------|------------|
| `L` | Move pointer left |
| `R` | Move pointer right |

---

### 5.2 Arithmetic Operations

| Instruction | Description |
|------------|------------|
| `A` | Increment current box value by 1 |
| `S` | Decrement current box value by 1 |

---

### 5.3 Input / Output

| Instruction | Description |
|------------|------------|
| `:(n)` | Take user input into box `n` |
| `;(n)` | Output value of box `n` |
| `.(value)` | Store hidden literal value into current box |

Allowed stored values:
- Integers
- Decimals
- Alphabets
- Alphanumeric strings
- Emojis  
(Not special characters)

---

### 5.4 Cross-Line Access
,[x,n]
Access box `n` from line `x`.

- Temporary access
- Usable only within that row context

---

### 5.5 Loop System
{[N]B} // loop body {E}
Loop executes while:
Current box value ≠ 0

This provides conditional repetition.

---

### 5.6 Jump System
IC == n_J(x)

where,  
            n = number 
            x = number of line 
            
Instruction counter-based jump mechanism.

Allows:
- Manual control flow
- Structured branching
- Program redirection

---

## 6. Removed Features (Intentional Design Simplicity)

The CP vCPU does NOT include:

- Vector registers
- Floating point units
- Complex general-purpose registers
- Pipelining
- Hardware multithreading
- GPU simulation
- Cycle-accurate simulation

This ensures:
- Lightweight execution
- Device compatibility
- Minimal overhead

---

## 7. Operating System Support

CP vCPU supports:

- Minimal kernel design
- Cooperative multitasking
- Basic scheduler
- Simple memory management
- Text-based or grid-based UI
- Bare-metal applications

---

## 8. Game Support

CP vCPU can support:

- Text-based games
- 2D grid games
- Turn-based systems
- Simple frame loop (FPS control)
- Sprite-like memory rendering

Not designed for:
- 3D graphics
- Heavy physics engines
- GPU-level rendering

---

## 9. Performance Philosophy

- Designed for compilation rather than interpretation
- Can be compiled to ARM64 native code
- Optimized for low memory footprint
- Suitable for old and modern devices

---

## 10. Theoretical Capability

CP vCPU includes:

- Memory
- Arithmetic
- Conditional looping
- Jump control

Therefore:

It is computationally universal (Turing-complete equivalent).

---

## 11. Design Goals

- Minimalism
- Determinism
- Portability
- Educational clarity
- Experimental computing foundation
- Lightweight OS experimentation
- Simple 2D game experimentation

---

## End of Specification