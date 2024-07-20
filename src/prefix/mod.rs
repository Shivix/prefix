mod tags;
use clap::ArgMatches;
use regex::Regex;
use std::io::{self, Write};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Field {
    tag: usize,
    value: String,
}

pub struct Flags {
    value: bool,
    strip: bool,
    tag: bool,
    summarise: Option<String>,
}

pub fn matches_to_flags(matches: &ArgMatches) -> Flags {
    Flags {
        value: matches.get_flag("value"),
        strip: matches.get_flag("strip"),
        tag: matches.get_flag("tag"),
        summarise: matches.get_one::<String>("summarise").cloned(),
    }
}

pub fn run(input: &Vec<String>, delimiter: &str, flags: Flags) -> Result<(), &'static str> {
    if flags.tag {
        for tag in input {
            print(parse_tag(tag));
        }
        return Ok(());
    }
    for msg in input {
        let parsed = match parse_fix_msg(msg) {
            Ok(parsed) => parsed,
            Err(_) => {
                // Maintain non FIX lines.
                println!("{}", msg);
                continue;
            }
        };
        if let Some(ref template) = flags.summarise {
            println!("{}", format_to_summary(parsed, template, flags.value));
        } else {
            print(&format_to_string(
                parsed,
                flags.value,
                delimiter,
                flags.strip,
            ));
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

fn parse_tag(input: &str) -> &str {
    let tag = input.parse::<usize>().expect("invalid fix tag provided");
    return tags::TAGS.get(tag).expect("not a standard fix tag");
}

fn format_to_string(
    input: Vec<Field>,
    value_flag: bool,
    delimiter: &str,
    strip_flag: bool,
) -> String {
    let mut result = String::new();
    let mut msg_counter = 0;
    for i in input {
        /* makes it easier to mentally parse seperate messages with default args and easier to
         * computationally parse when using a delimiter such as | ensures each message is on its
         * own line */
        if i.tag == 8 {
            msg_counter += 1;
            if msg_counter > 1 {
                result.push('\n');
            }
        }
        // Allow custom tags to still be printed without translation
        if i.tag >= tags::TAGS.len() {
            result.push_str(&i.tag.to_string());
        } else {
            result.push_str(tags::TAGS[i.tag]);
        }
        if strip_flag {
            result.push('=');
        } else {
            result.push_str(" = ");
        }
        result.push_str(match value_flag {
            true => translate_value(&i),
            false => &i.value,
        });
        result.push_str(delimiter);
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
            result = msg_type.1.to_string() + &result;
        }
    }
    result
}

fn print(input: &str) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle
        .write_all(input.as_bytes())
        .expect("could not print to stdout");
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
        let result = format_to_string(parsed, true, "|", false);
        let expected = String::from(
            "BeginString = FIX.4.4|Account = test|Symbol = ETH/USD|Side = Buy|29999 = 50|",
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn multiple_message_case() {
        let input =
            "8=FIX.4.2|1=ACCOUNT|299=1234^55=USDJPY\n8=FIX.4.4|1=ACCOUNT2|299=4321|55=EURJPY|";
        let parsed = parse_fix_msg(input).unwrap();
        let result = format_to_string(parsed, true, "|", true);
        let expected =
            String::from("BeginString=FIX.4.2|Account=ACCOUNT|QuoteEntryID=1234|Symbol=USDJPY|\nBeginString=FIX.4.4|Account=ACCOUNT2|QuoteEntryID=4321|Symbol=EURJPY|");
        assert_eq!(result, expected);
    }

    #[test]
    fn tag_case() {
        let input = "55";
        let parsed = parse_tag(input);
        assert_eq!(parsed, "Symbol");
    }
}
