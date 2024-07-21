mod tags;
use clap::ArgMatches;
use regex::Regex;
use std::{io::{self, IsTerminal}, str::FromStr};

#[derive(Debug, PartialEq)]
struct Field {
    tag: usize,
    value: String,
}

pub struct Options {
    delimiter: String,
    colour: bool,
    strip: bool,
    summarise: Option<String>,
    tag: bool,
    value: bool,
}

pub fn matches_to_flags(matches: &ArgMatches) -> Options {
    let when = matches.get_one::<String>("color").unwrap();
    let use_colour = (io::stdout().is_terminal() && when == "auto") || when == "always";
    Options {
        delimiter: matches.get_one::<String>("delimiter").unwrap().to_string(),
        colour: use_colour,
        strip: matches.get_flag("strip"),
        summarise: matches.get_one::<String>("summarise").cloned(),
        tag: matches.get_flag("tag"),
        value: matches.get_flag("value"),
    }
}

pub fn run(input: &Vec<String>, flags: Options) -> Result<(), &'static str> {
    for line in input {
        let parsed = match parse_fix_msg(line) {
            Ok(parsed) => parsed,
            Err(_) => {
                if flags.tag {
                    println!("{}", parse_tags(line));
                } else {
                    // Maintain non FIX lines.
                    println!("{}", line);
                }
                continue;
            }
        };
        if let Some(ref template) = flags.summarise {
            println!("{}", format_to_summary(parsed, template, flags.value));
        } else {
            println!(
                "{}",
                format_to_string(parsed, &flags)
            );
        };
    }
    Ok(())
}

fn parse_fix_msg(input: &str) -> Result<Vec<Field>, &'static str> {
    let input = input.trim();
    // matches against a number followed by an = followed by anything excluding the given delimiters
    // Current delimiters used: ^ | SOH \n
    let regex = Regex::new(r"(?P<tag>[0-9]+)=(?P<value>[^\^\|\x01\n]+)").expect("bad regex");
    let mut result = Vec::<Field>::new();

    if !regex.is_match(input) {
        // Do not panic on not finding a FIX message. Allows prefix to work well with fzf
        return Err("cannot find a valid FIX message");
    }

    for i in regex.captures_iter(input) {
        result.push(Field {
            tag: FromStr::from_str(&i["tag"]).expect("cannot parse tag"),
            value: i["value"].to_string(),
        })
    }
    Ok(result)
}

fn parse_tags(input: &str) -> String {
    let mut result = input.to_owned();
    let regex = Regex::new(r"[0-9]+").unwrap();
    for m in regex.find_iter(input) {
        let tag = m.as_str().parse::<usize>().unwrap();
        result = result.replace(m.as_str(), tags::TAGS.get(tag).unwrap_or(&m.as_str()));
    }
    result
}

fn add_colour(input: &str, use_colour: bool) -> String {
    if use_colour {
        format!("\x1b[33m{}\x1b[0m", input)
    } else {
        input.to_string()
    }
}

fn format_to_string(
    input: Vec<Field>,
    flags: &Options,
) -> String {
    let mut result = String::new();
    for field in input {
        // Allow custom tags to still be printed without translation
        if field.tag >= tags::TAGS.len() {
            result.push_str(&field.tag.to_string());
        } else {
            result.push_str(tags::TAGS[field.tag]);
        }
        if flags.strip {
            result.push_str(&add_colour("=", flags.colour));
        } else {
            result.push_str(&add_colour(" = ", flags.colour));
        }
        result.push_str(match flags.value {
            true => translate_value(&field),
            false => &field.value,
        });
        result.push_str(&add_colour(&flags.delimiter, flags.colour));
    }
    result
}

