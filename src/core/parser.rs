

/// Represents a part of the  raw input
#[derive(Debug, PartialEq)]
pub(crate) enum InputPart {
    Program(String),
    Command(String),
    ShortArg(String),
    LongArg(String),
    String(String),
}

/// Rppresents the raw input
pub(crate) struct Input {
    raw: String,
    split: Vec<String>,
    lexed: Vec<InputPart>,
}

impl Input {
    fn get_part(raw: &str) -> InputPart {
        match raw {
            arg if arg.starts_with("-") && !arg.starts_with("--") => {
                InputPart::ShortArg(arg.to_string())
            }
            arg if arg.starts_with("--") => InputPart::LongArg(arg.to_string()),
            string if string.contains(' ') => InputPart::String(string.to_string()),
            command => InputPart::Command(command.to_string()),
        }
    }

    pub fn new(raw: &str) -> Self {
        // TODO: handle this properly
        let split = shlex::split(raw).expect("Failed to parse commands");

        let mut lexed = vec![];

        let mut split_i = split.clone().into_iter();

        lexed.push(InputPart::Program(split_i.next().unwrap()));

        for item in split_i {
            lexed.push(Self::get_part(&item))
        }

        Self {
            raw: raw.to_string(),
            split,
            lexed,
        }
    }
}

#[cfg(test)]
mod app_tests {

    use super::*;

    #[test]
    fn shlex_parse() {
        let buf =
            r#"dock command -S --extended --string "This is a string with spaces""#.to_string();

        assert_eq!(
            vec![
                "dock",
                "command",
                "-S",
                "--extended",
                "--string",
                "This is a string with spaces"
            ],
            Input::new(&buf).split
        );
    }

    #[test]
    fn lexer() {
        let buf =
            r#"dock command -S --extended --string "This is a string with spaces""#.to_string();

        assert_eq!(
            vec![
                InputPart::Program("dock".to_string()),
                InputPart::Command("command".to_string()),
                InputPart::ShortArg("-S".to_string()),
                InputPart::LongArg("--extended".to_string()),
                InputPart::LongArg("--string".to_string()),
                InputPart::String("This is a string with spaces".to_string())
            ],
            Input::new(&buf).lexed
        );
    }
}
