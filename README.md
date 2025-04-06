# 简易RISC-V汇编编译器

这是一个简易的RISC-V汇编编译器，可以将基本的RISC-V汇编指令编译为二进制机器码。

## 功能概述

该编译器支持以下指令：
- `add` - 加法操作
- `addi` - 立即数加法
- `mul` - 乘法操作
- `bne` - 条件分支（不相等时跳转）
- `halt` - 停止执行

## 目录结构

```
.
├── src/           # 源代码目录
│   └── main.rs    # 主程序代码
├── asm/           # 汇编源文件目录
│   ├── sum.asm            # 计算1到10的和的汇编程序
│   ├── factorial.asm      # 计算10的阶乘的汇编程序
│   ├── fibonacci.asm      # 计算斐波那契数列的汇编程序
│   ├── sum_expected.txt       # sum.asm的预期输出
│   ├── factorial_expected.txt # factorial.asm的预期输出
│   └── fibonacci_expected.txt # fibonacci.asm的预期输出
├── out/           # 输出文件目录（程序运行时自动创建）
│   ├── *.o        # 二进制机器码文件（小端序）
│   └── *.txt      # 可读的二进制表示文件
└── target/        # 编译输出目录
    └── debug/     # 调试版本可执行文件
```

## 编译与运行

### 编译项目

```bash
cargo build
```

### 运行编译器

基本用法：

```bash
.\target\debug\huibian.exe <汇编文件名>
```

示例：

```bash
# 编译sum.asm文件
.\target\debug\huibian.exe sum

# 编译factorial.asm文件
.\target\debug\huibian.exe factorial

# 编译fibonacci.asm文件
.\target\debug\huibian.exe fibonacci
```

注意：输入的文件名不需要包含路径和扩展名，程序会自动从`asm/`目录读取对应的`.asm`文件。

## 输入与输出

### 输入文件

输入文件应放在`asm/`目录下，使用`.asm`扩展名。例如：`asm/sum.asm`。

汇编文件可使用的指令格式如下：

```
# 寄存器加法
add rd, rs1, rs2    # rd = rs1 + rs2

# 立即数加法
addi rd, rs1, imm   # rd = rs1 + imm

# 寄存器乘法
mul rd, rs1, rs2    # rd = rs1 * rs2

# 条件分支（不相等时跳转）
bne rs1, rs2, offset # 如果rs1 != rs2，PC += offset

# 停止执行
halt
```

### 输出文件

程序会在`out/`目录下生成两种输出文件：

1. **二进制机器码文件** (.o)：
   - 文件名格式：`out/<输入文件名>.o`
   - 内容：纯二进制指令数据，每条指令4字节，使用小端序存储
   - 不包含任何元数据（如魔数、指令数量等）

2. **可读的二进制表示文件** (.txt)：
   - 文件名格式：`out/<输入文件名>.txt`
   - 内容：每条指令的二进制表示，每行一条指令
   - 格式：`0b00000000000_00000_00000_00000_000000`

## 指令编码格式

所有指令都编码为32位字长格式：

### A类型指令（add/mul）
- 格式：前11位0_rs2[5位]_rs1[5位]_rd[5位]_opcode[6位]
- 示例：`add x1, x1, x3` → `0b00000000000_00011_00001_00001_000001`
- 注意：rs1和rs2在指令中的位置看起来是相反的

### B类型指令（addi）
- 格式：imm[11位]_rs1[5位]_rd[5位]_opcode[6位]
- 示例：`addi x1, x0, 1` → `0b00000000000_00001_00000_00001_000010`

### C类型指令（bne）
- 格式：offset[11位]_rs2[5位]_rs1[5位]_offset_low[5位]_opcode[6位]
- 示例：`bne x3, x2, -8` → `0b11111111111_00010_00011_11000_000011`

### D类型指令（halt）
- 格式：全0
- 示例：`halt` → `0b00000000000_00000_00000_00000_000000`

## 示例程序

### 1. 计算1到10的和 (sum.asm)

```asm
# 初始化
addi x1, x0, 0    # 结果寄存器初始化为0
addi x2, x0, 10   # 目标值为10
addi x3, x0, 0    # 计数器初始化为0

# 循环
addi x3, x3, 1    # 计数器加1
add x1, x1, x3    # 累加当前计数值
bne x3, x2, -8    # 如果计数器不等于10，跳回
halt              # 程序结束
```

### 2. 计算10的阶乘 (factorial.asm)

```asm
# 初始化
addi x1, x0, 1    # 结果寄存器初始化为1
addi x2, x0, 10   # 目标值为10
addi x3, x0, 0    # 计数器初始化为0

# 循环
addi x3, x3, 1    # 计数器加1
mul x1, x1, x3    # 累乘当前计数值
bne x3, x2, -8    # 如果计数器不等于10，跳回
halt              # 程序结束
```

### 3. 计算斐波那契数列 (fibonacci.asm)

```asm
# 初始化
addi x1, x0, 1    # 第一个斐波那契数
addi x2, x0, 1    # 第二个斐波那契数
addi x4, x0, 2    # 计数器（已计算2个数）
addi x5, x0, 7    # 目标是计算7个数

# 循环
add x3, x1, x2    # 计算下一个斐波那契数
add x1, x0, x2    # 更新x1为x2
add x2, x0, x3    # 更新x2为x3
addi x4, x4, 1    # 计数器加1
bne x4, x5, -16   # 如果计数器不等于7，跳回
halt              # 程序结束
```

## 查看输出

### 查看文本格式输出

```bash
type out\sum.txt
type out\factorial.txt
type out\fibonacci.txt
```

### 查看二进制输出（PowerShell）

```powershell
# 以十六进制查看二进制文件内容
$bytes = [System.IO.File]::ReadAllBytes('.\out\sum.o')
for ($i = 0; $i -lt $bytes.Length; $i += 4) {
    [BitConverter]::ToUInt32($bytes, $i).ToString('X8')
}
``` 