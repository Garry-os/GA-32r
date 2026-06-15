# GA-32 CPU architecture specification
## Overview
This is a **RISC-like** architecture. Below are the keypoints:
* **32-bit** CPU
* Uses **MM/IO** to communicate with hardwares.
* **Big-endian** byte order.
* The **stack** grows downward.
* **Fixed** instruction width (32-bit).

## Registers
The CPU features five general-purpose registers, two 32-bit pointers, and a 4-bit flag status register.

| Register | Size   | Description       |
| -------- | ------ | ----------------- |
| A        | 32-bit | General purpose   |
| B        | 32-bit | General purpose   |
| C        | 32-bit | General purpose   |
| D        | 32-bit | General purpose   |
| E        | 32-bit | Jumper register   |
| PC       | 32-bit | Program counter   |
| SP       | 32-bit | Stack pointer     |
| Z        | 4-bit  | CPU status        |

Z register (MSB to LSB):
- ZERO
- CARRY
- NEGATIVE
- OVERFLOW

**NOTE:** The Z and PC register isn't directly accessible (read-only) by the software or program by using regular instructions like `MOVE`.  
Instead, they're accessed indirectly (See **Instructions** for more details).  

<details>
<summary>Instructions list</summary>

## Instructions
**Instruction format (2 argument):** `[OP] [destination], [source]`  
**Instruction format (1 argument):** `[OP] [source]`  
`imm` stands for immediate value (16 bit).  
`reg` stands for register (32 bit).  
Any instructions that doesn't have **Flags affected** section meaning that it doesn't affect any flags.

Instruction layout:  
XXXXXXXXYYYYZZZZIIIIIIIIIIIIIIII  
- X: Instruction opcode (8 bits)  
- Y: Source register ID (4 bits)  
- Z: Destination register ID (4 bits)  
- I: Address/imm (16 bits)  
Each **instruction's** size is fixed (32-bit).

### MOV
**Format:** MOV [reg], [imm/reg]  
**Opcode:** 0x01  
**Description:** Set [reg] to [imm/reg].
### LOAD
**Format:** LOAD [reg], [imm/reg]  
**Opcode:** 0x02  
**Description:** Load value directly from [imm] or the memory address in [reg].
### STORE
**Format:** STORE [reg/imm], [reg]  
**Opcode:** 0x03  
**Description:** Store a value from [reg] into the memory address from [reg/imm].
### PUSH
**Format**: PUSH [imm/reg]  
**Opcode:** 0x04  
**Description:** Push a value from [imm/reg] into the stack.
### POP
**Format:** POP [reg]  
**Opcode:** 0x05  
**Description:** Pop a value from the stack into [reg].
### JNZ
**Format:** JNZ [imm/reg]  
**Opcode:** 0x06  
**Description:** Jump (by setting PC) to the address in register E if [imm/reg] is not zero; otherwise continue execution normally. 
### ADD
**Format:** ADD [reg], [imm/reg]  
**Opcode:** 0x07  
**Description:** Add [imm/reg] to [reg] ([reg] + [imm/reg]).  
**Flags affected:**
- ZERO: Set if the 32-bit result is exactly 0; otherwise cleared.
- CARRY: Set if the operation exceed unsigned 32-bit limit; otherwise cleared.
- NEGATIVE: Set if bit 31 of the result is set (negative); otherwise cleared.
- OVERFLOW: Set if the operation exceed signed 32-bit limit; otherwise cleared.
### SUB
**Format:** SUB [reg], [imm/reg]  
**Opcode:** 0x08  
**Description:** Subtract [imm/reg] from [reg] ([reg] - [imm/reg]).  
**Flags affected:**
- ZERO: Set if the 32-bit result is exactly 0; otherwise cleared.
- CARRY: Set if the operation underflow unsigned 32-bit limit; otherwise cleared.
- NEGATIVE: Set if bit 31 of the result is set (negative); otherwise cleared.
- OVERFLOW: Set if the operation underflow signed 32-bit limit; otherwise cleared.
### CALL
**Format:** CALL [imm/reg]  
**Opcode:** 0x09  
**Description:** Jump to the address in [imm/reg] and save the current address in the stack.
### RET
**Format:** RET  
**Opcode:** 0x0A  
**Description:** Return from a **CALL** to the address saved in the stack earlier.
### HLT
**Format:** HLT  
**Opcode:** 0x0B  
**Description:** Halt current CPU execution.
### NOP
**Format:** NOP  
**Opcode:** 0x0C  
**Description:** Performs no operation (1 CPU cycle).
### MOVU
**Format:** MOVU [reg], [imm]  
**Opcode:** 0x0D  
**Description:** Set [reg] upper 16-bit to [imm].  
**NOTE:** This instruction exists because of the 16-bit limit in the immediate register.
</details>

## Behaviors
### Reset vector
On reset vector, the CPU read a 32-bit big-endian address from (MSB to LSB):
- `0xFFFFFFF0` - `0xFFFFFFF3`.

Then, the `PC` will be initialize to the 32-bit value.  
Then `SP` will be reset to address 0x8FFFFFFF.
### Invalid opcode
If the CPU encounter an invalid instruction during execution, it stops execution completely.
## Memory layout
- 0x00000000 - 0x8FFFFFFF: General purpose RAM
- 0x8FFF0000 - 0x8FFFFFFF: Stack memory (Optional)
- 0x90000000 - 0x9000FFFF: MM/IO
- 0xFFFFFFF0 - 0xFFFFFFF3: Reset vector

## Devices MM/IO
- 0x10000000 - 0x100000FF: UART controller
- 0x10000100 - 0x100001FF: Keyboard controller
