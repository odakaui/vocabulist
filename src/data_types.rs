#[derive(Debug)]
pub struct Expression {
    expression: String,
    pos: Option<Pos>,
    sentence: Option<Sentence>,
    surface_string: Option<SurfaceString>,

    reading: Option<Reading>,
    definition: Option<Definition>,
}

impl Expression {
    pub fn new(expression: String, pos: Option<Pos>, sentence: Option<Sentence>, surface_string: Option<SurfaceString>, reading: Option<Reading>, definition: Option<Definition>) -> Expression {
        Expression {
            expression,
            surface_string,
            reading,
            pos,
            sentence,
            definition,
        }
    }

    pub fn get_expression(&self) -> &str {
        &self.expression
    }

    pub fn get_sentence(&self) -> &Option<Sentence> {
        &self.sentence
    }

    pub fn get_surface_string(&self) -> &Option<SurfaceString> {
        &self.surface_string
    }

    pub fn get_pos(&self) -> &Option<Pos> {
        &self.pos
    }
}

#[derive(Debug)]
pub struct SurfaceString(pub Vec<String>); 

#[derive(Debug)]
pub struct Reading(pub Vec<String>);

#[derive(Debug)]
pub struct Pos(pub Vec<String>);

#[derive(Debug)]
pub struct Sentence(pub Vec<String>);

#[derive(Debug)]
pub struct Definition(pub Vec<String>);
