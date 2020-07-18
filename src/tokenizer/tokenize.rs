use super::Token;
use std::error::Error;

pub trait Tokenize {
    fn tokenize(&self, sentence: &str) -> Result<Vec<Token>, Box<dyn Error>>;
}

impl Tokenize for Box<dyn Tokenize> {
    fn tokenize(&self, sentence: &str) -> Result<Vec<Token>, Box<dyn Error>> {
        self.as_ref().tokenize(sentence)
    }
}
