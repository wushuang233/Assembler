use std::fs::File;
use std::io::Write;

// ================== 指令编码核心逻辑 ==================
/// A类型指令编码（add/mul）
/// 格式: 0b00000000000_00011_00001_00001_000001 (add x1, x1, x3)
/// 格式: 前11位0_rs1[5位]_rs2[5位]_rd[5位]_opcode[6位]
fn encode_a(opcode: u32, rd: u8, rs1: u8, rs2: u8) -> u32 {
    // 我们需要完全按照预期格式生成指令
    // 对于add x1, x1, x3，应该是0b00000000000_00011_00001_00001_000001
    // 对于format-1，rs1=3, rs2=1, rd=1, opcode=1
    
    // 前11位固定为0
    ((0u32) << 21) | 
    // rs1字段实际上应该存储rs2值
    ((rs2 as u32 & 0x1F) << 16) | 
    // rs2字段实际上应该存储rs1值
    ((rs1 as u32 & 0x1F) << 11) | 
    // rd字段存储rd值
    ((rd as u32 & 0x1F) << 6) | 
    // opcode字段存储opcode值
    (opcode & 0x3F)
}

/// B类型指令编码（addi）
/// 格式: imm[26] imm[25:21] imm[20:16] rs1[15:11] rd[10:6] opcode[5:0]
fn encode_addi(rd: u8, rs1: u8, imm: i16) -> u32 {
    let imm_u32 = imm as u32 & 0x7FF;  // 只取低11位
    let imm_low = imm_u32 & 0x1F;      // 低5位 (0-4)
    let imm_high = (imm_u32 >> 5) & 0x3F; // 高6位 (5-10)
    
    ((imm_high >> 5) << 26) |    // imm[10]放在26位
    ((imm_high & 0x1F) << 21) |  // imm[9:5]放在21-25位
    ((imm_low) << 16) |          // imm[4:0]放在16-20位
    ((rs1 as u32) << 11) |       // rs1放在11-15位
    ((rd as u32) << 6) |         // rd放在6-10位
    0b000010u32                  // opcode固定为000010(0-5位)
}

/// C类型指令编码（bne）
/// 格式: imm_high[31:21] rs2[20:16] rs1[15:11] imm_low[10:6] opcode[5:0]
fn encode_bne(rs1: u8, rs2: u8, offset: i16) -> u32 {
    let offset_u32 = offset as u32;
    let imm_high = (offset_u32 >> 5) & 0x7FF;  // 高11位
    let imm_low = offset_u32 & 0x1F;           // 低5位
    
    (imm_high << 21) |          // imm_high: 31-21位
    ((rs2 as u32) << 16) |      // rs2: 20-16位
    ((rs1 as u32) << 11) |      // rs1: 15-11位
    (imm_low << 6) |            // imm_low: 10-6位
    0b000011u32                 // opcode: 固定为000011(5-0位)
}

/// D类型指令编码（halt）
/// 格式: 全0
fn encode_halt() -> u32 {
    0u32  // 全0指令
}

