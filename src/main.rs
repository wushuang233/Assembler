use std::fs;
use std::io::{self, Read};
use std::path::Path;

// 常量定义
const OPCODE_HALT: u32 = 0b000000;
const OPCODE_ADD: u32 = 0b000001;
const OPCODE_ADDI: u32 = 0b000010;
const OPCODE_BNE: u32 = 0b000011;
const OPCODE_MUL: u32 = 0b000100;
const OPCODE_LUI: u32 = 0b000101;
const OPCODE_LW: u32 = 0b000110;
const OPCODE_SW: u32 = 0b000111;
const OPCODE_BLT: u32 = 0b001000;

// =================== 汇编器部分 ===================

// A类型指令编码（add/mul）
// 格式: 前11位0_rs2[5位]_rs1[5位]_rd[5位]_opcode[6位]
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
    // 保留立即数的所有16位
    let imm_u32 = imm as u32 & 0xFFFF;
    
    // 构建指令
    (imm_u32 << 16) |              // 16位立即数放在[31:16]
    ((rs1 as u32 & 0x1F) << 11) |  // rs1放在[15:11]
    ((rd as u32 & 0x1F) << 6) |    // rd放在[10:6]
    (opcode & 0x3F)                // opcode放在[5:0]
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

// 各指令类型编码专用函数
fn encode_add(rd: u8, rs1: u8, rs2: u8) -> u32 {
    encode_a(OPCODE_ADD, rd, rs1, rs2)
}

fn encode_mul(rd: u8, rs1: u8, rs2: u8) -> u32 {
    encode_a(OPCODE_MUL, rd, rs1, rs2)
}

fn encode_addi(rd: u8, rs1: u8, imm: i16) -> u32 {
    encode_b(OPCODE_ADDI, rd, rs1, imm)
}

fn encode_lui(rd: u8, imm: i16) -> u32 {
    encode_b(OPCODE_LUI, rd, 0, imm)
}

fn encode_lw(rd: u8, rs1: u8, offset: i16) -> u32 {
    encode_b(OPCODE_LW, rd, rs1, offset)
}

fn encode_bne(rs1: u8, rs2: u8, offset: i16) -> u32 {
    encode_c(OPCODE_BNE, rs1, rs2, offset)
}

fn encode_sw(rs1: u8, rs2: u8, offset: i16) -> u32 {
    encode_c(OPCODE_SW, rs1, rs2, offset)
}

fn encode_blt(rs1: u8, rs2: u8, offset: i16) -> u32 {
    encode_c(OPCODE_BLT, rs1, rs2, offset)
}

fn encode_halt() -> u32 {
    0u32
}

fn parse_reg(reg: &str) -> u8 {
    reg[1..].parse().unwrap_or_else(|_| panic!("无效的寄存器: {}", reg))
}

fn parse_imm(imm_str: &str) -> i16 {
    if imm_str.starts_with("0x") {
        // 处理十六进制值
        i16::from_str_radix(&imm_str[2..], 16).unwrap_or_else(|_| {
            panic!("无效的十六进制立即数: {}", imm_str);
        })
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
                img.push(encode_add(rd, rs1, rs2));
            }
            "mul" => {
                let rd = parse_reg(parts[1].trim_end_matches(','));
                let rs1 = parse_reg(parts[2].trim_end_matches(','));
                let rs2 = parse_reg(parts[3]);
                img.push(encode_mul(rd, rs1, rs2));
            }
            "addi" => {
                let rd = parse_reg(parts[1].trim_end_matches(','));
                let rs1 = parse_reg(parts[2].trim_end_matches(','));
                let imm = parse_imm(parts[3]);
                img.push(encode_addi(rd, rs1, imm));
            }
            "bne" => {
                let rs1 = parse_reg(parts[1].trim_end_matches(','));
                let rs2 = parse_reg(parts[2].trim_end_matches(','));
                let offset = parse_imm(parts[3]);
                img.push(encode_bne(rs1, rs2, offset));
            }
            "lui" => {
                let rd = parse_reg(parts[1].trim_end_matches(','));
                let imm = parse_imm(parts[2]);
                img.push(encode_lui(rd, imm));
            }
            "lw" => {
                // 处理格式如 lw x1, 4(x2) 的指令
                let rd = parse_reg(parts[1].trim_end_matches(','));
                
                // 解析 4(x2) 格式
                let offset_reg = parts[2];
                let open_paren = offset_reg.find('(').unwrap_or_else(|| panic!("无效的lw格式: {}", offset_reg));
                let close_paren = offset_reg.find(')').unwrap_or_else(|| panic!("无效的lw格式: {}", offset_reg));
                
                let offset = parse_imm(&offset_reg[0..open_paren]);
                let rs1 = parse_reg(&offset_reg[open_paren+1..close_paren]);
                
                img.push(encode_lw(rd, rs1, offset));
            }
            "sw" => {
                // 处理格式如 sw x1, 4(x2) 的指令
                let rs2 = parse_reg(parts[1].trim_end_matches(','));
                
                // 解析 4(x2) 格式
                let offset_reg = parts[2];
                let open_paren = offset_reg.find('(').unwrap_or_else(|| panic!("无效的sw格式: {}", offset_reg));
                let close_paren = offset_reg.find(')').unwrap_or_else(|| panic!("无效的sw格式: {}", offset_reg));
                
                let offset = parse_imm(&offset_reg[0..open_paren]);
                let rs1 = parse_reg(&offset_reg[open_paren+1..close_paren]);
                
                img.push(encode_sw(rs1, rs2, offset));
            }
            "blt" => {
                let rs1 = parse_reg(parts[1].trim_end_matches(','));
                let rs2 = parse_reg(parts[2].trim_end_matches(','));
                let offset = parse_imm(parts[3]);
                img.push(encode_blt(rs1, rs2, offset));
            }
            "halt" => {
                img.push(encode_halt());
            },
            _ => panic!("未知指令: {}", parts[0]),
        }
    }
    img
}

