use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    // if config.ignore_case {
    //     println!("大小写不敏感");
    // } else {
    //     println!("大小写敏感");
    // }
    let results = if !config.ignore_case {
        search_case_sensitive(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in results {
        println!("{line}");
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string!"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file_path string!"),
        };
        let ignore_case = match args.next() {
            Some(s) => s.eq_ignore_ascii_case("true"),
            None => false,
        };
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
pub fn search_case_sensitive<'a>(query: &str, cont: &'a str) -> Vec<&'a str> {
    cont.lines().filter(|line| line.contains(query)).collect()
}
fn search_case_insensitive<'a>(query: &str, cont: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    cont.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "believe";
        let cont = "\
Maybe I just want to fly
Want to live, I don’t want to die
Maybe I just want to breathe
Maybe I just don’t believe
        ";
        assert_eq!(
            vec!["Maybe I just don’t believe"],
            search_case_sensitive(query, cont)
        );
    }
    #[test]
    fn case_insensitive() {
        let query = "Fly";
        let cont = "\
Maybe I just want to fly
Want to live, I don’t want to die
Maybe I just want to breathe
Maybe I just don’t believe
        ";
        assert_eq!(
            vec!["Maybe I just want to fly"],
            search_case_insensitive(query, cont)
        );
    }
}
