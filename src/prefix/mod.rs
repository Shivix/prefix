mod tags;
use regex::Regex;
use std::str::FromStr;
use std::io::{self, Write};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_parse_case() {
        let input = String::from("8=4.4^1=test^55=EUR/USD");
        let result = parse(input).unwrap();
        let expected: Vec<Value> = vec![Value{tag: 8, value: String::from("4.4")},
                                        Value{tag: 1, value: String::from("test".to_string())},
                                        Value{tag: 55, value: String::from("EUR/USD".to_string())}];
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_case() {
        let input = String::from("55=test^1=aaa^8=4.4^123=Capital^243:log[]efssdfkj39809^55=ETH-USD^001=55:05:22");
        let result = parse(input).unwrap();
        let expected: Vec<Value> = vec![Value{tag: 55, value: String::from("test")},
                                        Value{tag: 1, value: String::from("aaa")},
                                        Value{tag: 8, value: String::from("4.4")},
                                        Value{tag: 123, value: String::from("Capital")},
                                        Value{tag: 55, value: String::from("ETH-USD")},
                                        Value{tag: 001, value: String::from("55:05:22")}];
        assert_eq!(result, expected);
    }
}

#[derive(Debug, PartialEq)]
struct Value{
    tag: i32,
    value: String,
}

pub fn run(input: String) -> Result<(), &'static str> {
    let parsed = parse(input)?;
    print(parsed);
    Ok(())
}

// FIXME: currently will not parse \001 delimiter
fn parse(input: String) -> Result<Vec<Value>, &'static str>{
    // matches against a number followed by an = followed by anything excluding the given delimiters
    let regex = Regex::new(r"(?P<tag>[0-9]+)=(?P<value>[^\^\|]+)").expect("Bad regex");
    let mut result = Vec::<Value>::new();

    if !regex.is_match(&input) {
        return Err("Could not find a valid FIX message")
    }

    for i in regex.captures_iter(&input) {
        result.push(Value{tag: FromStr::from_str(&i["tag"]).expect("Could not parse tag"),
                          value: i["value"].to_string()})
    }
    Ok(result)
}

fn print(input: Vec<Value>) {
    // print all at once to aid piping
    let mut to_print = String::new();
    
    for i in input {
        // incase any non standard tags are used
        if i.tag as usize > tags::TAGS.len() {
            to_print.push_str(&i.tag.to_string());
        }
        else {
            to_print.push_str(tags::TAGS[i.tag as usize]);
        }
        to_print.push_str(" = ");
        to_print.push_str(&i.value);
        to_print.push('\n');
    }

    // print using stdout to allow further piping
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(to_print.as_bytes()).expect("Could not print to stdout");
}

