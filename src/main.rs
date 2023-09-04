use regex::Regex;
use std::env;
use std::process;
mod find_file;
use colored::*;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("使用方式: {} <目标目录> <要搜索的正则表达式>", args[0]);
        process::exit(1);
    }

    let pattern = &args[2];
    let regex = match Regex::new(pattern) {
        Ok(re) => re,
        Err(err) => {
            eprintln!("无效的正则表达式 '{}':{}", pattern, err);
            process::exit(1);
        }
    };
    
 match find_file::find(&args[1], &regex) {
        Ok(matches) => {
            if matches.is_empty() {
                println!("{}", "未找到匹配项。".red()); // 使用红色输出
            } else {
                println!("{}", "找到以下匹配项：".green()); // 使用绿色输出
                for file in matches {
                    println!("{}", file.blue()); // 使用蓝色输出
                }
            }
        }
        Err(error) => {
            println!("{}", format!("发生错误：{}", error).red()); // 使用红色输出错误信息
            process::exit(1);
        }
    }
}



