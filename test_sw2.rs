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

    // 当前实现
    fn encode_sw_current(rs1: u8, rs2: u8, offset: i16) -> u32 {
        encode_c(OPCODE_SW, rs1, rs2, offset)
    }
    
    // 修正的实现 - 交换rs1和rs2的顺序
    fn encode_sw_fixed(rs1: u8, rs2: u8, offset: i16) -> u32 {
        encode_c(OPCODE_SW, rs2, rs1, offset)
    }

    // 例1: sw x6, 0(x4)
    let expected1 = 0b00000000000_00110_00100_00000_000111;
    let current1 = encode_sw_current(4, 6, 0);
    let fixed1 = encode_sw_fixed(4, 6, 0);
    
    // 例2: sw x5, +4(x4)
    let expected2 = 0b00000000000_00101_00100_00100_000111;
    let current2 = encode_sw_current(4, 5, 4);
    let fixed2 = encode_sw_fixed(4, 5, 4);
    
    println!("===== 测试 sw x6, 0(x4) =====");
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
        
    println!("\n===== 测试 sw x5, +4(x4) =====");
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
    
    // 结论
    println!("\n===== 结论 =====");
    println!("问题：当前sw编码函数出现rs1和rs2位置错误");
    println!("说明：");
    println!("1. 期望格式应该是 rs2[20:16], rs1[15:11]，但当前实现是 rs1[20:16], rs2[15:11]");
    println!("2. 要得到正确编码，需要修改encode_sw函数，交换rs1和rs2的顺序:");
    println!("   encode_c(OPCODE_SW, rs2, rs1, offset)");
    
    println!("\n3. C型指令中编码方式不统一，需要根据指令类型分别处理:");
    println!("   - sw指令: rs1和rs2需要交换 (encode_c(OPCODE_SW, rs2, rs1, offset))");
    println!("   - blt指令: rs1和rs2需要交换 (encode_c(OPCODE_BLT, rs2, rs1, offset))");
    println!("   - bne指令: 目前的测试表明不需要交换");
} 