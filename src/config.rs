use crate::console::lineending::LineEnding;

pub struct Config {
    pub welcome: &'static str,
    pub line_ending_char: char,
    pub line_ending: LineEnding,
}
