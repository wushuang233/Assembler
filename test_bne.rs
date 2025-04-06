fn main() {
    // 常量定义
    const OPCODE_BNE: u32 = 0b000011;   // bne 如果 rs1 != rs2，则 pc += sext(offset)

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

    // 当前实现
    fn encode_bne_current(rs1: u8, rs2: u8, offset: i16) -> u32 {
        encode_c(OPCODE_BNE, rs1, rs2, offset)
    }
    
    // 修正的实现 - 交换rs1和rs2的顺序
    fn encode_bne_fixed(rs1: u8, rs2: u8, offset: i16) -> u32 {
        encode_c(OPCODE_BNE, rs2, rs1, offset)
    }

    // 实际例子中的bne指令：bne x3, x8, -28 和 bne x2, x1, -44
    // asm/sort.asm中：
    // bne x3, x8, -28
    let expected1 = 0b11111111111_00011_01000_00100_000011;
    let current1 = encode_bne_current(3, 8, -28);
    let fixed1 = encode_bne_fixed(3, 8, -28);

    // bne x2, x1, -44
    let expected2 = 0b11111111110_00010_00001_10100_000011;
    let current2 = encode_bne_current(2, 1, -44);
    let fixed2 = encode_bne_fixed(2, 1, -44);
    
    println!("===== 测试 bne x3, x8, -28 =====");
    println!("期望编码:    {:032b}", expected1);
    println!("当前实现:    {:032b}", current1);
    println!("修正后实现:  {:032b}", fixed1);
    println!("期望格式化:  0b{:011b}_{:05b}_{:05b}_{:05b}_{:06b}", 
        (expected1 >> 21) & 0x7FF,
        (expected1 >> 16) & 0x1F,
        (expected1 >> 11) & 0x1F,
        (expected1 >> 6) & 0x1F,
        expected1 & 0x3F);
    println!("当前格式化:  0b{:011b}_{:05b}_{:05b}_{:05b}_{:06b}", 
        (current1 >> 21) & 0x7FF,
        (current1 >> 16) & 0x1F,
        (current1 >> 11) & 0x1F,
        (current1 >> 6) & 0x1F,
        current1 & 0x3F);
    println!("修正格式化:  0b{:011b}_{:05b}_{:05b}_{:05b}_{:06b}", 
        (fixed1 >> 21) & 0x7FF,
        (fixed1 >> 16) & 0x1F,
        (fixed1 >> 11) & 0x1F,
        (fixed1 >> 6) & 0x1F,
        fixed1 & 0x3F);
        
    println!("\n===== 测试 bne x2, x1, -44 =====");
    println!("期望编码:    {:032b}", expected2);
    println!("当前实现:    {:032b}", current2);
    println!("修正后实现:  {:032b}", fixed2);
    println!("期望格式化:  0b{:011b}_{:05b}_{:05b}_{:05b}_{:06b}", 
        (expected2 >> 21) & 0x7FF,
        (expected2 >> 16) & 0x1F,
        (expected2 >> 11) & 0x1F,
        (expected2 >> 6) & 0x1F,
        expected2 & 0x3F);
    println!("当前格式化:  0b{:011b}_{:05b}_{:05b}_{:05b}_{:06b}", 
        (current2 >> 21) & 0x7FF,
        (current2 >> 16) & 0x1F,
        (current2 >> 11) & 0x1F,
        (current2 >> 6) & 0x1F,
        current2 & 0x3F);
    println!("修正格式化:  0b{:011b}_{:05b}_{:05b}_{:05b}_{:06b}", 
        (fixed2 >> 21) & 0x7FF,
        (fixed2 >> 16) & 0x1F,
        (fixed2 >> 11) & 0x1F,
        (fixed2 >> 6) & 0x1F,
        fixed2 & 0x3F);
    
    // 结果判断
    println!("\n===== 测试结果 =====");
    if current1 == expected1 && current2 == expected2 {
        println!("当前实现正确 ✓");
    } else {
        println!("当前实现错误 ✗");
    }
    
    if fixed1 == expected1 && fixed2 == expected2 {
        println!("修正实现正确 ✓");
    } else {
        println!("修正实现错误 ✗");
    }
    
    println!("\n===== 结论 =====");
    let is_current_correct = current1 == expected1 && current2 == expected2;
    let is_fixed_correct = fixed1 == expected1 && fixed2 == expected2;
    
    if is_current_correct && !is_fixed_correct {
        println!("bne指令当前实现是正确的，不需要修改");
    } else if !is_current_correct && is_fixed_correct {
        println!("bne指令需要修改，应该交换rs1和rs2的顺序");
    } else {
        println!("测试结果不明确，需要进一步检查");
    }
    
    println!("\n===== C类型指令格式说明 =====");
    println!("在当前实现的C类型指令编码函数encode_c中：");
    println!("- rs1位于[20:16]，rs2位于[15:11]");
    println!("- 但不同指令对rs1、rs2的解释不同");
    println!("- 对于sw：基址寄存器实际上应该放在[15:11]，源寄存器应该放在[20:16]");
    println!("- 对于blt：比较的顺序需要调整");
    println!("- 对于bne：需要验证正确性");
} 