fn format_to_summary(input: Vec<Field>, template: &str, value_flag: bool) -> String {
    let mut result = String::from(template);
    let mut order_type = String::new();
    for field in input {
        let value = if value_flag {
            translate_value(&field)
        } else {
            &field.value
        };
        if !template.is_empty() {
            // Replace tag numbers in template to tag name.
            /* TODO: How in efficient is attempting to search and replace for each field?
             *       Profile and consider parsing which tags are in template first. */
            result = result.replace(&field.tag.to_string(), value);
        }
        if field.tag == 35 {
            order_type = field.value;
        }
    }
    for msg_type in tags::MSG_TYPES {
        if msg_type.0 == order_type {
            result = format!("{} {}", msg_type.1, result);
        }
    }
    result
}

// Not ideal but leaves it simple and easy for anyone to add values. This function is opt in.
fn translate_value(field: &Field) -> &str {
    match field.tag {
        // OrdType
        40 => match field.value.as_str() {
            "1" => "Market",
            "2" => "Limit",
            "3" => "Stop",
            // Keep as one word for better usage with awk
            "4" => "StopLimit",
            "D" => "PreviouslyQuoted",
            _ => &field.value,
        },
        // Side
        54 => match field.value.as_str() {
            "1" => "Buy",
            "2" => "Sell",
            _ => &field.value,
        },
        // TimeInForce
        59 => match field.value.as_str() {
            "0" => "Day",
            "1" => "GTC",
            "2" => "OPG",
            "3" => "IOC",
            "4" => "FOK",
            "5" => "GTX",
            "6" => "GTD",
            _ => &field.value,
        },
        // ExecType
        150 => match field.value.as_str() {
            "0" => "New",
            "1" => "PartialFill",
            "2" => "Fill",
            "4" => "Canceled",
            "8" => "Rejected",
            "F" => "Trade",
            _ => &field.value,
        },
        // SubscriptionRequestType
        263 => match field.value.as_str() {
            "0" => "Snapshot",
            "1" => "Subscribe",
            "2" => "Unsubscribe",
            _ => &field.value,
        },
        // MDEntryType
        269 => match field.value.as_str() {
            "0" => "Bid",
            "1" => "Offer",
            "2" => "Trade",
            _ => &field.value,
        },
        // MDUpdateAction
        279 => match field.value.as_str() {
            "0" => "New",
            "1" => "Change",
            "2" => "Delete",
            _ => &field.value,
        },
        _ => &field.value,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! field {
        ($($tag:literal,$value:literal),+) => {
            $(
                Field{
                    tag: $tag,
                    value: String::from($value),
                }
            ),+
        }
    }

    #[test]
    fn basic_parse_case() {
        let input = "8=4.4^1=test^55=EUR/USD";
        let result = parse_fix_msg(input).unwrap();
        let expected: Vec<Field> = vec![field!(8, "4.4"), field!(1, "test"), field!(55, "EUR/USD")];
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_case() {
        let input =
            "25=test^1=aaa^8=4.4^123=Capital^243:log[]efssdfkj39809^55=ETH-USD^101=55:05:22";
        let result = parse_fix_msg(input).unwrap();
        let expected: Vec<Field> = vec![
            field!(25, "test"),
            field!(1, "aaa"),
            field!(8, "4.4"),
            field!(123, "Capital"),
            field!(55, "ETH-USD"),
            field!(101, "55:05:22"),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn format_case() {
        let input = "8=FIX.4.4^1=test^55=ETH/USD^54=1^29999=50";
        let parsed = parse_fix_msg(input).unwrap();
        let flags = Options {
            delimiter: String::from("|"),
            colour: false,
            strip: true,
            summarise: None,
            tag: false,
            value: false,
        };
        let result = format_to_string(parsed, &flags);
        let expected = String::from(
            "BeginString = FIX.4.4|Account = test|Symbol = ETH/USD|Side = Buy|29999 = 50|",
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn tag_case() {
        let input = "symbol? 55";
        let parsed = parse_tags(input);
        assert_eq!(parsed, "symbol? Symbol");
        let input = "54,11,8";
        let parsed = parse_tags(input);
        assert_eq!(parsed, "Side,ClOrdID,BeginString");
    }
}
