use super::{Token, Tokenize};
use std::error::Error;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// tokenize a sentence using the jumanpp binary at path
fn jumanpp(path: &PathBuf, sentence: &str) -> Result<String, Box<dyn Error>> {
    let mut jumanpp = Command::new(path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    jumanpp
        .stdin
        .as_mut()
        .ok_or("Failed to open jumanpp stdio")?
        .write_all(sentence.as_bytes())?;

    let jumanpp_output = jumanpp.wait_with_output()?;

    let token_string = String::from_utf8_lossy(&jumanpp_output.stdout).to_string();

    Ok(token_string)
}

/// convert the output from jumanpp to a list of Token structs
fn tokenize_output(sentence: &str, output: &str) -> Vec<Token> {
    let output_list: Vec<Vec<&str>> = output.lines().map(|x| x.split(' ').collect()).collect();

    let mut token_list: Vec<Token> = Vec::new();
    for token in output_list.iter() {
        if token[0] != "EOS" && token[0] != "@" && token[3] != "特殊" {
            let surface_string = vec![token[0].to_string()];
            let pos = vec![token[3].to_string()];
            let sentence = vec![sentence.to_string()];
            let token = Token::new(token[2].to_string())
                .pos(pos)
                .sentence(sentence)
                .surface_string(surface_string);

            token_list.push(token);
        }
    }

    token_list
}

/// the Jumanpp struct
pub struct Jumanpp {
    path: PathBuf,
}

impl Jumanpp {
    /// the Jumanpp constructor
    pub fn new(path: PathBuf) -> Self {
        Jumanpp { path }
    }
}

impl Tokenize for Jumanpp {
    /// implement the required method for Tokenize
    fn tokenize(&self, sentence: &str) -> Result<Vec<Token>, Box<dyn Error>> {
        let token_string = jumanpp(&self.path, sentence)?;
        let token_list = tokenize_output(sentence, token_string.as_ref());

        Ok(token_list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// create the expected token list
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

        expected_token_list
    }

    /// check the output of the tokenize method
    #[test]
    #[ignore]
    fn test_tokenize() {
        let sentence = "魅力がたっぷりと詰まっている";
        let jumanpp = Jumanpp::new(PathBuf::from("jumanpp"));

        let token_list = jumanpp
            .tokenize(sentence)
            .expect("Failed to tokenize sentence");

        assert_eq!(token_list, expected_token_list());
    }
}
