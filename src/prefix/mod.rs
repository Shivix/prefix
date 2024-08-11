mod tags;

use clap::ArgMatches;
use regex::Regex;
use std::io::{self, IsTerminal};

#[derive(Debug, PartialEq)]
struct Field {
    tag: usize,
    value: String,
}

pub struct Options {
    delimiter: String,
    colour: bool,
    strict: bool,
    strip: bool,
    summary: Option<String>,
    tag: bool,
    value: bool,
    only_fix: bool,
}

#[derive(Debug, PartialEq)]
enum FixMsg {
    Full(Vec<Field>),
    Partial(Vec<Field>),
    None,
}

pub fn matches_to_flags(matches: &ArgMatches) -> Options {
    let when = matches.get_one::<String>("color").unwrap();
    let use_colour = (io::stdout().is_terminal() && when == "auto") || when == "always";
    Options {
        delimiter: matches.get_one::<String>("delimiter").unwrap().to_string(),
        colour: use_colour,
        strict: matches.get_flag("strict"),
        strip: matches.get_flag("strip"),
        summary: matches.get_one::<String>("summary").cloned(),
        tag: matches.get_flag("tag"),
        value: matches.get_flag("value"),
        only_fix: matches.get_flag("only-fix"),
    }
}

fn get_msg_regex() -> Regex {
    Regex::new(r"(?P<tag>[0-9]+)=(?P<value>[^\^\|\x01]+)").unwrap()
}

fn get_tag_regex() -> Regex {
    Regex::new(r"[0-9]+").unwrap()
}

pub fn run(input: &[String], flags: &Options) {
    let fix_msg_regex = get_msg_regex();
    let fix_tag_regex = get_tag_regex();

    for (i, line) in input.iter().enumerate() {
        match parse_fix_msg(line, &fix_msg_regex) {
            FixMsg::Full(parsed) => {
                print_fix_msg(i, input.len(), &parsed, flags);
            }
            FixMsg::Partial(parsed) => {
                if !flags.strict {
                    print_fix_msg(i, input.len(), &parsed, flags);
                } else if !flags.only_fix {
                    print_non_fix_msg(line, &fix_tag_regex, flags);
                }
            }
            FixMsg::None => {
                if !flags.only_fix {
                    print_non_fix_msg(line, &fix_tag_regex, flags);
                }
            }
        }
    }
}

fn print_non_fix_msg(line: &str, fix_tag_regex: &Regex, flags: &Options) {
    if flags.tag {
        println!("{}", parse_tags(line, fix_tag_regex));
    } else {
        println!("{}", line);
    }
}

fn print_fix_msg(line_number: usize, last_line: usize, fix_msg: &[Field], flags: &Options) {
    if let Some(ref template) = flags.summary {
        println!("{}", format_to_summary(fix_msg, template, flags.value));
    } else {
        // Avoid adding an empty new line at the bottom of the output.
        if line_number + 1 == last_line && flags.delimiter == "\n" {
            print!("{}", format_to_string(fix_msg, flags));
        } else {
            println!("{}", format_to_string(fix_msg, flags));
        }
    };
}

fn parse_fix_msg(input: &str, regex: &Regex) -> FixMsg {
    // matches against a number followed by an = followed by anything excluding the given delimiters
    // Current delimiters used: ^ | SOH
    if !regex.is_match(input) {
        // If a log file is being piped in, it's expected to have some lines without FIX messages.
        return FixMsg::None;
    }

    let mut contains_begin_string = false;
    let mut contains_check_sum = false;

    let mut result = Vec::new();
    for i in regex.captures_iter(input) {
        let tag = i["tag"].parse().expect("found non numerical tag");
        if tag == 8 {
            contains_begin_string = true;
        } else if tag == 10 {
            contains_check_sum = true;
        }
        result.push(Field {
            tag,
            value: i["value"].to_string(),
        })
    }
    if !contains_begin_string || !contains_check_sum {
        return FixMsg::Partial(result);
    }
    FixMsg::Full(result)
}

