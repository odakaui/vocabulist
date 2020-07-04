use std::error::Error;
use std::collections::HashMap;
use serde_json::{Value, json};
use crate::Config;

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

    // @TODO error handling

    Ok(response)
}

fn url_for_expression(expression: &str, reading: &str) -> (String, String) {
        let url_string = format!("https://assets.languagepod101.com/dictionary/japanese/audiomp3.php?kanji={}&kana={}", expression, reading);
        let file_string = format!("vocabulist_{}_{}", expression, reading);

        (url_string, file_string)
}

fn create_note(p: &Config, definition: &str, expression: &str, reading: &str, sentence: &str, url_list: &Vec<(String, String)>) -> Value {
    let anki = p.anki();
    let deck_name = anki.deck_name();
    let model_name = anki.model_name();

    // todo parse note fields from config object
    let field_list = anki.fields();

    if field_list.len() != 2 {
        panic!("The configuration data either the fields array or values array is missing.");
    }

    let fields = &field_list[0];
    let values = &field_list[1];

    if fields.len() != values.len() {
        panic!("The configuration data is invalid the fields and values array are not the same length.");
    }

    let field_value_iter = fields.iter().zip(values.iter());
    let mut audio_field_list: Vec<String> = Vec::new();
    let mut field_map: HashMap<String, String> = HashMap::new(); 
    for (f, v) in field_value_iter {
        let value = match &v.to_lowercase()[..] {
            "audio" => {
                audio_field_list.push(f.to_string());
                ""
            },
            "definition" => definition,
            "expression" => expression,
            "reading" => reading,
            "sentence" => sentence,
            _ => ""
        };

        field_map.insert(f.to_string(), value.to_string());
    }

    let fields = json!(field_map);

    let options = json!({
            "allowDuplicate": anki.allow_duplicates(),
            "duplicateScope": anki.duplicate_scope(),
        });
    
    let tags = anki.tags();

    let audio_field_list = audio_field_list;

    match p.anki().audio() {
        false => {
            json!({
                    "note": {
                        "deckName": deck_name,
                        "modelName": model_name,
                        "fields": fields,
                        "options": options,
                        "tags": tags,
                    }
                })
        },
        true => {
            let mut audio_list: Vec<Value> = Vec::new();
            for (url, file_name) in url_list.iter() {
                let audio = json!({
                        "url": url,
                        "filename": file_name,
                        "skipHash": "7e2c2f954ef6051373ba916f000168dc",
                        "fields": audio_field_list
                    });

                audio_list.push(audio);
            }

            json!({
                    "note": {
                        "deckName": deck_name,
                        "modelName": model_name,
                        "fields": fields,
                        "options": options,
                        "tags": tags,
                        "audio": audio_list
                    }
                })
        },
    }
}

pub fn create_url_list(expression: &str, reading_list: &Vec<String>) -> Vec<(String, String)> {
    let mut url_list: Vec<(String, String)> = Vec::new();
    match reading_list.len() {
        0 => url_list.push(url_for_expression(expression, expression)),
        _ => {
            for reading in reading_list.iter() {
                url_list.push(url_for_expression(expression, reading));
            }
        }
    }

    url_list
}

pub fn insert_note(p: &Config, definition: &str, expression: &str, reading: &str, sentence: &str, url_list: &Vec<(String, String)>) -> Result<(), Box<dyn Error>> {
    let params = create_note(p, definition, expression, reading, sentence, url_list);
    invoke("addNote".to_string(), params)?;

    Ok(())

        


}

