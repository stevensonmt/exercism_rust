extern crate regex;

use regex::Regex;

pub fn reply(prompt: &str) -> String {
    let question = Regex::new(r"\?\s*$").unwrap();
    let yelling = Regex::new(r"^[A-Z]+[[:graph:]]*[^a-z]*$|[^a-z][0-9]*!$").unwrap();
    let silence = Regex::new(r"\A\s*\z").unwrap();

    let reply = match silence.is_match(prompt) {
                true => "Fine. Be that way!",
                _ => match yelling.is_match(prompt) {
                    true => "Whoa, chill out!",
                        _ => match question.is_match(prompt) {
                            true => "Sure.",
                            _ => "Whatever.",
                        },
                    },
                };
    String::from(reply)
}
