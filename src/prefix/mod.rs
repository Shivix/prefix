mod tags;

use clap::ArgMatches;
use regex::Regex;
use std::{
    collections::HashMap,
    io::{self, IsTerminal, Write},
    process,
};

#[derive(Debug, PartialEq, Clone)]
struct Field {
    tag: usize,
    value: String,
}

pub struct Options {
    delimiter: String,
    colour: bool,
    only_fix: bool,
    repeating: bool,
    strict: bool,
    strip: bool,
    summary: Option<String>,
    tag: bool,
    value: bool,
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
    let delimiter;
    let strip;
    if matches.get_flag("porcelain") {
        delimiter = String::from("\x01");
        strip = true;
    } else {
        delimiter = matches.get_one::<String>("delimiter").unwrap().to_string();
        strip = matches.get_flag("strip");
    }
    Options {
        delimiter,
        colour: use_colour,
        only_fix: matches.get_flag("only-fix"),
        repeating: matches.get_flag("repeating"),
        strict: matches.get_flag("strict"),
        strip,
        summary: matches.get_one::<String>("summary").cloned(),
        tag: matches.get_flag("tag"),
        value: matches.get_flag("value"),
    }
}

pub fn get_msg_regex() -> Regex {
    // This regex will only match valid fields and any malformed fields will be ignored.
    // This means its very unlikely for prefix to fail to parse a FIX message.
    Regex::new(r"(?P<tag>[0-9]+)=(?P<value>[^\^\|\x01]+)").unwrap()
}

pub fn get_tag_regex() -> Regex {
    Regex::new(r"[0-9]+").unwrap()
}

pub fn get_summary_regexes(flags: &Options) -> HashMap<String, Regex> {
    let mut summary_regexes = HashMap::<String, Regex>::new();
    if flags.summary.is_some() {
        let template = flags.summary.as_ref().unwrap();
        let re = Regex::new(r"\d+").unwrap();
        for number in re.find_iter(template) {
            let number = number.as_str();
            summary_regexes.insert(
                number.to_string(),
                Regex::new(&format!(r"\b{}\b", number)).unwrap(),
            );
        }
    }
    summary_regexes
}

pub fn run(
    input: &str,
    last_line: bool,
    msg_regex: &Regex,
    tag_regex: &Regex,
    summary_regexes: &HashMap<String, Regex>,
    flags: &Options,
) {
    let mut stdout = io::stdout();

    match parse_fix_msg(input, &msg_regex) {
        FixMsg::Full(parsed) => {
            print_fix_msg(&mut stdout, last_line, &parsed, &summary_regexes, flags);
        }
        FixMsg::Partial(parsed) => {
            if !flags.strict {
                print_fix_msg(&mut stdout, last_line, &parsed, &summary_regexes, flags);
            } else if !flags.only_fix {
                print_non_fix_msg(&mut stdout, input, &tag_regex, flags);
            }
        }
        FixMsg::None => {
            if !flags.only_fix {
                print_non_fix_msg(&mut stdout, input, &tag_regex, flags);
            }
        }
    }
}

fn handle_broken_pipe(result: io::Result<()>) {
    if let Err(e) = result {
        // When piping into certain programs like head, printing to stdout can fail. This is
        // expected and we do not want to panic, instead we terminate cleanly. Prefix is not
        // designed to be used for anything besides printing. And this keeps the behaviour closer
        // to other unix tools that will terminate upon receiving the SIGPIPE (which rust programs ignore by default)
        if e.kind() == io::ErrorKind::BrokenPipe {
            process::exit(0);
        }
        panic!("Error writing to stdout: {}", e);
    }
}

fn print_non_fix_msg(stdout: &mut io::Stdout, line: &str, tag_regex: &Regex, flags: &Options) {
    let result = if flags.tag {
        writeln!(stdout, "{}", parse_tags(line, tag_regex))
    } else {
        writeln!(stdout, "{}", line)
    };
    handle_broken_pipe(result);
}

fn print_fix_msg(
    stdout: &mut io::Stdout,
    last_line: bool,
    fix_msg: &[Field],
    regex_by_tag: &HashMap<String, Regex>,
    flags: &Options,
) {
    let result = if flags.summary.is_some() {
        writeln!(
            stdout,
            "{}",
            format_to_summary(fix_msg, regex_by_tag, flags)
        )
    } else {
        // Avoid adding an empty new line at the bottom of the output.
        if last_line && flags.delimiter == "\n" {
            write!(stdout, "{}", format_to_string(fix_msg, flags))
        } else {
            writeln!(stdout, "{}", format_to_string(fix_msg, flags))
        }
    };
    handle_broken_pipe(result);
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
        let tag = i["tag"]
            .parse()
            .unwrap_or_else(|_| panic!("could not parse tag: {}", &i["tag"]));
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
    let fix_msg = if flags.repeating {
        &combine_repeating_groups(input)
    } else {
        input
    };
    fix_msg.iter().fold(String::new(), |result, field| {
        // Allow custom tags to still be printed without translation
        let tag = if field.tag >= tags::TAGS.len() {
            &field.tag.to_string()
        } else {
            tags::TAGS[field.tag]
        };
        let separator = add_colour(if flags.strip { "=" } else { " = " }, flags.colour);
        let value = if flags.value {
            if flags.repeating {
                &translate_combined_values(field)
            } else {
                translate_value(field)
            }
        } else {
            &field.value
        };
        let delimiter = add_colour(&flags.delimiter, flags.colour);
        result + tag + &separator + value + &delimiter
    })
}