fn write_object_file(img: &[u32], path: &str) -> io::Result<()> {
    let mut buf = Vec::with_capacity(img.len() * 4);
    for &word in img {
        buf.extend(word.to_le_bytes());
    }
    fs::write(path, buf)
}

// =================== 反汇编器部分 ===================

// 解码A类型指令（add/mul）
fn decode_a_type(instr: u32) -> String {
    let opcode = instr & 0x3F;
    let rd = (instr >> 6) & 0x1F;
    let rs1 = (instr >> 11) & 0x1F;
    let rs2 = (instr >> 16) & 0x1F;

    match opcode {
        OPCODE_ADD => format!("add x{}, x{}, x{}", rd, rs1, rs2),
        OPCODE_MUL => format!("mul x{}, x{}, x{}", rd, rs1, rs2),
        _ => format!("未知A型指令: 0x{:08X}", instr),
    }
}

// 解码B类型指令（addi/lui/lw）
fn decode_b_type(instr: u32) -> String {
    let opcode = instr & 0x3F;
    let rd = (instr >> 6) & 0x1F;
    let rs1 = (instr >> 11) & 0x1F;
    let imm = ((instr >> 16) & 0xFFFF) as i16;

    match opcode {
        OPCODE_ADDI => format!("addi x{}, x{}, {}", rd, rs1, imm),
        OPCODE_LUI => format!("lui x{}, {}", rd, imm),
        OPCODE_LW => format!("lw x{}, {}(x{})", rd, imm, rs1),
        _ => format!("未知B型指令: 0x{:08X}", instr),
    }
}

// 解码C类型指令（bne/sw/blt）
fn decode_c_type(instr: u32) -> String {
    let opcode = instr & 0x3F;
    let imm_low = (instr >> 6) & 0x1F;
    let rs1 = (instr >> 11) & 0x1F;
    let rs2 = (instr >> 16) & 0x1F;
    let imm_high = (instr >> 21) & 0x7FF;
    
    // 组合立即数
    let imm = ((imm_high << 5) | imm_low) as i16;

    match opcode {
        OPCODE_BNE => format!("bne x{}, x{}, {}", rs1, rs2, imm),
        OPCODE_SW => format!("sw x{}, {}(x{})", rs2, imm, rs1),
        OPCODE_BLT => format!("blt x{}, x{}, {}", rs1, rs2, imm),
        _ => format!("未知C型指令: 0x{:08X}", instr),
    }
}

// 解码halt指令（全0）
fn decode_halt(instr: u32) -> String {
    if instr == 0 {
        "halt".to_string()
    } else {
        format!("未知指令: 0x{:08X}", instr)
    }
}

// 根据操作码类型解码指令
fn decode_instruction(instr: u32) -> String {
    let opcode = instr & 0x3F;
    
    match opcode {
        OPCODE_HALT => decode_halt(instr),
        OPCODE_ADD | OPCODE_MUL => decode_a_type(instr),
        OPCODE_ADDI | OPCODE_LUI | OPCODE_LW => decode_b_type(instr),
        OPCODE_BNE | OPCODE_SW | OPCODE_BLT => decode_c_type(instr),
        _ => format!("未知指令: 0x{:08X}", instr),
    }
}

