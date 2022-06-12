mod tags;
use regex::Regex;
use std::io::{self, Write};
use std::str::FromStr;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_parse_case() {
        let input = "8=4.4^1=test^55=EUR/USD";
        let result = parse(input).unwrap();
        let expected: Vec<Field> = vec![
            Field {
                tag: 8,
                value: String::from("4.4"),
            },
            Field {
                tag: 1,
                value: String::from("test".to_string()),
            },
            Field {
                tag: 55,
                value: String::from("EUR/USD".to_string()),
            },
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_case() {
        let input =
            "55=test^1=aaa^8=4.4^123=Capital^243:log[]efssdfkj39809^55=ETH-USD^001=55:05:22";
        let result = parse(input).unwrap();
        let expected: Vec<Field> = vec![
            Field {
                tag: 55,
                value: String::from("test"),
            },
            Field {
                tag: 1,
                value: String::from("aaa"),
            },
            Field {
                tag: 8,
                value: String::from("4.4"),
            },
            Field {
                tag: 123,
                value: String::from("Capital"),
            },
            Field {
                tag: 55,
                value: String::from("ETH-USD"),
            },
            Field {
                tag: 001,
                value: String::from("55:05:22"),
            },
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn format_case() {
        let input = "8=FIX.4.4^1=test^55=ETH/USD^54=1^29999=50";
        let parsed = parse(input).unwrap();
        let result = format_to_string(parsed, true, "|");
        let expected = String::from(
            "BeginString = FIX.4.4|Account = test|Symbol = ETH/USD|Side = Buy|29999 = 50|",
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn multiple_message_case() {
        let input =
            "8=FIX.4.2|1=ACCOUNT|299=1234^55=USDJPY\n8=FIX.4.4|1=ACCOUNT2|299=4321|55=EURJPY|";
        let parsed = parse(input).unwrap();
        let result = format_to_string(parsed, true, "| ");
        let expected =
            String::from("BeginString = FIX.4.2| Account = ACCOUNT| QuoteEntryID = 1234| Symbol = USDJPY| \nBeginString = FIX.4.4| Account = ACCOUNT2| QuoteEntryID = 4321| Symbol = EURJPY| ");
        assert_eq!(result, expected);
    }
}

#[derive(Debug, PartialEq)]
struct Field {
    tag: usize,
    value: String,
}

pub fn run(input: &str, value_flag: bool, delimiter: &str) -> Result<(), &'static str> {
    let parsed = parse(input)?;
    let to_print = format_to_string(parsed, value_flag, delimiter);
    print(to_print);
    Ok(())
}

fn parse(input: &str) -> Result<Vec<Field>, &'static str> {
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

fn format_to_string(input: Vec<Field>, value_flag: bool, delimiter: &str) -> String {
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
        result.push_str(" = ");
        result.push_str(match value_flag {
            true => translate_value(&i),
            false => &i.value,
        });
        result.push_str(delimiter);
    }
    result
}

fn print(input: String) {
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