fn format_to_summary(
    input: &[Field],
    regex_by_tag: &HashMap<String, Regex>,
    flags: &Options,
) -> String {
    let template = flags.summary.as_ref().unwrap();
    let mut result = String::from(template);
    for field in input {
        let value = if flags.value {
            if flags.repeating {
                &translate_combined_values(field)
            } else {
                translate_value(field)
            }
        } else {
            &field.value
        };
        if !template.is_empty() {
            let tag = field.tag.to_string();
            if template.contains(&tag) {
                // Use a regex with line boundaries to ensure we don't overwrite partial numbers.
                // Replace tag numbers in template to tag name.
                result = regex_by_tag[tag.as_str()]
                    .replace_all(&result, value)
                    .to_string();
            }
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

fn translate_combined_values(field: &Field) -> String {
    let mut values = String::new();
    for value in field.value.split(',') {
        if !values.is_empty() {
            values.push(',')
        }
        values.push_str(
            tags::VALUES
                .get(format!("{}-{}", field.tag, value).as_str())
                .unwrap_or(&value),
        );
    }
    values
}

fn combine_repeating_groups(input: &[Field]) -> Vec<Field> {
    let mut result = Vec::<Field>::new();
    for field in input {
        if let Some(existing_field) = result.iter_mut().find(|f| f.tag == field.tag) {
            existing_field.value.push_str(&format!(",{}", field.value));
        } else {
            result.push(field.clone());
        }
    }
    result
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
            delimiter: String::from("\n"),
            colour: false,
            only_fix: false,
            repeating: false,
            strict: false,
            strip: false,
            summary: None,
            tag: false,
            value: false,
        };
        let result = format_to_string(&parsed, &flags);
        let expected = String::from(
            "BeginString = FIX.4.4\nAccount = test\nSymbol = ETH/USD\nSide = 1\n29999 = 50\n",
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn format_args_case() {
        let input = "8=FIX.4.4|1=test|55=ETH/USD|54=1|29999=50";
        let FixMsg::Partial(parsed) = parse_fix_msg(input, &get_msg_regex()) else {
            panic!("Should be a partial FIX message");
        };
        let flags = Options {
            delimiter: String::from("|"),
            colour: true,
            only_fix: true,
            repeating: true,
            strict: true,
            strip: true,
            summary: None,
            tag: true,
            value: true,
        };
        let result = format_to_string(&parsed, &flags);
        let expected = String::from(
            "BeginString=FIX.4.4|Account=test|Symbol=ETH/USD|Side=Buy|29999=50|"
                .replace("|", "\x1b[33m|\x1b[0m")
                .replace("=", "\x1b[33m=\x1b[0m"),
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn summary_case() {
        let input = [
            field!(8, "4.4"),
            field!(35, "D"),
            field!(55, "EUR/USD"),
            field!(10, "123"),
        ];
        let flags = Options {
            delimiter: String::from("\n"),
            colour: false,
            only_fix: false,
            repeating: false,
            strict: false,
            strip: false,
            summary: Some(String::from("for 55")),
            tag: false,
            value: false,
        };

        let regex_by_tag =
            HashMap::<String, Regex>::from([(String::from("55"), Regex::new(r"\b55\b").unwrap())]);
        let result = format_to_summary(&input, &regex_by_tag, &flags);
        let expected = String::from("NewOrderSingle for EUR/USD");
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

    #[test]
    fn repeating_case() {
        let input = [
            field!(8, "4.4"),
            field!(55, "EUR/USD"),
            field!(268, "3"),
            field!(270, "1.1"),
            field!(270, "1.2"),
            field!(270, "1.3"),
            field!(271, "1000"),
            field!(271, "2500"),
            field!(271, "5000"),
            field!(10, "123"),
        ];
        let result = combine_repeating_groups(&input);
        let expected = [
            field!(8, "4.4"),
            field!(55, "EUR/USD"),
            field!(268, "3"),
            field!(270, "1.1,1.2,1.3"),
            field!(271, "1000,2500,5000"),
            field!(10, "123"),
        ];
        assert_eq!(result, expected);
    }
}
