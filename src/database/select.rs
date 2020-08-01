use rusqlite::{params, Connection, Transaction};
use std::error::Error;

/// select expressions from database
pub fn select_expression(tx: &Transaction, limit: u32) -> Result<Vec<String>, Box<dyn Error>> {
    let query = match limit {
        0 => "SELECT expression FROM expressions \
                ORDER BY frequency DESC;"
            .to_string(),
        _ => format!(
            "SELECT expression FROM expressions \
                ORDER BY frequency DESC \
                LIMIT {};",
            limit
        ),
    };

    let expression_list = expression_list(tx, &query)?;

    Ok(expression_list)
}

/// select excluded expressions from database
pub fn select_expression_excluded(
    tx: &Transaction,
    limit: u32,
) -> Result<Vec<String>, Box<dyn Error>> {
    let query = match limit {
        0 => "SELECT expression FROM expressions \
                WHERE expressions.is_excluded = 1 \
                ORDER BY frequency DESC;"
            .to_string(),
        _ => format!(
            "SELECT expression FROM expressions \
                    WHERE expressions.is_excluded = 1 \
                    ORDER BY frequency DESC
                    LIMIT {};",
            limit
        ),
    };

    let expression_list: Vec<String> = expression_list(tx, &query)?;

    Ok(expression_list)
}

/// select expressions in anki from database
pub fn select_expression_in_anki(
    tx: &Transaction,
    limit: u32,
) -> Result<Vec<String>, Box<dyn Error>> {
    let query = match limit {
        0 => "SELECT expression FROM expressions \
            WHERE expressions.in_anki = 1 \
            ORDER BY frequency DESC;"
            .to_string(),
        _ => format!(
            "SELECT expression FROM expressions \
                    WHERE expressions.in_anki = 1 \
                    ORDER BY frequency DESC
                    LIMIT {};",
            limit
        ),
    };

    let expression_list: Vec<String> = expression_list(tx, &query)?;

    Ok(expression_list)
}

/// check sentence exists in database
pub fn select_sentence_exists(conn: &Connection, sentence: &str) -> Result<bool, Box<dyn Error>> {
    let mut statement = conn.prepare("SELECT sentence FROM sentences WHERE sentence = ?;")?;

    Ok(statement.exists(params![sentence])?)
}

fn expression_list(tx: &Transaction, query: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut statement = tx.prepare(query)?;
    let expression_list: Vec<String> = statement
        .query_map(params![], |row| Ok(row.get(0)?))?
        .filter_map(|term| term.ok())
        .collect();

    Ok(expression_list)
}

#[cfg(test)]
mod tests {
    use super::super::{connection, initialize, transaction, Term};
    use super::*;
    use rusqlite::DropBehavior;
    use std::path::PathBuf;

    #[test]
    fn test_select_expression() -> Result<(), Box<dyn Error>> {
        run_test_select_expression("test_select_expression.db", |tx| {
            let expected_term = vec!["何".to_string()];
            let expected_list = vec!["何".to_string(), "は".to_string(), "名前".to_string()];

            let mut statement = tx.prepare("SELECT count(*) FROM expressions;")?;
            let term_count: i32 = statement
                .query_map(params![], |row| Ok(row.get(0)?))?
                .filter_map(|row| row.ok())
                .collect::<Vec<i32>>()[0];

            statement.finalize()?;

            // result
            let mut result_list = select_expression(&tx, 0)?;
            let mut result_one = select_expression(&tx, 1)?;

            // assert
            assert_eq!(term_count, 3);
            assert_eq!(result_list, expected_list);
            assert_eq!(result_one, expected_term);

            Ok(())
        })?;

        Ok(())
    }

    #[test]
    fn test_select_expression_excluded() -> Result<(), Box<dyn Error>> {
        run_test_select_expression("test_select_expression_excluded.db", |tx| {
            let expected_all = vec!["何".to_string(), "は".to_string()];
            let expected_one = vec!["何".to_string()];
            let excluded_list = vec!["何", "は"];

            let query = "UPDATE expressions SET is_excluded = 1 WHERE expression = ?;";

            for term in excluded_list.iter() {
                tx.execute(query, params![term])?;
            }

            // result
            let result_all = select_expression_excluded(&tx, 0)?;
            let result_one = select_expression_excluded(&tx, 1)?;

            // assert
            assert_eq!(result_all, expected_all);
            assert_eq!(result_one, expected_one);

            Ok(())
        })?;

        Ok(())
    }

