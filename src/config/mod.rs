use serde::{Deserialize, Serialize};
use std::path::{PathBuf};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    database_path: String,
    dictionary_path: String,
    backend: String,
    anki: AnkiConnect,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnkiConnect {
    deck_name: String,
    model_name: String,
    allow_duplicates: bool,
    duplicate_scope: String,
    audio: bool,
    fields: Vec<Vec<String>>,
    tags: Vec<String>,
}

impl Config {
    pub fn default(config_directory_path: PathBuf) -> Config {
        let deck_name = "Default".to_string();
        let model_name = "Basic".to_string();
        let allow_duplicates = false;
        let duplicate_scope = "deck".to_string();
        let audio = false;
        let fields = vec![
            vec!["Front".to_string(), "Back".to_string()],
            vec!["expression".to_string(), "definition".to_string()],
        ];
        let tags = vec!["vocabulist".to_string()];
        let anki = AnkiConnect::new(
            deck_name,
            model_name,
            allow_duplicates,
            duplicate_scope,
            audio,
            fields,
            tags,
        );
        let backend = "mecab".to_string();

        Config {
            database_path: config_directory_path
                .join("vocabulist_rs.db")
                .to_str()
                .unwrap()
                .to_string(),
            dictionary_path: config_directory_path
                .join("jmdict.db")
                .to_str()
                .unwrap()
                .to_string(),
            anki: anki,
            backend: backend,
        }
    }

    pub fn database_path(&self) -> &str {
        &self.database_path
    }

    pub fn dictionary_path(&self) -> &str {
        &self.dictionary_path
    }

    pub fn anki(&self) -> &AnkiConnect {
        &self.anki
    }

    pub fn backend(&self) -> &str {
        &self.backend
    }
}

impl AnkiConnect {
    pub fn new(
        deck_name: String,
        model_name: String,
        allow_duplicates: bool,
        duplicate_scope: String,
        audio: bool,
        fields: Vec<Vec<String>>,
        tags: Vec<String>,
    ) -> AnkiConnect {
        AnkiConnect {
            deck_name,
            model_name,
            allow_duplicates,
            duplicate_scope,
            audio,
            fields,
            tags,
        }
    }

    pub fn deck_name(&self) -> &str {
        &self.deck_name
    }

    pub fn model_name(&self) -> &str {
        &self.model_name
    }

    pub fn allow_duplicates(&self) -> bool {
        self.allow_duplicates
    }

    pub fn duplicate_scope(&self) -> &str {
        &self.duplicate_scope
    }

    pub fn audio(&self) -> bool {
        self.audio
    }

    pub fn fields(&self) -> &Vec<Vec<String>> {
        &self.fields
    }

    pub fn tags(&self) -> &Vec<String> {
        &self.tags
    }
}
