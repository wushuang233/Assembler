fn main() {
    // 常量定义
    const OPCODE_BLT: u32 = 0b001000;   // blt 如果 rs1 <s rs2，则 pc += sext(offset)

    // C类型指令编码（bne/sw/blt）
    // 注意：在这里rs1和rs2的位置需要修正
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

    // 正确的实现，需要交换rs1和rs2的位置
    fn encode_blt_fixed(rs1: u8, rs2: u8, offset: i16) -> u32 {
        // 这里rs1和rs2需要交换，才能使结果符合期望
        encode_c(OPCODE_BLT, rs1, rs2, offset)
    }

    // 原始实现
    fn encode_blt(rs1: u8, rs2: u8, offset: i16) -> u32 {
        encode_c(OPCODE_BLT, rs1, rs2, offset)
    }

    // 测试 blt x5, x6, +12
    let expected = 0b00000000000_00110_00101_01100_001000;
    let result = encode_blt(5, 6, 12);
    let fixed_result = encode_blt_fixed(6, 5, 12); // 交换rs1和rs2参数顺序
    
    println!("Expected:     {:032b}", expected);
    println!("Result:       {:032b}", result);
    println!("Fixed Result: {:032b}", fixed_result);
    
    println!("Expected formatted: 0b{:011b}_{:05b}_{:05b}_{:05b}_{:06b}", 
        (expected >> 21) & 0x7FF,
        (expected >> 16) & 0x1F,
        (expected >> 11) & 0x1F,
        (expected >> 6) & 0x1F,
        expected & 0x3F);
    println!("Result formatted:   0b{:011b}_{:05b}_{:05b}_{:05b}_{:06b}", 
        (result >> 21) & 0x7FF,
        (result >> 16) & 0x1F,
        (result >> 11) & 0x1F,
        (result >> 6) & 0x1F,
        result & 0x3F);
    println!("Fixed formatted:    0b{:011b}_{:05b}_{:05b}_{:05b}_{:06b}", 
        (fixed_result >> 21) & 0x7FF,
        (fixed_result >> 16) & 0x1F,
        (fixed_result >> 11) & 0x1F,
        (fixed_result >> 6) & 0x1F,
        fixed_result & 0x3F);
    
    if result == expected {
        println!("原始编码测试通过!");
    } else {
        println!("原始编码测试失败!");
    }
    
    if fixed_result == expected {
        println!("修正后的编码测试通过!");
    } else {
        println!("修正后的编码测试失败!");
    }
    
    // 解释问题所在
    println!("\n问题分析:");
    println!("在原始实现中，期望blt x5, x6, +12的编码是: 0b00000000000_00110_00101_01100_001000");
    println!("但实际编码是:                            0b00000000000_00101_00110_01100_001000");
    println!("原因是rs1和rs2的位置被互换了，要获得正确的结果，应该使用blt x6, x5, +12");
    println!("或者修改编码函数，在encode_blt中互换rs1和rs2的参数顺序。");
} 