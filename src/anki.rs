use crate::Config;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::error::Error;

fn request(action: String, params: Value) -> Value {
    json!({"action": action, "params": params, "version": 6})
}

fn invoke(action: String, params: Value) -> Result<Value, Box<dyn Error>> {
    let request_json = request(action, params);

    let client = reqwest::blocking::Client::new();
    let response = client
        .post("http://localhost:8765")
        .body(request_json.to_string())
        .send()?
        .text()?;

    let response = serde_json::from_str(&response)?;

    // @TODO error handling

    Ok(response)
}

fn url_for_expression(expression: &str, reading: &str) -> (String, String) {
    let url_string = format!(
        "https://assets.languagepod101.com/dictionary/japanese/audiomp3.php?kanji={}&kana={}",
        expression, reading
    );
    let file_string = format!("vocabulist_{}_{}", expression, reading);

    (url_string, file_string)
}

fn verify_fields(field_list: &Vec<Vec<String>>) {
    if field_list.len() != 2 {
        panic!("The configuration data either the fields array or values array is missing.");
    }

    let fields = &field_list[0];
    let values = &field_list[1];

    if fields.len() != values.len() {
        panic!("The configuration data is invalid the fields and values array are not the same length.");
    }
}

fn create_fields(
    field_list: &Vec<Vec<String>>,
    definition: &str,
    expression: &str,
    reading: &str,
    sentence: &str,
) -> Value {
    verify_fields(field_list);
    let field_value_iter = field_list[0].iter().zip(field_list[1].iter());

    // if the reading is blank, set reading to equal expression
    let mut reading = reading;
    if reading == "" {
        reading = expression;
    }

    let mut audio_field_list: Vec<String> = Vec::new();
    let mut field_map: HashMap<String, String> = HashMap::new();
    for (f, v) in field_value_iter {
        let value = match &v.to_lowercase()[..] {
            "audio" => {
                audio_field_list.push(f.to_string());
                ""
            }
            "definition" => definition,
            "expression" => expression,
            "reading" => reading,
            "sentence" => sentence,
            _ => "",
        };

        field_map.insert(f.to_string(), value.to_string());
    }

    json!(field_map)
}

fn create_options(allow_duplicates: bool, duplicate_scope: String) -> Value {
    json!({
            "allowDuplicate": allow_duplicates,
            "duplicateScope": duplicate_scope,
    })
}

fn create_audio_fields(field_list: &Vec<Vec<String>>) -> Value {
    verify_fields(field_list);
    let field_value_iter = field_list[0].iter().zip(field_list[1].iter());

    let mut audio_field_list: Vec<String> = Vec::new();
    for (f, v) in field_value_iter {
        if v.to_lowercase() == "audio" {
            audio_field_list.push(f.to_string());
        }
    }

    json!(audio_field_list)
}

fn create_audio_list(audio_fields: &Value, url_list: &Vec<(String, String)>) -> Vec<Value> {
    let mut audio_list: Vec<Value> = Vec::new();
    for (url, file_name) in url_list.iter() {
        let audio = json!({
            "url": url,
            "filename": file_name,
            "skipHash": "7e2c2f954ef6051373ba916f000168dc",
            "fields": audio_fields
        });

        audio_list.push(audio);
    }

    audio_list
}

