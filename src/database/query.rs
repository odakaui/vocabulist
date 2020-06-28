pub mod expression {
    use std::error::Error;

    use rusqlite::{Transaction, params};

    use crate::expression::Expression;

    pub fn update_is_excluded(tx: &Transaction, expression: Expression, is_excluded: bool) -> Result<(), Box<dyn Error>> {
        let is_excluded = if is_excluded { 1 } else { 0 };
        let query = "UPDATE expressions SET is_excluded = ? WHERE expression = ?;";
        tx.execute(&query, params![is_excluded, expression.get_expression()])?;

        Ok(())
    }
}
