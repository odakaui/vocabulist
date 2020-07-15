use super::{Token, Tokenizer};
use std::io::Write;
use std::process::{Command, Stdio};

fn jumanpp(sentence: &str) -> String {
    let mut jumanpp = Command::new("jumanpp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start jumanpp process");

    let stdin = jumanpp.stdin.as_mut().expect("Failed to get jumanpp stdin");
    stdin
        .write_all(sentence.as_bytes())
        .expect("Failed to write to jumanpp stdin");

    let jumanpp_output = jumanpp
        .wait_with_output()
        .expect("Failed to wait for jumanpp");

    let token_string = String::from_utf8_lossy(&jumanpp_output.stdout).to_string();

    token_string
}

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

fn tokenize_sentence(sentence: &str) -> Vec<Token> {
    let token_string = jumanpp(sentence);
    let token_list = tokenize_output(sentence, token_string.as_ref());

    token_list
}

pub struct Jumanpp;

impl Tokenizer for Jumanpp {
    fn new() -> Self {
        Jumanpp
    }

    fn tokenize_sentence_list(
        &self,
        sentence_list: &Vec<String>,
        callback: &dyn Fn(),
    ) -> Vec<Token> {
        let mut token_list: Vec<Token> = Vec::new();
        for sentence in sentence_list.iter() {
            let e = tokenize_sentence(sentence);
            token_list.extend(e);

            callback();
        }

        token_list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_output() {
        let sentence = "魅力がたっぷりと詰まっている";
        let output = "魅力 みりょく 魅力 名詞 6 普通名詞 1 * 0 * 0 \"代表表記:魅力/みりょく カテゴリ:抽象物\"\n\
            が が が 助詞 9 格助詞 1 * 0 * 0 NIL\n\
            たっぷり たっぷり たっぷりだ 形容詞 3 * 0 ナノ形容詞 22 語幹 1 \"代表表記:たっぷりだ/たっぷりだ\"\n\
            と と と 助詞 9 格助詞 1 * 0 * 0 NIL\n\
            詰まって つまって 詰まる 動詞 2 * 0 子音動詞ラ行 10 タ系連用テ形 14 \"代表表記:詰まる/つまる ドメイン:料理・食事 自他動詞:他:詰める/つめる\"\n\
            いる いる いる 接尾辞 14 動詞性接尾辞 7 母音動詞 1 基本形 2 \"代表表記:いる/いる\"\n\
            EOS";

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

        let token_list = tokenize_output(sentence, output);

        assert_eq!(token_list, expected_token_list);
    }

    #[test]
    #[should_panic]
    fn test_tokenize_output_should_panic() {
        let sentence = "Hello World";
        let output = "魅力 みりょく 魅力 名詞 6 普通名詞 1 * 0 * 0 \"代表表記:魅力/みりょく カテゴリ:抽象物\"\n\
            が が が 助詞 9 格助詞 1 * 0 * 0 NIL\n\
            たっぷり たっぷり たっぷりだ 形容詞 3 * 0 ナノ形容詞 22 語幹 1 \"代表表記:たっぷりだ/たっぷりだ\"\n\
            と と と 助詞 9 格助詞 1 * 0 * 0 NIL\n\
            詰まって つまって 詰まる 動詞 2 * 0 子音動詞ラ行 10 タ系連用テ形 14 \"代表表記:詰まる/つまる ドメイン:料理・食事 自他動詞:他:詰める/つめる\"\n\
            いる いる いる 接尾辞 14 動詞性接尾辞 7 母音動詞 1 基本形 2 \"代表表記:いる/いる\"\n\
            EOS";

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

        let token_list = tokenize_output(sentence, output);

        assert_eq!(token_list, expected_token_list);
    }
}