fn create_note(
    p: &Config,
    definition: &str,
    expression: &str,
    reading: &str,
    sentence: &str,
    url_list: &Vec<(String, String)>,
) -> Value {
    let anki = p.anki();
    let field_list = anki.fields();

    let deck_name = anki.deck_name();
    let model_name = anki.model_name();
    let fields = create_fields(field_list, definition, expression, reading, sentence);
    let options = create_options(anki.allow_duplicates(), anki.duplicate_scope().to_string());
    let tags = anki.tags();
    let audio_fields = create_audio_fields(field_list);
    let mut audio_list: Vec<Value> = Vec::new();

    if anki.audio() {
        audio_list = create_audio_list(&audio_fields, url_list);
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
}

fn note_id_list(p: &Config) -> Result<Vec<Value>, Box<dyn Error>> {
    let result = invoke(
        "findNotes".to_string(),
        json!({ "query": format!("deck:\"{}\"", p.anki().deck_name()) }),
    )?;

    if !result["result"].is_array() {
        panic!("Response from Anki Connect does not contain an note id array");
    }

    let mut id_list: Vec<Value> = Vec::new();
    for id in result["result"].as_array().unwrap().iter() {
        id_list.push(id.clone());
    }

    Ok(id_list)
}

fn note_info_list_for_id_list(id_list: &Vec<Value>) -> Result<Vec<Value>, Box<dyn Error>> {
    let result = invoke("notesInfo".to_string(), json!({ "notes": id_list }))?;

    if !result["result"].is_array() {
        panic!("Response from Anki Connect does not contain a note info array");
    }

    let mut info_list: Vec<Value> = Vec::new();
    for info in result["result"].as_array().unwrap().iter() {
        info_list.push(info.clone());
    }

    Ok(info_list)
}

fn expression_list_for_info_list(
    p: &Config,
    info_list: &Vec<Value>,
) -> Result<Vec<String>, Box<dyn Error>> {
    let fields = p.anki().fields();
    verify_fields(fields);
    let field_value_iter = fields[0].iter().zip(fields[1].iter());

    let mut expression_field = "".to_string();
    for (f, v) in field_value_iter {
        if v.to_lowercase() == "expression" {
            expression_field = f.to_string();
            break;
        }
    }

    let mut expression_list: Vec<String> = Vec::new();
    for note in info_list.iter() {
        match note.is_object() {
            true => {
                let note = note.as_object().unwrap();
                let fields = note["fields"].as_object().unwrap();
                let expression = fields[&expression_field].as_object().unwrap()["value"]
                    .as_str()
                    .unwrap()
                    .to_string();

                expression_list.push(expression);
            }
            false => panic!("Invalid note in info_list"),
        }
    }

    Ok(expression_list)
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

pub fn insert_note(
    p: &Config,
    definition: &str,
    expression: &str,
    reading: &str,
    sentence: &str,
    url_list: &Vec<(String, String)>,
) -> Result<(), Box<dyn Error>> {
    let params = create_note(p, definition, expression, reading, sentence, url_list);
    invoke("addNote".to_string(), params)?;

    Ok(())
}

pub fn expression_list(p: &Config) -> Result<Vec<String>, Box<dyn Error>> {
    let id_list = note_id_list(&p)?;
    let info_list = note_info_list_for_id_list(&id_list)?;
    let expression_list = expression_list_for_info_list(&p, &info_list)?;

    Ok(expression_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn field_map(
        definition: String,
        expression: String,
        reading: String,
        sentence: String,
    ) -> Value {
        let mut field_map: HashMap<String, String> = HashMap::new();

        field_map.insert("Expression".to_string(), expression);
        field_map.insert("Reading".to_string(), reading);
        field_map.insert("Definition".to_string(), definition);
        field_map.insert("Sentence".to_string(), sentence);

        json!(field_map)
    }

    #[test]
    fn create_fields_all_information() {
        let field_list = vec![
            vec![
                "Expression".to_string(),
                "Reading".to_string(),
                "Definition".to_string(),
                "Sentence".to_string(),
            ],
            vec![
                "expression".to_string(),
                "reading".to_string(),
                "definition".to_string(),
                "sentence".to_string(),
            ],
        ];

        let expression = "塩".to_string();
        let reading = "しお".to_string();
        let definition = "salt (i.e. sodium chloride); common salt; table salt".to_string();
        let sentence = "ここのソースは舐めてみるとちょっと塩っぱい".to_string();

        let fields = field_map(
            definition.clone(),
            expression.clone(),
            reading.clone(),
            sentence.clone(),
        );

        assert_eq!(
            create_fields(&field_list, &definition, &expression, &reading, &sentence),
            fields
        );
    }

    #[test]
    fn create_fields_no_reading() {
        let field_list = vec![
            vec![
                "Expression".to_string(),
                "Reading".to_string(),
                "Definition".to_string(),
                "Sentence".to_string(),
            ],
            vec![
                "expression".to_string(),
                "reading".to_string(),
                "definition".to_string(),
                "sentence".to_string(),
            ],
        ];

        let expression = "塩".to_string();
        let reading = "".to_string();
        let definition = "salt (i.e. sodium chloride); common salt; table salt".to_string();
        let sentence = "ここのソースは舐めてみるとちょっと塩っぱい".to_string();

        let fields = field_map(
            definition.clone(),
            expression.clone(),
            expression.clone(),
            sentence.clone(),
        );

        assert_eq!(
            create_fields(&field_list, &definition, &expression, &reading, &sentence),
            fields
        );
    }

    #[test]
    #[should_panic]
    fn create_fields_should_panic() {
        let field_list = vec![
            vec![
                "Expression".to_string(),
                "Reading".to_string(),
                "Definition".to_string(),
                "Sentence".to_string(),
            ],
            vec![
                "expression".to_string(),
                "reading".to_string(),
                "definition".to_string(),
                "sentence".to_string(),
            ],
        ];

        let expression = "塩".to_string();
        let reading = "しお".to_string();
        let definition = "salt (i.e. sodium chloride); common salt; table salt".to_string();
        let sentence = "ここのソースは舐めてみるとちょっと塩っぱい".to_string();

        let fields = field_map(
            definition.clone(),
            expression.clone(),
            reading.clone(),
            sentence.clone(),
        );

        assert_eq!(create_fields(&field_list, "", "", "", ""), fields);
    }
}
