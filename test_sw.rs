fn main() {
    // 常量定义
    const OPCODE_SW: u32 = 0b000111;   // sw M[x[rs1] + sext(imm)] = x[rs2]

    // C类型指令编码（bne/sw/blt）
    // 格式: imm_high[31:21] rs1[20:16] rs2[15:11] imm_low[10:6] opcode[5:0]
    fn encode_c(opcode: u32, rs1: u8, rs2: u8, offset: i16) -> u32 {
        // 处理有符号扩展
        let offset_u32 = offset as u32;
        // 提取高11位和低5位
        let imm_high = (offset_u32 >> 5) & 0x7FF;
        let imm_low = offset_u32 & 0x1F;
        
        (imm_high << 21) |
        ((rs1 as u32) << 16) |  // rs1放在[20:16]
        ((rs2 as u32) << 11) |  // rs2放在[15:11]
        (imm_low << 6) |
        (opcode & 0x3F)
    }

    // 原始实现 - 需要注意测试中实际参数顺序
    fn encode_sw(rs1: u8, rs2: u8, offset: i16) -> u32 {
        encode_c(OPCODE_SW, rs1, rs2, offset)
    }

    // 测试 sw x6, 0(x4) - 这里参数顺序是encode_sw(rs1, rs2, offset)
    // 即对于 sw x6, 0(x4)，我们调用encode_sw(4, 6, 0)
    let expected1 = 0b00000000000_00100_00110_00000_000111;
    let result1 = encode_sw(4, 6, 0);

    // 测试 sw x5, +4(x4)
    let expected2 = 0b00000000000_00100_00101_00100_000111;
    let result2 = encode_sw(4, 5, 4);
    
    println!("==== 测试 sw x6, 0(x4) ====");
    println!("期望结果:    {:032b}", expected1);
    println!("实际结果:    {:032b}", result1);
    println!("期望格式化:  0b{:011b}_{:05b}_{:05b}_{:05b}_{:06b}", 
        (expected1 >> 21) & 0x7FF,
        (expected1 >> 16) & 0x1F,
        (expected1 >> 11) & 0x1F,
        (expected1 >> 6) & 0x1F,
        expected1 & 0x3F);
    println!("实际格式化:  0b{:011b}_{:05b}_{:05b}_{:05b}_{:06b}", 
        (result1 >> 21) & 0x7FF,
        (result1 >> 16) & 0x1F,
        (result1 >> 11) & 0x1F,
        (result1 >> 6) & 0x1F,
        result1 & 0x3F);
        
    println!("\n==== 测试 sw x5, +4(x4) ====");
    println!("期望结果:    {:032b}", expected2);
    println!("实际结果:    {:032b}", result2);
    println!("期望格式化:  0b{:011b}_{:05b}_{:05b}_{:05b}_{:06b}", 
        (expected2 >> 21) & 0x7FF,
        (expected2 >> 16) & 0x1F,
        (expected2 >> 11) & 0x1F,
        (expected2 >> 6) & 0x1F,
        expected2 & 0x3F);
    println!("实际格式化:  0b{:011b}_{:05b}_{:05b}_{:05b}_{:06b}", 
        (result2 >> 21) & 0x7FF,
        (result2 >> 16) & 0x1F,
        (result2 >> 11) & 0x1F,
        (result2 >> 6) & 0x1F,
        result2 & 0x3F);
    
    // 结果判断
    println!("\n结论:");
    if result1 == expected1 && result2 == expected2 {
        println!("✓ encode_sw函数正确，不需要修改");
    } else {
        println!("✗ encode_sw函数有问题，需要修改");
    }
    
    println!("\n注意事项:");
    println!("1. C类型指令的格式为: imm_high[31:21] | rs1[20:16] | rs2[15:11] | imm_low[10:6] | opcode[5:0]");
    println!("2. 对于sw指令，例如'sw x6, 0(x4)':");
    println!("   - x4是基址寄存器(rs1)，放在[20:16]位置");
    println!("   - x6是源寄存器(rs2)，放在[15:11]位置");
    println!("3. 当前函数encode_sw(rs1, rs2, offset)已经正确实现");
} 