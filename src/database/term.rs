pub struct Term {
    expression: String,
    pos: String,
    sentence: String,
    surface_string: String,
}

impl Term {
    pub fn new(expression: String, pos: String, sentence: String, surface_string: String) -> Self {
        Term {
            expression,
            pos,
            sentence,
            surface_string,
        }
    }

    pub fn expression(&self) -> &str {
        &self.expression
    }

    pub fn pos(&self) -> &str {
        &self.pos
    }

    pub fn sentence(&self) -> &str {
        &self.sentence
    }

    pub fn surface_string(&self) -> &str {
        &self.surface_string
    }
}