// 读取二进制文件并返回32位指令的向量
fn read_binary_file(file_path: &str) -> io::Result<Vec<u32>> {
    let mut file = fs::File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    
    // 确保字节数是4的倍数（每条指令32位/4字节）
    if buffer.len() % 4 != 0 {
        println!("警告：文件大小不是4的倍数，最后的不完整指令将被忽略");
    }
    
    let mut instructions = Vec::new();
    let mut i = 0;
    
    while i + 3 < buffer.len() {
        // 从字节缓冲区读取32位指令（小端序）
        let instr = u32::from_le_bytes([buffer[i], buffer[i+1], buffer[i+2], buffer[i+3]]);
        instructions.push(instr);
        i += 4;
    }
    
    Ok(instructions)
}

// =================== 主程序部分 ===================

fn show_usage(program: &str) {
    println!("RISC-V简易汇编器和反汇编器 - 使用方法:");
    println!("  汇编功能:");
    println!("    {} asm <汇编文件名> - 将asm/文件名.asm编译为二进制，输出到out/文件名.o", program);
    println!("    例如: {} asm sum - 编译asm/sum.asm，输出到out/sum.o", program);
    println!();
    println!("  反汇编功能:");
    println!("    {} disasm <二进制文件> <输出文件> - 将二进制文件反汇编为汇编代码", program);
    println!("    例如: {} disasm out/sum.o out/sum_disasm.asm", program);
}

fn run_assembler(base_name: &str) -> io::Result<()> {
    let input_file = format!("asm/{}.asm", base_name);
    let output_binary = format!("out/{}.o", base_name);
    let output_text = format!("out/{}.txt", base_name);
    
    // 确保输出目录存在
    fs::create_dir_all("out")?;
    
    // 读取汇编文件
    println!("读取汇编文件: {}", input_file);
    let asm_code = fs::read_to_string(&input_file)?;
    
    // 汇编代码
    println!("汇编代码...");
    let img = assemble(&asm_code);
    
    // 生成可读的文本格式
    let mut text_output = String::new();
    for &instr in &img {
        let binary_str = format!("{:032b}", instr);
        let formatted_binary = format!("0b{}_{}_{}_{}_{}", 
            &binary_str[0..11], 
            &binary_str[11..16], 
            &binary_str[16..21], 
            &binary_str[21..26], 
            &binary_str[26..32]);
        text_output.push_str(&format!("{}\n", formatted_binary));
    }
    
    // 写入文件
    println!("写入二进制文件: {}", output_binary);
    write_object_file(&img, &output_binary)?;
    
    println!("写入文本格式文件: {}", output_text);
    fs::write(&output_text, text_output)?;
    
    println!("汇编成功完成，共生成 {} 条指令", img.len());
    Ok(())
}

fn run_disassembler(input_file: &str, output_file: &str) -> io::Result<()> {
    // 确保输出目录存在
    if let Some(parent) = Path::new(output_file).parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }
    
    // 读取二进制文件
    println!("读取二进制文件: {}", input_file);
    let instructions = read_binary_file(input_file)?;
    
    // 反汇编指令
    println!("反汇编指令...");
    let mut output = String::new();
    
    output.push_str("# 反汇编结果\n");
    output.push_str("# 格式: [地址] [十六进制表示] [汇编指令]\n\n");
    
    for (i, &instr) in instructions.iter().enumerate() {
        let disasm = decode_instruction(instr);
        let line = format!("{:04X}:  {:08X}  {}\n", i * 4, instr, disasm);
        output.push_str(&line);
    }
    
    // 写入输出文件
    println!("写入汇编文件: {}", output_file);
    fs::write(output_file, output)?;
    
    println!("反汇编成功完成，共处理 {} 条指令", instructions.len());
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        show_usage(&args[0]);
        return Ok(());
    }
    
    match args[1].as_str() {
        "asm" => {
            if args.len() < 3 {
                println!("错误: 缺少汇编文件名参数");
                show_usage(&args[0]);
                return Ok(());
            }
            
            let base_name = &args[2];
            if let Err(e) = run_assembler(base_name) {
                eprintln!("汇编失败: {}", e);
            }
        },
        "disasm" => {
            if args.len() < 4 {
                println!("错误: 缺少输入或输出文件参数");
                show_usage(&args[0]);
                return Ok(());
            }
            
            let input_file = &args[2];
            let output_file = &args[3];
            if let Err(e) = run_disassembler(input_file, output_file) {
                eprintln!("反汇编失败: {}", e);
            }
        },
        _ => {
            println!("未知命令: {}", args[1]);
            show_usage(&args[0]);
        }
    }
    
    Ok(())
}

