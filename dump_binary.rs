use std::fs;
use std::env;
use std::path::Path;

fn main() -> std::io::Result<()> {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("用法: {} <文件名>", args[0]);
        println!("示例: {} sum - 将检查out/sum.o文件", args[0]);
        return Ok(());
    }
    
    // 获取基本文件名（不带扩展名）
    let base_name = &args[1];
    let binary_file = format!("out/{}.o", base_name);
    
    // 读取二进制文件
    let data = fs::read(&binary_file)?;
    
    // 检查魔数
    if data.len() < 8 {
        println!("文件太小，无法包含头信息");
        return Ok(());
    }
    
    // 读取魔数
    let magic = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
    println!("魔数: 0x{:X}", magic);
    
    // 读取指令数量
    let count = u32::from_be_bytes([data[4], data[5], data[6], data[7]]) as usize;
    println!("指令数量: {}", count);
    
    // 检查文件大小是否符合预期
    if data.len() != 8 + count * 4 {
        println!("文件大小不符合预期，预期 {} 字节，实际 {} 字节", 8 + count * 4, data.len());
        return Ok(());
    }
    
    // 逐条打印指令
    println!("\n指令内容:");
    for i in 0..count {
        let offset = 8 + i * 4;
        let instr = u32::from_be_bytes([
            data[offset], 
            data[offset + 1], 
            data[offset + 2], 
            data[offset + 3]
        ]);
        
        // 使用下划线分割二进制表示
        let binary_str = format!("{:032b}", instr);
        let formatted_binary = format!("0b{}_{}_{}_{}_{}", 
            &binary_str[0..11], 
            &binary_str[11..16], 
            &binary_str[16..21], 
            &binary_str[21..26], 
            &binary_str[26..32]);
            
        println!("指令 {}: {} (十六进制: 0x{:08X})", i+1, formatted_binary, instr);
    }
    
    // 检查是否存在预期输出文件
    let expected_path = Path::new("asm").join(format!("{}_expected.txt", base_name));
    if expected_path.exists() {
        match fs::read_to_string(&expected_path) {
            Ok(expected) => {
                println!("\n与预期输出比较 ({}):", expected_path.display());
                
                let expected_lines: Vec<_> = expected.lines()
                    .filter(|line| line.starts_with("0b"))
                    .collect();
                
                if expected_lines.len() != count {
                    println!("预期指令数量 ({}) 与实际指令数量 ({}) 不匹配", 
                        expected_lines.len(), count);
                } else {
                    // 创建一个匹配结果的向量
                    let mut matches = vec![false; count];
                    
                    for (i, expected_line) in expected_lines.iter().enumerate() {
                        let offset = 8 + i * 4;
                        let actual = u32::from_be_bytes([
                            data[offset], 
                            data[offset + 1], 
                            data[offset + 2], 
                            data[offset + 3]
                        ]);
                        
                        // 将预期文本转换为数字进行比较
                        let expected_str = expected_line.trim()
                            .replace("_", "")
                            .replace("0b", "");
                        
                        let expected_num = match u32::from_str_radix(&expected_str, 2) {
                            Ok(num) => num,
                            Err(_) => {
                                println!("无法解析预期值: {}", expected_line);
                                continue;
                            }
                        };
                        
                        matches[i] = actual == expected_num;
                        
                        if actual == expected_num {
                            println!("指令 {}: 匹配 ✓", i+1);
                        } else {
                            println!("指令 {}: 不匹配 ✗", i+1);
                            println!("  预期: 0b{:032b}", expected_num);
                            println!("  实际: 0b{:032b}", actual);
                        }
                    }
                    
                    // 总结
                    let match_count = matches.iter().filter(|&&m| m).count();
                    println!("\n总结: {} 条指令中有 {} 条匹配, {} 条不匹配", 
                        count, match_count, count - match_count);
                    if match_count == count {
                        println!("所有指令都与预期匹配！✓✓✓");
                    } else {
                        println!("存在不匹配的指令！❌");
                    }
                }
            },
            Err(e) => println!("无法读取预期输出文件: {}", e),
        }
    } else {
        println!("\n未找到预期输出文件: {}", expected_path.display());
    }
    
    Ok(())
} 