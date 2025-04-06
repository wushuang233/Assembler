// A类型指令编码（add/mul）
// 格式: 前11位0_rs1[5位]_rs2[5位]_rd[5位]_opcode[6位]
fn encode_a(opcode: u32, rd: u8, rs1: u8, rs2: u8) -> u32 {
    // 前11位固定为0
    ((0u32) << 21) | 
    ((rs2 as u32 & 0x1F) << 16) | 
    ((rs1 as u32 & 0x1F) << 11) | 
    ((rd as u32 & 0x1F) << 6) | 
    (opcode & 0x3F)
}

// B类型指令编码（addi/lui/lw）
// 格式: imm[16位]_rs1[5位]_rd[5位]_opcode[6位]
fn encode_b(opcode: u32, rd: u8, rs1: u8, imm: i16) -> u32 {
    println!("B类型编码: opcode=0x{:X}, rd={}, rs1={}, imm=0x{:X} (二进制: {:016b})", 
             opcode, rd, rs1, imm, imm);
             
    // 保留立即数的所有16位
    let imm_u32 = imm as u32 & 0xFFFF;
    println!("imm_u32: 0x{:X} (二进制: {:016b})", imm_u32, imm_u32);
    
    // 构建指令
    let result = (imm_u32 << 16) |            // 16位立即数放在[31:16]
                 ((rs1 as u32 & 0x1F) << 11) | // rs1放在[15:11]
                 ((rd as u32 & 0x1F) << 6) |   // rd放在[10:6]
                 (opcode & 0x3F);              // opcode放在[5:0]
                 
    println!("最终编码结果: 0x{:X} (二进制: {:032b})", result, result);
    result
}

// B类型指令编码之addi的专用函数
fn encode_addi(rd: u8, rs1: u8, imm: i16) -> u32 {
    encode_b(0b000010u32, rd, rs1, imm)
}

// B类型指令编码之lui的专用函数
fn encode_lui(rd: u8, imm: i16) -> u32 {
    // 使用标准B类型编码，不需要特殊处理
    encode_b(0b000101u32, rd, 0, imm)
}

// B类型指令编码之lw的专用函数
fn encode_lw(rd: u8, rs1: u8, offset: i16) -> u32 {
    encode_b(0b000110u32, rd, rs1, offset)
}

// C类型指令编码（bne/sw/blt）
// 格式: imm_high[31:21] rs2[20:16] rs1[15:11] imm_low[10:6] opcode[5:0]
fn encode_c(opcode: u32, rs1: u8, rs2: u8, offset: i16) -> u32 {
    let offset_u32 = offset as u32;
    let imm_high = (offset_u32 >> 5) & 0x7FF;
    let imm_low = offset_u32 & 0x1F;
    
    (imm_high << 21) |
    ((rs2 as u32) << 16) |
    ((rs1 as u32) << 11) |
    (imm_low << 6) |
    (opcode & 0x3F)
}

// C类型指令编码之bne的专用函数
fn encode_bne(rs1: u8, rs2: u8, offset: i16) -> u32 {
    encode_c(0b000011u32, rs1, rs2, offset)
}

// C类型指令编码之sw的专用函数
fn encode_sw(rs1: u8, rs2: u8, offset: i16) -> u32 {
    encode_c(0b000111u32, rs1, rs2, offset)
}

// C类型指令编码之blt的专用函数
fn encode_blt(rs1: u8, rs2: u8, offset: i16) -> u32 {
    encode_c(0b001000u32, rs1, rs2, offset)
}


fn encode_halt() -> u32 {
    0u32
}

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
    
    #[test]
    fn test_lui() {
        // lui x2, 42 -> 0b00000000001_01010_00000_00010_000101
        let expected = 0b00000000001_01010_00000_00010_000101;
        let actual = encode_lui(2, 42);
        println!("Expected (lui): {:032b}", expected);
        println!("Actual   (lui): {:032b}", actual);
        assert_eq!(actual, expected);
    }
    
    #[test]
    fn test_lw() {
        // lw x3, 4(x1) -> 0b00000000000_00100_00001_00011_000110
        let expected = 0b00000000000_00100_00001_00011_000110;
        let actual = encode_lw(3, 1, 4);
        println!("Expected (lw): {:032b}", expected);
        println!("Actual   (lw): {:032b}", actual);
        assert_eq!(actual, expected);
    }
    
    #[test]
    fn test_sw() {
        // sw x2, 8(x1) -> 0b00000000000_00010_00001_01000_000111
        let expected = 0b00000000000_00010_00001_01000_000111;
        let actual = encode_sw(1, 2, 8);
        println!("Expected (sw): {:032b}", expected);
        println!("Actual   (sw): {:032b}", actual);
        assert_eq!(actual, expected);
    }
    
    #[test]
    fn test_blt() {
        // blt x4, x5, 16 -> 0b00000000000_00101_00100_10000_001000
        let expected = 0b00000000000_00101_00100_10000_001000;
        let actual = encode_blt(4, 5, 16);
        println!("Expected (blt): {:032b}", expected);
        println!("Actual   (blt): {:032b}", actual);
        assert_eq!(actual, expected);
    }
}

fn parse_reg(reg: &str) -> u8 {
    reg[1..].parse().unwrap_or_else(|_| panic!("Invalid register: {}", reg))
}

