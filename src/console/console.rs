use crate::LineEnding;
use crate::config::Config;
use crate::console::error::ConsoleError;

impl Console {
    fn trim(&self, input: String) -> Result<String, ()> {
        let last : char = input.chars().last().unwrap();
        return match self.config.line_ending {
            LineEnding::None => Ok(input),
            LineEnding::Optional => {
                if last.eq(&self.config.line_ending_char) {
                    return Ok(self.remove_last_char(&input));
                }
                Ok(input)
            },
            LineEnding::Required => {
                if !last.eq(&self.config.line_ending_char) {
                    return Err(());
                }
                Ok(self.remove_last_char(&input))
            },
        }
    }
    fn remove_last_char(&self, input: &String) -> String {
        return input.chars().take(input.len() - 1).collect();
    }
    pub fn welcome(&self) {
        println!("{}\n", self.config.welcome);
    }
    pub fn running(&self) -> bool {
        return self.keep_open;
    }
    pub fn close(&mut self) {
        self.keep_open = false;
    }
    pub fn next(&mut self, input: String) -> Result<(), ConsoleError> {
        let parsed_result : Result<String, ()> = self.trim(input);
        if parsed_result.is_err() {
            return Err(ConsoleError::InvalidSyntax);
        }

        let parsed : String = parsed_result.unwrap();

        if parsed.is_empty() {
            return Err(ConsoleError::NoInput);
        } else if parsed.eq("exit") {
            self.close();
            return Ok(());
        } else if parsed.eq("fail") {
            return Err(ConsoleError::UnknownInput);
        }

        println!("Interpreting: \"{}\"", parsed);

        return Ok(());
    }
}

pub struct Console {
    pub keep_open: bool,
    pub config : Config,
}
