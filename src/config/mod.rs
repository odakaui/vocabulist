use super::VERSION;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const DATABASE: &str = "vocabulist_rs.db";
const DICTIONARY: &str = "jmdict.db";

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    database_path: PathBuf,
    dictionary_path: PathBuf,
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
    pub fn default(configuration_path: PathBuf) -> Config {
        let dictionary_path = configuration_path.join(DICTIONARY);
        let database_path = configuration_path.join(DATABASE);

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
            database_path: database_path,
            dictionary_path: dictionary_path,
            anki: anki,
            backend: backend,
        }
    }

    pub fn homebrew(configuration_path: PathBuf) -> Config {
        let dictionary_path = PathBuf::from(format!(
            "/usr/local/Cellar/vocabulist/{}/share/{}",
            VERSION, DICTIONARY
        ));
        let database_path = configuration_path.join(DATABASE);
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
            database_path: database_path,
            dictionary_path: dictionary_path,
            anki: anki,
            backend: backend,
        }
    }

    pub fn database_path(&self) -> &PathBuf {
        &self.database_path
    }

    pub fn dictionary_path(&self) -> &PathBuf {
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
