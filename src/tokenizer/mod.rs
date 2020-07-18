pub mod jumanpp;
pub mod mecab;
pub mod token;
pub mod tokenize;

use std::error::Error;
use token::Token;
use tokenize::Tokenize;

/// a wrapper struct that takes a struct that implements Tokenze
pub struct Tokenizer<T>
where
    T: Tokenize,
{
    tokenizer: T,
}

impl<T> Tokenizer<T>
where
    T: Tokenize,
{
    /// create a new Tokenizer struct
    ///     where T is a struct that implements Tokenize
    pub fn new(tokenizer: T) -> Self
    where
        T: Tokenize,
    {
        Tokenizer { tokenizer }
    }

    /// tokenize a list of sentences and return a list of Token structs
    pub fn tokenize(
        &self,
        sentence_list: &Vec<String>,
        callback: &mut dyn FnMut(),
    ) -> Result<Vec<Token>, Box<dyn Error>> {
        let mut token_list: Vec<Token> = Vec::new();
        for sentence in sentence_list.iter() {
            let list = self.tokenizer.tokenize(sentence)?;
            token_list.extend(list);

            callback();
        }

        Ok(token_list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// mock struct that implements Tokenize
    struct Backend;

    /// implement Tokenize for the Backend struct
    impl Tokenize for Backend {
        /// implement tokenize for the Backend struct
        fn tokenize(&self, sentence: &str) -> Result<Vec<Token>, Box<dyn Error>> {
            let mut token_list: Vec<Token> = Vec::new();
            match sentence {
                "魅力がたっぷりと詰まっている" => {
                    token_list.push(
                        Token::new("魅力".to_string())
                            .pos(vec!["名詞".to_string()])
                            .sentence(vec!["魅力がたっぷりと詰まっている".to_string()])
                            .surface_string(vec!["魅力".to_string()]),
                    );
                    token_list.push(
                        Token::new("が".to_string())
                            .pos(vec!["助詞".to_string()])
                            .sentence(vec!["魅力がたっぷりと詰まっている".to_string()])
                            .surface_string(vec!["が".to_string()]),
                    );
                    token_list.push(
                        Token::new("たっぷりだ".to_string())
                            .pos(vec!["形容詞".to_string()])
                            .sentence(vec!["魅力がたっぷりと詰まっている".to_string()])
                            .surface_string(vec!["たっぷり".to_string()]),
                    );
                    token_list.push(
                        Token::new("と".to_string())
                            .pos(vec!["助詞".to_string()])
                            .sentence(vec!["魅力がたっぷりと詰まっている".to_string()])
                            .surface_string(vec!["と".to_string()]),
                    );
                    token_list.push(
                        Token::new("詰まる".to_string())
                            .pos(vec!["動詞".to_string()])
                            .sentence(vec!["魅力がたっぷりと詰まっている".to_string()])
                            .surface_string(vec!["詰まって".to_string()]),
                    );
                    token_list.push(
                        Token::new("いる".to_string())
                            .pos(vec!["接尾辞".to_string()])
                            .sentence(vec!["魅力がたっぷりと詰まっている".to_string()])
                            .surface_string(vec!["いる".to_string()]),
                    );

                    Ok(token_list)
                }
                "はるさんハウスはどこですか" => {
                    token_list.push(
                        Token::new("はる".to_string())
                            .pos(vec!["名詞".to_string()])
                            .sentence(vec!["はるさんハウスはどこですか".to_string()])
                            .surface_string(vec!["はる".to_string()]),
                    );
                    token_list.push(
                        Token::new("さん".to_string())
                            .pos(vec!["接尾辞".to_string()])
                            .sentence(vec!["はるさんハウスはどこですか".to_string()])
                            .surface_string(vec!["さん".to_string()]),
                    );
                    token_list.push(
                        Token::new("ハウス".to_string())
                            .pos(vec!["名詞".to_string()])
                            .sentence(vec!["はるさんハウスはどこですか".to_string()])
                            .surface_string(vec!["ハウス".to_string()]),
                    );
                    token_list.push(
                        Token::new("は".to_string())
                            .pos(vec!["助詞".to_string()])
                            .sentence(vec!["はるさんハウスはどこですか".to_string()])
                            .surface_string(vec!["は".to_string()]),
                    );
                    token_list.push(
                        Token::new("どこ".to_string())
                            .pos(vec!["指示詞".to_string()])
                            .sentence(vec!["はるさんハウスはどこですか".to_string()])
                            .surface_string(vec!["どこ".to_string()]),
                    );
                    token_list.push(
                        Token::new("だ".to_string())
                            .pos(vec!["判定詞".to_string()])
                            .sentence(vec!["はるさんハウスはどこですか".to_string()])
                            .surface_string(vec!["です".to_string()]),
                    );
                    token_list.push(
                        Token::new("か".to_string())
                            .pos(vec!["助詞".to_string()])
                            .sentence(vec!["はるさんハウスはどこですか".to_string()])
                            .surface_string(vec!["か".to_string()]),
                    );

                    Ok(token_list)
                }
                _ => Ok(token_list),
            }
        }
    }

    /// return a list of tokens to compare to the result of tokenze
    fn expected_token_list() -> Vec<Token> {
        let mut expected_token_list: Vec<Token> = Vec::new();
        expected_token_list.push(
            Token::new("魅力".to_string())
                .pos(vec!["名詞".to_string()])
                .sentence(vec!["魅力がたっぷりと詰まっている".to_string()])
                .surface_string(vec!["魅力".to_string()]),
        );
        expected_token_list.push(
            Token::new("が".to_string())
                .pos(vec!["助詞".to_string()])
                .sentence(vec!["魅力がたっぷりと詰まっている".to_string()])
                .surface_string(vec!["が".to_string()]),
        );
        expected_token_list.push(
            Token::new("たっぷりだ".to_string())
                .pos(vec!["形容詞".to_string()])
                .sentence(vec!["魅力がたっぷりと詰まっている".to_string()])
                .surface_string(vec!["たっぷり".to_string()]),
        );
        expected_token_list.push(
            Token::new("と".to_string())
                .pos(vec!["助詞".to_string()])
                .sentence(vec!["魅力がたっぷりと詰まっている".to_string()])
                .surface_string(vec!["と".to_string()]),
        );
        expected_token_list.push(
            Token::new("詰まる".to_string())
                .pos(vec!["動詞".to_string()])
                .sentence(vec!["魅力がたっぷりと詰まっている".to_string()])
                .surface_string(vec!["詰まって".to_string()]),
        );
        expected_token_list.push(
            Token::new("いる".to_string())
                .pos(vec!["接尾辞".to_string()])
                .sentence(vec!["魅力がたっぷりと詰まっている".to_string()])
                .surface_string(vec!["いる".to_string()]),
        );
        expected_token_list.push(
            Token::new("はる".to_string())
                .pos(vec!["名詞".to_string()])
                .sentence(vec!["はるさんハウスはどこですか".to_string()])
                .surface_string(vec!["はる".to_string()]),
        );
        expected_token_list.push(
            Token::new("さん".to_string())
                .pos(vec!["接尾辞".to_string()])
                .sentence(vec!["はるさんハウスはどこですか".to_string()])
                .surface_string(vec!["さん".to_string()]),
        );
        expected_token_list.push(
            Token::new("ハウス".to_string())
                .pos(vec!["名詞".to_string()])
                .sentence(vec!["はるさんハウスはどこですか".to_string()])
                .surface_string(vec!["ハウス".to_string()]),
        );
        expected_token_list.push(
            Token::new("は".to_string())
                .pos(vec!["助詞".to_string()])
                .sentence(vec!["はるさんハウスはどこですか".to_string()])
                .surface_string(vec!["は".to_string()]),
        );
        expected_token_list.push(
            Token::new("どこ".to_string())
                .pos(vec!["指示詞".to_string()])
                .sentence(vec!["はるさんハウスはどこですか".to_string()])
                .surface_string(vec!["どこ".to_string()]),
        );
        expected_token_list.push(
            Token::new("だ".to_string())
                .pos(vec!["判定詞".to_string()])
                .sentence(vec!["はるさんハウスはどこですか".to_string()])
                .surface_string(vec!["です".to_string()]),
        );
        expected_token_list.push(
            Token::new("か".to_string())
                .pos(vec!["助詞".to_string()])
                .sentence(vec!["はるさんハウスはどこですか".to_string()])
                .surface_string(vec!["か".to_string()]),
        );

        expected_token_list
    }

    /// assert that Tokenizer.tokenize returns the correct output
    #[test]
    fn test_tokenize() {
        let sentence_list = vec![
            "魅力がたっぷりと詰まっている".to_string(),
            "はるさんハウスはどこですか".to_string(),
        ];

        let expected_token_list = expected_token_list();

        let backend = Backend;
        let tokenizer = Tokenizer::new(backend);

        let mut called = 0;
        let mut callback = || called += 1;

        let token_list = tokenizer
            .tokenize(&sentence_list, &mut callback)
            .expect("Failed to unwrap token_list");

        assert_eq!(token_list, expected_token_list);
        assert_eq!(called, 2);
    }
}