fn parse_tags(input: &str, regex: &Regex) -> String {
    let mut result = input.to_owned();
    for m in regex.find_iter(input) {
        let tag = m.as_str().parse::<usize>().unwrap();
        result = result.replace(m.as_str(), tags::TAGS.get(tag).unwrap_or(&m.as_str()));
    }
    result
}

fn add_colour(input: &str, use_colour: bool) -> String {
    if use_colour {
        // TODO: Allow configuring colour using ENV variable
        format!("\x1b[33m{}\x1b[0m", input)
    } else {
        input.to_string()
    }
}

fn format_to_string(input: &[Field], flags: &Options) -> String {
    input.iter().fold(String::new(), |result, field| {
        // Allow custom tags to still be printed without translation
        let tag = if field.tag >= tags::TAGS.len() {
            &field.tag.to_string()
        } else {
            tags::TAGS[field.tag]
        };
        let separator = add_colour(if flags.strip { "=" } else { " = " }, flags.colour);
        let value = if flags.value {
            translate_value(field)
        } else {
            &field.value
        };
        let delimiter = add_colour(&flags.delimiter, flags.colour);
        result + tag + &separator + value + &delimiter
    })
}

fn format_to_summary(input: &[Field], template: &str, value_flag: bool) -> String {
    let mut result = String::from(template);
    for field in input {
        let value = if value_flag {
            translate_value(field)
        } else {
            &field.value
        };
        if !template.is_empty() {
            // Replace tag numbers in template to tag name.
            result = result.replace(&field.tag.to_string(), value);
        }
        if field.tag == 35 {
            let msg_type = tags::MSG_TYPES
                .iter()
                .find(|(msg_type, _)| *msg_type == field.value)
                .expect("Invalid msg type")
                .1;
            result = format!("{} {}", msg_type, result);
        }
    }
    result
}

fn translate_value(field: &Field) -> &str {
    tags::VALUES
        .get(format!("{}-{}", field.tag, field.value).as_str())
        .unwrap_or(&field.value.as_str())
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
        let input = "8=4.4|1=test|55=EUR/USD|10=123";
        let result = parse_fix_msg(input, &get_msg_regex());
        let expected = FixMsg::Full(vec![
            field!(8, "4.4"),
            field!(1, "test"),
            field!(55, "EUR/USD"),
            field!(10, "123"),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_case() {
        let input =
            "25=test|1=aaa|8=4.4|123=Capital|243:log[]efssdfkj39809|55=ETH-USD|101=55:05:22";
        let result = parse_fix_msg(input, &get_msg_regex());
        let expected = FixMsg::Partial(vec![
            field!(25, "test"),
            field!(1, "aaa"),
            field!(8, "4.4"),
            field!(123, "Capital"),
            field!(55, "ETH-USD"),
            field!(101, "55:05:22"),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn format_case() {
        let input = "8=FIX.4.4|1=test|55=ETH/USD|54=1|29999=50";
        let FixMsg::Partial(parsed) = parse_fix_msg(input, &get_msg_regex()) else {
            panic!("Should be a partial FIX message");
        };
        let flags = Options {
            delimiter: String::from("|"),
            colour: false,
            strict: false,
            strip: false,
            summary: None,
            tag: false,
            value: true,
            only_fix: false,
        };
        let result = format_to_string(&parsed, &flags);
        let expected = String::from(
            "BeginString = FIX.4.4|Account = test|Symbol = ETH/USD|Side = Buy|29999 = 50|",
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn tag_case() {
        let input = "symbol? 55";
        let parsed = parse_tags(input, &get_tag_regex());
        assert_eq!(parsed, "symbol? Symbol");
        let input = "54,11,8";
        let parsed = parse_tags(input, &get_tag_regex());
        assert_eq!(parsed, "Side,ClOrdID,BeginString");
    }
}
