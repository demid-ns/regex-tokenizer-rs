/// Represents a token with its text content and byte positions in the original input
#[derive(Debug)]
pub struct Token {
    /// The text content of the token
    pub text: String,
    /// Starting byte position in the original input
    pub start: usize,
    /// Ending byte position in the original input (exclusive)
    pub end: usize,
}

impl Token {
    /// Creates a new token with the given text and position
    fn new(text: String, start: usize, end: usize) -> Self {
        Self { text, start, end }
    }
}

/// A tokenizer that uses regular expressions to extract tokens from text
#[derive(Debug)]
pub struct RegexTokenizer {
    /// The compiled regular expression used for tokenization
    pub regex: regex::Regex,
}

impl RegexTokenizer {
    /// Creates a new tokenizer with the given regex pattern
    ///
    /// # Arguments
    /// * `pattern` - The regular expression pattern to use for tokenization
    ///
    /// # Panics
    /// Panics if the regex pattern is invalid
    pub fn new(pattern: &str) -> Self {
        let regex = regex::Regex::new(pattern).unwrap();

        Self { regex }
    }

    /// Tokenizes the input text using the configured regex pattern
    ///
    /// # Arguments
    /// * `text` - The text to tokenize
    ///
    /// # Returns
    /// A vector of tokens with their text content and byte positions
    pub fn tokenize(&self, text: &str) -> Vec<Token> {
        let matches: Vec<Token> = self
            .regex
            .find_iter(text)
            .map(|m| Token::new(m.as_str().to_string(), m.start(), m.end()))
            .collect();

        matches
    }
}
