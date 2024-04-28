use std::error::Error;
use std::{fs, process};
use std::env;
use std::path::Path;
use pulldown_cmark::{Parser, Options, Event, Tag, html};
use scraper::Html;  // 用于解析HTML文档

pub struct Config {
    pub query : String,
    pub file_path : String,
    pub file_type: String,
    pub ignore_case : bool,
}

pub fn run(config : Config) ->Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;
    if config.file_type=="Text" {
        let (results, r1, r2) = if config.ignore_case {
            search_case_insensitive(&config.query, &contents)
        } else {
            search(&config.query, &contents)
        };

        for line in results {
            println!("{line}");
        }
        if env::var("ROWSOFFILE").unwrap() == "1" {
            println!("ROWS OF FILE: {r1}")
        }
        if env::var("ROWS").unwrap() == "1" {
            println!("ROWS: {r2}")
        }
    } else if config.file_type=="Markdown" {
        let (results, r1, r2) = if config.ignore_case {
            search_case_insensitive_for_md(&config.query, &contents)
        } else {
            searchMd(&config.query, &contents)
        };

        for line in results {
            println!("{line}");
        }
        if env::var("ROWSOFFILE").unwrap() == "1" {
            println!("ROWS OF FILE: {r1}")
        }
        if env::var("ROWS").unwrap() == "1" {
            println!("ROWS: {r2}")
        }
    } else {
        eprint!("FILE TYPE NOT SUPPORT");
        process::exit(1);
    }
    Ok(())
}

impl Config {
   /* pub fn new(args:&Vec<String>) ->Config {
        Config{
            query:args[1].clone(),
            file_path:args[2].clone(),
        }
    }*/
    pub fn build(args:&Vec<String>) -> Result<Config,&'static str> {
        if args.len()<3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        let path = Path::new(&file_path);

        let file_type = match path.extension().and_then(|s| s.to_str()) {
            Some("md") => "Markdown",
            Some("txt") => "Text",
            _ => "Unknown",
        };
        let typee=String::from(file_type);
        Ok(Config {
            query:query,
            file_path:file_path,
            file_type:typee,
            ignore_case:ignore_case,
        })
    }
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> (Vec<&'a str>,u64,u64) {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    let mut r1 :u64=0;
    let mut r2 :u64=0;
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
            r2+=1;
        }
        r1+=1;
    }

    (results,r1,r2)
}

pub fn search_case_insensitive_for_md<'a>(
    query:&str,
    contents: &'a str,
) -> (Vec<&'a str>,u64,u64) {
    let mut results = Vec::new();
    let mut total_lines: u64 = 0;
    let mut matched_lines: u64 = 0;

    // 使用正则表达式移除Markdown标签
    let re = regex::Regex::new(r"([\*_`\[\]<>]|\(\S+?\))").unwrap();
    let plain_text = re.replace_all(contents, "");

    // 将查询字符串和文本内容都转换为小写
    let query_lower = query.to_lowercase();
    let text_lower = plain_text.to_lowercase();

    // 按行分割并搜索
    for line in text_lower.lines() {
        total_lines += 1;
        if line.contains(&query_lower) {
            // 在原始文本中找到对应的行
            let line_number = total_lines - 1;
            let original_line = contents.lines().nth(line_number as usize).unwrap();
            results.push(original_line);
            matched_lines += 1;
        }
    }

    (results, total_lines, matched_lines)
}

// in lib.rs
pub fn search<'a>(query: &str, contents: &'a str) -> (Vec<&'a str>,u64,u64) {
    let mut results = Vec::new();
    let mut r1 :u64=0;
    let mut r2 :u64=0;
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
            r2+=1;
        }
        r1+=1;
    }

    (results,r1,r2)
}

pub fn searchMd<'a>(query: &str,contents: &'a str) -> (Vec<&'a str>,u64,u64) {
    let mut results = Vec::new();
    let mut total_lines: u64 = 0;
    let mut matched_lines: u64 = 0;

    // 使用正则表达式移除Markdown标签
    let re = regex::Regex::new(r"([\*_`\[\]<>]|\(\S+?\))").unwrap();

    // 按行分割并搜索
    for line in contents.lines() {
        total_lines += 1;
        let plain_line = re.replace_all(line, "");
        if plain_line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line);
            matched_lines += 1;
        }
    }

    (results, total_lines, matched_lines)
}
