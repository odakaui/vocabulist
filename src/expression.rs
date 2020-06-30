#[derive(Debug, Clone, Default)]
pub struct Expression {
    expression: String,
    pos: Vec<String>,
    sentence: Vec<String>,
    surface_string: Vec<String>,

    reading: Vec<String>,
    definition: Vec<String>,
}

impl Expression {
    pub fn new(expression: String) -> Expression {
        Expression {
            expression,
            ..Default::default()
        }
    }
    
    pub fn pos(self, pos: Vec<String>) -> Expression {
        Expression {
            expression: self.expression,
            pos: pos,
            sentence: self.sentence,
            surface_string: self.surface_string,
            reading: self.reading,
            definition: self.definition
        }
    }

    pub fn sentence(self, sentence: Vec<String>) -> Expression {
        Expression {
            expression: self.expression,
            pos: self.pos,
            sentence: sentence,
            surface_string: self.surface_string,
            reading: self.reading,
            definition: self.definition
        }
    }

    pub fn surface_string(self, surface_string: Vec<String>) -> Expression {
        Expression {
            expression: self.expression,
            pos: self.pos,
            sentence: self.sentence,
            surface_string: surface_string,
            reading: self.reading,
            definition: self.definition
        }
    }

    pub fn get_expression(&self) -> &str {
        &self.expression
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

