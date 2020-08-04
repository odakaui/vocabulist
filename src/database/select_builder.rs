pub enum OrderBy {
    Frequency,
    Expression,
    Pos,
    Sentence,
    SurfaceString,
}

pub struct SelectBuilder {
    query: String,
    join_list: Vec<String>,
    filter: Vec<String>,
    limit: u32,
    order_by: OrderBy,
    asc: bool,
}

impl SelectBuilder {
    pub fn new() -> Self {
        SelectBuilder {
            query: "SELECT expression, pos, sentence, surface_string FROM expressions".to_string(),
            join_list: vec![
                "JOIN expressions_pos_sentences_surface_strings ON expression_id = expressions.id"
                    .to_string(),
                "JOIN pos ON pos.id = pos_id".to_string(),
                "JOIN sentences ON sentences.id = sentence_id".to_string(),
                "JOIN surface_strings ON surface_strings.id = surface_string_id".to_string(),
            ],
            filter: vec![],
            limit: 0,
            order_by: OrderBy::Frequency,
            asc: false,
        }
    }

    pub fn limit(self, limit: u32) -> Self {
        SelectBuilder {
            query: self.query,
            join_list: self.join_list,
            filter: self.filter,
            limit: limit,
            order_by: self.order_by,
            asc: self.asc,
        }
    }

    pub fn query(&self) -> String {
        let mut list: Vec<String> = Vec::new();

        // base query
        let base_query = self.query.to_string();
        list.push(base_query);

        // joins
        let join_string = (&self.join_list).join(" ");
        list.push(join_string);

        if self.limit > 0 {
            list.push(format!("LIMIT {}", self.limit));
        }

        format!("{};", list.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query() {
        let expected_query = "SELECT expression, pos, sentence, surface_string FROM expressions \
            JOIN expressions_pos_sentences_surface_strings ON expression_id = expressions.id \
            JOIN pos ON pos.id = pos_id \
            JOIN sentences ON sentences.id = sentence_id \
            JOIN surface_strings ON surface_strings.id = surface_string_id;";

        let select = SelectBuilder::new();
        let query = select.query();

        assert_eq!(query, expected_query);
    }

    #[test]
    fn test_limit() {
        let limit_zero = "SELECT expression, pos, sentence, surface_string FROM expressions \
            JOIN expressions_pos_sentences_surface_strings ON expression_id = expressions.id \
            JOIN pos ON pos.id = pos_id \
            JOIN sentences ON sentences.id = sentence_id \
            JOIN surface_strings ON surface_strings.id = surface_string_id;";

        let limit_ten = "SELECT expression, pos, sentence, surface_string FROM expressions \
            JOIN expressions_pos_sentences_surface_strings ON expression_id = expressions.id \
            JOIN pos ON pos.id = pos_id \
            JOIN sentences ON sentences.id = sentence_id \
            JOIN surface_strings ON surface_strings.id = surface_string_id \
            LIMIT 10;";

        let limit_hundred = "SELECT expression, pos, sentence, surface_string FROM expressions \
            JOIN expressions_pos_sentences_surface_strings ON expression_id = expressions.id \
            JOIN pos ON pos.id = pos_id \
            JOIN sentences ON sentences.id = sentence_id \
            JOIN surface_strings ON surface_strings.id = surface_string_id \
            LIMIT 100;";

        assert_eq!(SelectBuilder::new().query(), limit_zero);
        assert_eq!(SelectBuilder::new().limit(0).query(), limit_zero);

        assert_eq!(SelectBuilder::new().limit(10).query(), limit_ten);

        assert_eq!(SelectBuilder::new().limit(100).query(), limit_hundred);
    }
}
