use std::error::Error;
use std::{env, fs};

// config 配置
pub struct Config {
    // 需要查找的字符串
    pub query_str: String,
    // 被查找的文件名
    pub filename: String,
    // 区分大小写
    pub case_sensitive: bool,
}

impl Config {
    // parse config from args
    pub fn from(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        Ok(
            Config {
                query_str: args[1].clone(),
                filename: args[2].clone(),
                case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
            }
        )
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let result = if config.case_sensitive {
        search(&config.query_str, &contents)
    } else {
        search_case_insensitive(&config.query_str, &contents)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query_str: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = vec![];

    for line in contents.lines() {
        if line.contains(query_str) {
            result.push(line)
        }
    }

    result
}

pub fn search_case_insensitive<'a>(query_str: &str, contents: &'a str) -> Vec<&'a str> {
    let query_str = query_str.to_lowercase();
    let mut result = vec![];

    for line in contents.lines() {
        if line.to_lowercase().contains(&query_str) {
            result.push(line)
        }
    }

    result
}