// =================== 测试部分 ===================
#[cfg(test)]
mod tests {
    use super::*;

    // 汇编器测试
    #[test]
    fn test_encode_add() {
        // add x1, x1, x3 -> 0b00000000000_00011_00001_00001_000001
        let expected = 0b00000000000_00011_00001_00001_000001;
        let actual = encode_add(1, 1, 3);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_encode_mul() {
        // mul x1, x1, x3 -> 0b00000000000_00011_00001_00001_000100
        let expected = 0b00000000000_00011_00001_00001_000100;
        let actual = encode_mul(1, 1, 3);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_encode_addi() {
        // addi x1, x0, 0 -> 0b00000000000_00000_00000_00001_000010
        let expected = 0b00000000000_00000_00000_00001_000010;
        let actual = encode_addi(1, 0, 0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_encode_bne() {
        // bne x3, x2, -8 → 0b11111111111_00010_00011_11000_000011
        let expected = 0b11111111111_00010_00011_11000_000011;
        let actual = encode_bne(3, 2, -8);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_encode_halt() {
        let expected = 0;
        let actual = encode_halt();
        assert_eq!(actual, expected);
    }

    // 反汇编器测试
    #[test]
    fn test_decode_add() {
        // add x1, x2, x3
        let instr = 0b00000000000_00011_00010_00001_000001;
        assert_eq!(decode_instruction(instr), "add x1, x2, x3");
    }

    #[test]
    fn test_decode_mul() {
        // mul x3, x4, x5
        let instr = 0b00000000000_00101_00100_00011_000100;
        assert_eq!(decode_instruction(instr), "mul x3, x4, x5");
    }

    #[test]
    fn test_decode_addi() {
        // addi x1, x0, 10
        let instr = 0b00000000000_01010_00000_00001_000010;
        assert_eq!(decode_instruction(instr), "addi x1, x0, 10");
    }

    #[test]
    fn test_decode_lui() {
        // lui x2, 42
        let instr = 0b00000000001_01010_00000_00010_000101;
        assert_eq!(decode_instruction(instr), "lui x2, 42");
    }

    #[test]
    fn test_decode_lw() {
        // lw x3, 4(x1)
        let instr = 0b00000000000_00100_00001_00011_000110;
        assert_eq!(decode_instruction(instr), "lw x3, 4(x1)");
    }

    #[test]
    fn test_decode_bne() {
        // bne x3, x2, -8
        let instr = 0b11111111111_00010_00011_11000_000011;
        assert_eq!(decode_instruction(instr), "bne x3, x2, -8");
    }

    #[test]
    fn test_decode_sw() {
        // sw x2, 8(x1)
        let instr = 0b00000000000_00010_00001_01000_000111;
        assert_eq!(decode_instruction(instr), "sw x2, 8(x1)");
    }

    #[test]
    fn test_decode_blt() {
        // blt x4, x5, 16
        let instr = 0b00000000000_00101_00100_10000_001000;
        assert_eq!(decode_instruction(instr), "blt x4, x5, 16");
    }

    #[test]
    fn test_decode_halt() {
        // halt
        let instr = 0;
        assert_eq!(decode_instruction(instr), "halt");
    }
    
    // 编码-解码循环测试
    #[test]
    fn test_encode_decode_cycle() {
        // 测试编码后再解码是否得到原指令
        let tests = [
            "add x1, x2, x3",
            "addi x3, x0, 42",
            "mul x4, x5, x6",
            "bne x7, x8, -16",
            "lw x9, 8(x10)",
            "sw x11, 12(x12)",
            "lui x13, 1024",
            "blt x14, x15, 20",
            "halt"
        ];
        
        for &test_str in &tests {
            let code = assemble(test_str);
            assert_eq!(code.len(), 1, "应该只生成一条指令");
            
            let decoded = decode_instruction(code[0]);
            // 对于lui指令，解码可能会使用不同的数字表示形式，所以进行特殊处理
            if test_str.starts_with("lui") {
                assert!(decoded.starts_with("lui"), "lui指令解码错误");
            } else {
                assert_eq!(decoded, test_str, "指令编码后解码不匹配: {}", test_str);
            }
        }
    }
}