fn parse_imm(imm_str: &str) -> i16 {
    if imm_str.starts_with("0x") {
        // 处理十六进制值
        println!("解析十六进制立即数: {}", imm_str);
        let value = i16::from_str_radix(&imm_str[2..], 16).unwrap_or_else(|_| {
            panic!("无效的十六进制立即数: {}", imm_str);
        });
        println!("解析结果: {}, 二进制表示: {:016b}", value, value);
        value
    } else {
        // 处理十进制值
        imm_str.parse().unwrap_or_else(|_| {
            panic!("无效的十进制立即数: {}", imm_str);
        })
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
            "lui" => {
                let rd = parse_reg(parts[1].trim_end_matches(','));
                let imm = parse_imm(parts[2]);
                
                println!("处理lui指令: rd={}, imm={}", rd, imm);
                let encoded = encode_lui(rd, imm);
                println!("编码结果: 0b{:032b}", encoded);
                img.push(encoded);
            }
            "lw" => {
                // 处理格式如 lw x1, 4(x2) 的指令
                let rd = parse_reg(parts[1].trim_end_matches(','));
                
                // 解析 4(x2) 格式
                let offset_reg = parts[2];
                let open_paren = offset_reg.find('(').unwrap_or_else(|| panic!("Invalid lw format: {}", offset_reg));
                let close_paren = offset_reg.find(')').unwrap_or_else(|| panic!("Invalid lw format: {}", offset_reg));
                
                let offset = parse_imm(&offset_reg[0..open_paren]);
                let rs1 = parse_reg(&offset_reg[open_paren+1..close_paren]);
                
                println!("处理lw指令: rd={}, rs1={}, offset={}", rd, rs1, offset);
                let encoded = encode_lw(rd, rs1, offset);
                println!("编码结果: 0b{:032b}", encoded);
                img.push(encoded);
            }
            "sw" => {
                // 处理格式如 sw x1, 4(x2) 的指令
                let rs2 = parse_reg(parts[1].trim_end_matches(','));
                
                // 解析 4(x2) 格式
                let offset_reg = parts[2];
                let open_paren = offset_reg.find('(').unwrap_or_else(|| panic!("Invalid sw format: {}", offset_reg));
                let close_paren = offset_reg.find(')').unwrap_or_else(|| panic!("Invalid sw format: {}", offset_reg));
                
                let offset = parse_imm(&offset_reg[0..open_paren]);
                let rs1 = parse_reg(&offset_reg[open_paren+1..close_paren]);
                
                println!("处理sw指令: rs1={}, rs2={}, offset={}", rs1, rs2, offset);
                let encoded = encode_sw(rs1, rs2, offset);
                println!("编码结果: 0b{:032b}", encoded);
                img.push(encoded);
            }
            "blt" => {
                let rs1 = parse_reg(parts[1].trim_end_matches(','));
                let rs2 = parse_reg(parts[2].trim_end_matches(','));
                let offset = parse_imm(parts[3]);
                
                println!("处理blt指令: rs1={}, rs2={}, offset={}", rs1, rs2, offset);
                let encoded = encode_blt(rs1, rs2, offset);
                println!("编码结果: 0b{:032b}", encoded);
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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        println!("用法: {} <asm文件名>", args[0]);
        println!("示例: {} sum - 将编译asm/sum.asm，输出到out/sum.o和out/sum.txt", args[0]);
        return;
    }
    
    let base_name = &args[1];
    let input_file = format!("asm/{}.asm", base_name);
    let output_binary = format!("out/{}.o", base_name);
    let output_text = format!("out/{}.txt", base_name);
    
    if let Err(e) = std::fs::create_dir_all("out") {
        eprintln!("无法创建输出目录: {}", e);
        return;
    }
    
    let asm_code = match std::fs::read_to_string(&input_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("无法读取文件 {}: {}", input_file, e);
            return;
        }
    };

    let img = assemble(&asm_code);
    
    let mut text_output = String::new();
    
    println!("生成的指令（二进制格式）:");
    for (i, &instr) in img.iter().enumerate() {
        let binary_str = format!("{:032b}", instr);
        let formatted_binary = format!("0b{}_{}_{}_{}_{}", 
            &binary_str[0..11], 
            &binary_str[11..16], 
            &binary_str[16..21], 
            &binary_str[21..26], 
            &binary_str[26..32]);
            
        println!("指令 {}: {} (十六进制: 0x{:08X})", i+1, formatted_binary, instr);
        
        text_output.push_str(&format!("{}\n", formatted_binary));
    }
    
    match write_object_file(&img, &output_binary) {
        Ok(_) => println!("生成二进制文件: {}", output_binary),
        Err(e) => {
            eprintln!("写入文件时出错: {}", e);
            return;
        }
    }
    
    match std::fs::write(&output_text, text_output) {
        Ok(_) => println!("生成文本文件: {}", output_text),
        Err(e) => eprintln!("写入文本文件时出错: {}", e),
    }
}

fn write_object_file(img: &[u32], path: &str) -> std::io::Result<()> {
    let mut buf = Vec::with_capacity(img.len() * 4);
    for &word in img {
        buf.extend(word.to_le_bytes());
    }
    std::fs::write(path, buf)?;
    Ok(())
}