    #[test]
    fn test_select_expression_in_anki() -> Result<(), Box<dyn Error>> {
        run_test_select_expression("test_select_expression_in_anki.db", |tx| {
            let expected_all = vec!["何".to_string(), "は".to_string()];
            let expected_one = vec!["何".to_string()];
            let in_anki_list = vec!["何", "は"];

            let query = "UPDATE expressions SET in_anki = 1 WHERE expression = ?;";
            for term in in_anki_list.iter() {
                tx.execute(query, params![term])?;
            }

            // result
            let result_all = select_expression_in_anki(&tx, 0)?;
            let result_one = select_expression_in_anki(&tx, 1)?;

            // assert
            assert_eq!(result_all, expected_all);
            assert_eq!(result_one, expected_one);

            Ok(())
        })?;

        Ok(())
    }

    #[test]
    fn test_select_sentence_list() -> Result<(), Box<dyn Error>> {
        let db_path = setup("test_select_sentence_list.db")?;

        let mut conn = connection(&db_path)?;

        let tx = transaction(&mut conn)?;
        initialize(&tx)?;
        tx.commit()?;

        let sentence_list: Vec<String> = vec![
            "プロ野球は今、客を５０００人まで入れて試合をしています。", 
            "８月からはイベントの客の数を増やしてもいいと国が言っていたため、会場の半分まで客を増やす予定でした。"
        ]
        .iter()
        .map(|sentence| sentence.to_string())
        .collect();

        let query = "INSERT INTO sentences (sentence) VALUES (?);";
        for sentence in sentence_list.iter() {
            conn.execute(query, params![sentence])?;
        }

        let does_exist = select_sentence_exists(&conn, &sentence_list[0])?;
        let does_not_exist = select_sentence_exists(&conn, "Hello World")?;

        tear_down(db_path)?;

        assert!(does_exist == true, "sentence does not exist when it should");
        assert!(
            does_not_exist == false,
            "sentence exists when it should not"
        );

        Ok(())
    }

    fn setup(db_name: &str) -> Result<PathBuf, Box<dyn Error>> {
        // get path to executable
        let test_dir = std::env::current_exe()?
            .parent()
            .unwrap()
            .join("test_database");

        // create tmp directory
        if !test_dir.is_dir() {
            std::fs::create_dir(&test_dir)?;
        }

        let db_path = test_dir.join(db_name);

        Ok(db_path)
    }

    fn run_test_select_expression(
        name: &str,
        f: fn(&Transaction) -> Result<(), Box<dyn Error>>,
    ) -> Result<(), Box<dyn Error>> {
        // setup
        let path = setup(name)?;
        let mut conn = connection(&path)?;

        let tx = transaction(&mut conn)?;
        initialize(&tx)?;

        let mut term_list: Vec<Term> = Vec::new();
        term_list.push(Term::new(
            "名前".to_string(),
            "名詞".to_string(),
            "名前は何ですか".to_string(),
            "名前".to_string(),
        ));
        term_list.push(Term::new(
            "は".to_string(),
            "助詞".to_string(),
            "名前は何ですか".to_string(),
            "は".to_string(),
        ));
        term_list.push(Term::new(
            "は".to_string(),
            "助詞".to_string(),
            "『しんのすけ』という名前はからかいの対象ですか".to_string(),
            "は".to_string(),
        ));
        term_list.push(Term::new(
            "何".to_string(),
            "名詞".to_string(),
            "今のアナウンスは何だったのですか。".to_string(),
            "何".to_string(),
        ));
        term_list.push(Term::new(
            "何".to_string(),
            "名詞".to_string(),
            "名前は何ですか".to_string(),
            "何".to_string(),
        ));
        term_list.push(Term::new(
            "何".to_string(),
            "名詞".to_string(),
            "何時ですか".to_string(),
            "何".to_string(),
        ));

        let query = "INSERT OR IGNORE INTO expressions (expression) VALUES (?) \
                        ON CONFLICT (expression) DO UPDATE SET frequency = frequency + 1;";
        for term in term_list.iter() {
            tx.execute(query, params![term.expression()])?;
        }

        // test
        let safe_tx = std::panic::AssertUnwindSafe(&tx);
        let result = std::panic::catch_unwind(|| {
            f(&safe_tx).unwrap();
        });

        // teardown
        tx.finish()?;
        conn.close().or(Err("Failed to close database"))?;

        std::fs::remove_file(path)?;

        if let Err(err) = result {
            std::panic::resume_unwind(err);
        }

        Ok(())
    }

    fn tear_down(db_path: PathBuf) -> Result<(), Box<dyn Error>> {
        std::fs::remove_file(db_path)?;

        Ok(())
    }
}
