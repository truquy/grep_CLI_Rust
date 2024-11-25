use std::error::Error;
use std::fs;

pub fn run(cli: CLI) -> Result<(), Box<dyn Error>> {
    let content: String = fs::read_to_string(cli.file)?;
    
    for line in search(&cli.text, &content) {
        println!("{}", line,);
    }
    println!("\n");
    Ok(())
}

pub struct CLI {
    pub text: String,
    pub file: String,
}

impl CLI {
    pub fn new(cli_args: &[String]) -> Result<CLI, &str> {
        // Check the index of the cli
        if cli_args.len() < 3 {
            return Err("Not enough argument");
        }

        let text = cli_args[1].clone();
        let file = cli_args[2].clone();

        Ok(CLI { text, file })
    }
}

// Writing Test module

pub fn search<'a>(text: &str, content:&'a str) ->Vec<&'a str> {
    let mut results = Vec::new();

    for line in content.lines() {
        if line.contains(text) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let text = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick threee." ;

    assert_eq!(vec!["safe, fast, productive."], search (text, content));
    }
}