# RISC-V 汇编器与反汇编器

## 项目介绍

这是一个简易的 RISC-V 指令集汇编器和反汇编器实现，支持基本的指令编码和解码功能。本项目实现了指令的编码、汇编、反汇编以及二进制文件的读写操作。

## 支持的指令

本汇编器/反汇编器支持以下 RISC-V 指令：

| 指令 | 操作码 | 格式类型 | 功能描述 |
|------|--------|----------|----------|
| `halt` | `000000` | - | 停止执行 |
| `add` | `000001` | A类型 | `x[rd] = x[rs1] + x[rs2]` |
| `addi` | `000010` | B类型 | `x[rd] = x[rs1] + sext(imm)` |
| `bne` | `000011` | C类型 | 如果 `rs1 != rs2`，则 `pc += sext(offset)` |
| `mul` | `000100` | A类型 | `x[rd] = x[rs1] * x[rs2]` |
| `lui` | `000101` | B类型 | `x[rd] = sext(imm) << 16` |
| `lw` | `000110` | B类型 | `x[rd] = M[x[rs1] + sext(imm)]` |
| `sw` | `000111` | C类型 | `M[x[rs1] + sext(imm)] = x[rs2]` |
| `blt` | `001000` | C类型 | 如果 `rs1 <s rs2`，则 `pc += sext(offset)` |
| `slli` | `001001` | B类型 | `x[rd] = x[rs1] << imm` |
| `sub` | `001010` | A类型 | `x[rd] = x[rs1] - x[rs2]` |

## 指令格式

项目支持三种指令类型：

### A 类型指令 (add/mul/sub)
格式: 前11位0_rs2[5位]_rs1[5位]_rd[5位]_opcode[6位]

### B 类型指令 (addi/lui/lw/slli)
格式: imm[16位]_rs1[5位]_rd[5位]_opcode[6位]

### C 类型指令 (bne/sw/blt)
格式: imm_high[31:21]_rs1[20:16]_rs2[15:11]_imm_low[10:6]_opcode[5:0]

**注意**：C 类型指令中，rs1 和 rs2 的位置处理在不同指令间有所不同：
- `bne` 指令：rs1 位于 [20:16]，rs2 位于 [15:11]
- `sw` 指令：源寄存器位于 [20:16]，基址寄存器位于 [15:11]
- `blt` 指令：比较顺序需要调整，第二个寄存器位于 [20:16]，第一个寄存器位于 [15:11]

## 使用方法

### 汇编功能
将汇编代码文件转换为二进制文件：
```
cargo run asm <汇编文件名>
```
例如：`cargo run asm sum` 将编译 `asm/sum.asm`，并输出到 `out/sum.o` 和 `out/sum.txt`

### 反汇编功能
将二进制文件反汇编为汇编代码：
```
cargo run disasm <二进制文件> <输出文件>
```
例如：`cargo run disasm out/sum.o out/sum_disasm.asm`

## 文件结构
- `src/main.rs`: 主程序源代码
- `asm/`: 存放汇编源文件的目录
- `out/`: 编译和反汇编输出目录

## 实现细节

### 编码与解码
- `encode_a()`: 对 A 类型指令进行编码
- `encode_b()`: 对 B 类型指令进行编码
- `encode_c()`: 对 C 类型指令进行编码
- `decode_a_type()`: 解码 A 类型指令
- `decode_b_type()`: 解码 B 类型指令
- `decode_c_type()`: 解码 C 类型指令

### 文件操作
- `write_object_file()`: 将指令写入二进制文件
- `read_binary_file()`: 从二进制文件读取指令

### 汇编与反汇编
- `assemble()`: 将汇编代码转换为机器码
- `decode_instruction()`: 将机器码转换为汇编代码

## 测试
项目包含多个单元测试，测试每个指令的编码和解码是否正确。可以通过以下命令运行测试：
```
cargo test
```

## 注意事项
在汇编过程中，寄存器参数和立即数的处理需要特别注意：
- 寄存器以 "x" 开头，后跟编号，如 `x0`, `x1`
- 立即数支持十进制和十六进制（0x前缀）
- 内存访问指令（如 `lw` 和 `sw`）使用 `offset(reg)` 格式，例如 `lw x1, 4(x2)` 