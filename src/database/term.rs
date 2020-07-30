use std::cmp::Ordering;

#[derive(Debug, Eq)]
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

impl Ord for Term {
    fn cmp(&self, other: &Self) -> Ordering {
        (&self.expression, &self.surface_string, &self.sentence).cmp(&(&other.expression, &other.surface_string, &other.sentence))
    }
}

impl PartialOrd for Term {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Term {
    fn eq(&self, other: &Self) -> bool {
        (&self.expression, &self.surface_string, &self.sentence) == (&other.expression, &other.surface_string, &other.sentence)
    }
}
