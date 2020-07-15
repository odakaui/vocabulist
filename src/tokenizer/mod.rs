pub mod jumanpp;

pub trait Tokenizer {
    fn new() -> Self;

    fn tokenize_sentence_list(&self, sentence_list: &Vec<String>, callback: &dyn Fn()) -> Vec<Token>;
}

#[derive(Debug, Clone, Default)]
pub struct Token {
    token: String,
    pos: Vec<String>,
    sentence: Vec<String>,
    surface_string: Vec<String>,

    reading: Vec<String>,
    definition: Vec<String>,
}

impl Token {
    pub fn new(token: String) -> Token {
        Token {
            token,
            ..Default::default()
        }
    }

    pub fn pos(self, pos: Vec<String>) -> Token {
        Token {
            token: self.token,
            pos: pos,
            sentence: self.sentence,
            surface_string: self.surface_string,
            reading: self.reading,
            definition: self.definition,
        }
    }

    pub fn sentence(self, sentence: Vec<String>) -> Token {
        Token {
            token: self.token,
            pos: self.pos,
            sentence: sentence,
            surface_string: self.surface_string,
            reading: self.reading,
            definition: self.definition,
        }
    }

    pub fn surface_string(self, surface_string: Vec<String>) -> Token {
        Token {
            token: self.token,
            pos: self.pos,
            sentence: self.sentence,
            surface_string: surface_string,
            reading: self.reading,
            definition: self.definition,
        }
    }

    pub fn get_token(&self) -> &str {
        &self.token
    }

    pub fn get_sentence(&self) -> &Vec<String> {
        &self.sentence
    }

    pub fn get_surface_string(&self) -> &Vec<String> {
        &self.surface_string
    }

    pub fn get_pos(&self) -> &Vec<String> {
        &self.pos
    }
}
