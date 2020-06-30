use std::error::Error;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::Preference;

// #[derive(Serialize, Deserialize)]
// struct Note {
//     deckName: String,
//     modelName: String,
//     fields: Fields,
//     options: Options,
//     tags: Vec<String>,
//     audio: Option<Vec<Audio>>,
// }
// 
// impl Note {
//     fn new(fields: Fields, options: Options, audio: Option<Vec<Audio>>) -> Note {
//         Note {
//             deckName: "Default".to_string(),
//             modelName: "Vocabulist".to_string(),
//             fields,
//             options,
//             tags: vec!["vocabulist".to_string()],
//             audio,
//         }
//     }
// }
// 
// #[derive(Serialize, Deserialize)]
// struct Audio {
//     url: String,
//     filename: String,
//     skipHash: String,
//     fields: Vec<String>
// }
// 
// #[derive(Serialize, Deserialize)]
// struct Options {
//     allowDuplicate: bool,
//     duplicateScope: String,
// }
// 
// impl Options {
//     fn new() -> Options {
//         Options {
//             allowDuplicate: false,
//             duplicateScope: "deck".to_string()
//         }
//     }
// }
// 
// #[derive(Serialize, Deserialize)]
// struct Fields {
//     Definition: String,
//     Expression: String,
//     Reading: String,
//     Sentence: String
// }
// 
// impl Fields {
//     fn new(definition: String, expression: String, reading: String, sentence: String) -> Fields {
//         Fields {
//             Definition: definition,
//             Expression: expression,
//             Reading: reading,
//             Sentence: sentence
//         }
//     }
// }

fn request(action: String, params: Value) -> Value {
    json!({"action": action, "params": params, "version": 6})
}

fn invoke(action: String, params: Value) -> Result<Value, Box<dyn Error>> {
    let request_json = request(action, params);

    let client = reqwest::blocking::Client::new();
    let response = client.post("http://localhost:8765")
        .body(request_json.to_string())
        .send()?
        .text()?;

    let response = serde_json::from_str(&response)?;

    Ok(response)
}

pub fn create_url_list(expression: &str, reading_list: &Vec<String>) -> Vec<String> {
    let mut url_list = Vec::new();
    for reading in reading_list.iter() {
        let url_string = format!("https://assets.languagepod101.com/dictionary/japanese/audiomp3.php?kanji={}&kana={}", expression, reading);
        
        url_list.push(url_string);
    }

    url_list
}

pub fn insert_note(p: &Preference, definition: &str, expression: &str, reading: &str, sentence: &str, url_list: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let params = json!({
            "note": {
                "deckName": "Default",
                "modelName": "Vocabulist",
                "fields": {
                    "Definition": definition,
                    "Expression": expression,
                    "Reading": reading,
                    "Sentence": sentence,
                },
                "options": {
                    "allowDuplicate": false,
                    "duplicateScope": "deck"
                },
                "tags": [
                    "vocabulist"
                ],
            }
        });

    let response = invoke("addNote".to_string(), params)?;

    println!("{}", response);

    Ok(())



}