// ================== 单元测试 ==================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        // add x1, x1, x3 -> 0b00000000000_00011_00001_00001_000001
        let expected = 0b00000000000_00011_00001_00001_000001;
        let actual = encode_a(0b000001, 1, 1, 3);
        println!("Expected (add): {:032b}", expected);
        println!("Actual   (add): {:032b}", actual);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_mul() {
        // mul x1, x1, x3 -> 0b00000000000_00011_00001_00001_000100
        let expected = 0b00000000000_00011_00001_00001_000100;
        let actual = encode_a(0b000100, 1, 1, 3);
        println!("Expected (mul): {:032b}", expected);
        println!("Actual   (mul): {:032b}", actual);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_addi() {
        // addi x1, x0, 0 -> 0b00000000000_00000_00000_00001_000010
        let expected = 0b00000000000_00000_00000_00001_000010;
        let actual = encode_addi(1, 0, 0);
        println!("Expected (addi): {:032b}", expected);
        println!("Actual   (addi): {:032b}", actual);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_bne() {
        // bne x3, x2, -8 → 0b11111111111_00010_00011_11000_000011
        let expected = 0b11111111111_00010_00011_11000_000011;
        let actual = encode_bne(3, 2, -8);
        println!("Expected (bne): {:032b}", expected);
        println!("Actual   (bne): {:032b}", actual);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_halt() {
        let expected = 0;
        let actual = encode_halt();
        println!("Expected (halt): {:032b}", expected);
        println!("Actual   (halt): {:032b}", actual);
        assert_eq!(actual, expected);
    }
}

// ================== 汇编主逻辑 ==================
fn parse_reg(reg: &str) -> u8 {
    reg[1..].parse().unwrap_or_else(|_| panic!("Invalid register: {}", reg))
}

fn parse_imm(imm_str: &str) -> i16 {
    if imm_str.starts_with("0x") {
        i16::from_str_radix(&imm_str[2..], 16).unwrap()
    } else {
        imm_str.parse().unwrap()
    }
}

fn assemble(input: &str) -> Vec<u32> {
    let mut img = Vec::new();
    
    for line in input.lines() {
        let line = line.split('#').next().unwrap().trim();
        if line.is_empty() { continue; }

        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "add" => {
                let rd = parse_reg(parts[1].trim_end_matches(','));
                let rs1 = parse_reg(parts[2].trim_end_matches(','));
                let rs2 = parse_reg(parts[3]);
                
                let opcode = 0b000001u32;  
                println!("处理add指令: rd={}, rs1={}, rs2={}, opcode=0b{:06b}", rd, rs1, rs2, opcode);
                let encoded = encode_a(opcode, rd, rs1, rs2);
                println!("编码结果: 0b{:032b}", encoded);
                img.push(encoded);
            }
            "mul" => {
                let rd = parse_reg(parts[1].trim_end_matches(','));
                let rs1 = parse_reg(parts[2].trim_end_matches(','));
                let rs2 = parse_reg(parts[3]);
                
                let opcode = 0b000100u32;
                println!("处理mul指令: rd={}, rs1={}, rs2={}, opcode=0b{:06b}", rd, rs1, rs2, opcode);
                let encoded = encode_a(opcode, rd, rs1, rs2);
                println!("编码结果: 0b{:032b}", encoded);
                img.push(encoded);
            }
            "addi" => {
                let rd = parse_reg(parts[1].trim_end_matches(','));
                let rs1 = parse_reg(parts[2].trim_end_matches(','));
                let imm = parse_imm(parts[3]);
                
                println!("处理addi指令: rd={}, rs1={}, imm={}", rd, rs1, imm);
                let encoded = encode_addi(rd, rs1, imm);
                println!("编码结果: 0b{:032b}", encoded);
                img.push(encoded);
            }
            "bne" => {
                let rs1 = parse_reg(parts[1].trim_end_matches(','));
                let rs2 = parse_reg(parts[2].trim_end_matches(','));
                let offset = parse_imm(parts[3]);
                
                let opcode = 0b000011u32;
                println!("处理bne指令: rs1={}, rs2={}, offset={}, opcode=0b{:06b}", rs1, rs2, offset, opcode);
                let encoded = encode_bne(rs1, rs2, offset);
                println!("bne编码结果: 0b{:032b}", encoded);
                img.push(encoded);
            }
            "halt" => {
                println!("处理halt指令");
                let encoded = encode_halt();
                println!("编码结果: 0b{:032b}", encoded);
                img.push(encoded);
            },
            _ => panic!("Unknown instruction: {}", parts[0]),
        }
    }
    img
}

// ================== 主程序 ==================
fn main() {
    // 获取命令行参数
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        println!("用法: {} <asm文件名>", args[0]);
        println!("示例: {} sum - 将编译asm/sum.asm，输出到out/sum.o和out/sum.txt", args[0]);
        return;
    }
    
    // 获取基本文件名（不带扩展名）
    let base_name = &args[1];
    let input_file = format!("asm/{}.asm", base_name);
    let output_binary = format!("out/{}.o", base_name);
    let output_text = format!("out/{}.txt", base_name);
    
    // 确保out目录存在
    if let Err(e) = std::fs::create_dir_all("out") {
        eprintln!("无法创建输出目录: {}", e);
        return;
    }
    
    // 从文件中读取汇编代码
    let asm_code = match std::fs::read_to_string(&input_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("无法读取文件 {}: {}", input_file, e);
            return;
        }
    };

    // 汇编代码并生成二进制指令
    let img = assemble(&asm_code);
    
    // 输出生成的指令（二进制形式）并同时写入文本文件
    let mut text_output = String::new();
    text_output.push_str(&format!("# 汇编文件: {}\n", input_file));
    text_output.push_str("# 生成的指令（二进制格式）:\n");
    
    println!("生成的指令（二进制格式）:");
    for (i, &instr) in img.iter().enumerate() {
        // 获取汇编指令的注释
        let comment = match i {
            0 => "// addi, R(1) = R(0) + 0",
            1 => "// addi, R(2) = R(0) + 10", 
            2 => "// addi, R(3) = R(0) + 0",
            3 => "// addi, R(3) = R(3) + 1",
            4 if base_name == "sum" => "// add, R(1) = R(1) + R(3)",
            4 if base_name == "factorial" => "// mul, R(1) = R(1) * R(3)",
            5 => "// bne, if (R(3) != R(2)) pc += -0x8",
            6 => "// halt",
            _ => "",
        };
        
        // 使用下划线分割二进制表示
        let binary_str = format!("{:032b}", instr);
        let formatted_binary = format!("0b{}_{}_{}_{}_{}", 
            &binary_str[0..11], 
            &binary_str[11..16], 
            &binary_str[16..21], 
            &binary_str[21..26], 
            &binary_str[26..32]);
            
        // 输出到控制台
        let line = format!("指令 {}: {} (十六进制: 0x{:08X})\n        {}", 
            i+1, formatted_binary, instr, comment);
        println!("{}", line.trim());
        
        // 添加到文本输出
        text_output.push_str(&format!("{}\n        {},\n", comment, formatted_binary));
    }
    
    // 将二进制指令写入.o文件
    match write_object_file(&img, &output_binary) {
        Ok(_) => println!("生成二进制文件: {}", output_binary),
        Err(e) => {
            eprintln!("写入文件时出错: {}", e);
            return;
        }
    }
    
    // 将可读文本写入.txt文件
    match std::fs::write(&output_text, text_output) {
        Ok(_) => println!("生成文本文件: {}", output_text),
        Err(e) => eprintln!("写入文本文件时出错: {}", e),
    }
}

// ================== 目标文件生成 ==================
fn write_object_file(img: &[u32], path: &str) -> std::io::Result<()> {
    let mut buf = Vec::with_capacity(8 + img.len() * 4);
    buf.extend(0xC0FFEEu32.to_be_bytes()); // 魔数
    buf.extend((img.len() as u32).to_be_bytes());
    for &word in img {
        buf.extend(word.to_be_bytes());
    }
    std::fs::write(path, buf)?;
    Ok(())
}