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
    pub fn from(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        args.next();

        let query_str = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        Ok(
            Config {
                query_str,
                filename,
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
    contents.lines().filter(|line| {
        line.contains(query_str)
    }).collect()
}

pub fn search_case_insensitive<'a>(query_str: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| {
        line.to_lowercase().contains(&query_str)
    }).collect()
}