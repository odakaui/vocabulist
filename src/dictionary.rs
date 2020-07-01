use std::error::Error;
use rusqlite::{Connection, OpenFlags, params};

const SELECT_SENSE_ID_FOR_KEB: &str =   "SELECT sense.id FROM sense INNER JOIN entry ON entry.ent_seq = sense.ent_seq INNER JOIN entry_keb ON entry_keb.ent_seq = entry.ent_seq INNER JOIN keb ON keb.id = entry_keb.keb_id WHERE keb = ?;";

const SELECT_SENSE_ID_FOR_REB: &str =   "SELECT sense.id FROM sense INNER JOIN entry ON entry.ent_seq = sense.ent_seq INNER JOIN entry_reb ON entry_reb.ent_seq = entry.ent_seq INNER JOIN reb ON reb.id = entry_reb.reb_id WHERE reb = ?;";

const SELECT_GLOSS_FOR_SENSE_ID: &str = "SELECT gloss FROM gloss INNER JOIN sense_gloss ON sense_gloss.gloss_id = gloss.id WHERE sense_gloss.sense_id = ?;";

const SELECT_POS_FOR_SENSE_ID: &str =   "SELECT pos FROM pos INNER JOIN sense_pos ON sense_pos.pos_id = pos.id WHERE sense_pos.sense_id = ?;";

const SELECT_READING_FOR_KEB: &str = "SELECT reb from reb INNER JOIN entry_reb ON entry_reb.reb_id = reb.id INNER JOIN entry ON entry.ent_seq = entry_reb.ent_seq INNER JOIN entry_keb ON entry_keb.ent_seq = entry.ent_seq INNER JOIN keb ON keb.id = entry_keb.keb_id WHERE keb = ? ORDER BY reb.id ASC;";


pub struct DictionaryDefinition {
    definition_list: Vec<String>,
    pos_list: Vec<String>
}

impl DictionaryDefinition {
    fn new(definition_list: Vec<String>, pos_list: Vec<String>) -> DictionaryDefinition {
        DictionaryDefinition {
            definition_list,
            pos_list,
        }
    }

    fn definition_list(&self) -> &Vec<String> {
        &self.definition_list
    }

    fn pos_list(&self) -> &Vec<String> {
        &self.pos_list
    }
}


pub fn connect(path: &str) -> Result<Connection, Box<dyn Error>> {
    let conn = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;

    Ok(conn)
}

pub fn select_definition_for_expression(conn: &Connection, expression: &str) -> Result<(Vec<DictionaryDefinition>, bool), Box<dyn Error>> {
    let mut is_specific = true;
    let params = params![expression];

    // get a list of sense ids for a given keb
    let mut statement = conn.prepare(SELECT_SENSE_ID_FOR_KEB)?;
    let mut id_list: Vec<i32> = statement.query_map(params, |row| Ok(row.get(0)?))?
        .map(|x| x.unwrap())
        .collect();

    // if the keb query returned 0 rows, try the reb query
    if id_list.len() == 0 {
        is_specific = false;
        let mut statement = conn.prepare(SELECT_SENSE_ID_FOR_REB)?;
        id_list = statement.query_map(params, |row| Ok(row.get(0)?))?
            .map(|x| x.unwrap())
            .collect();
    }

    let mut select_gloss = conn.prepare(SELECT_GLOSS_FOR_SENSE_ID)?;
    let mut select_pos = conn.prepare(SELECT_POS_FOR_SENSE_ID)?;

    let mut definition_list: Vec<DictionaryDefinition> = Vec::new();
    for id in id_list {
        let gloss_list: Vec<String> = select_gloss.query_map(params![id], |row| Ok(row.get(0)?))?
            .map(|x| x.unwrap())
            .collect();

        let pos_list: Vec<String> = select_pos.query_map(params![id], |row| Ok(row.get(0)?))?
            .map(|x| x.unwrap())
            .collect();

        definition_list.push(DictionaryDefinition::new(gloss_list, pos_list));
    }

    Ok((definition_list, is_specific))
}

pub fn filter_definition_with_pos_list(definition_list: &Vec<DictionaryDefinition>, allowed_pos_list: &Vec<String>) -> (Vec<Vec<String>>, bool) {
    let mut is_specific = true;
    let mut cached_pos_list: &Vec<String> = &Vec::new();
    let mut filtered_definition_list: Vec<Vec<String>> = Vec::new();
    for definition in definition_list.iter() {
        let definition_list = definition.definition_list();
        let mut pos_list = definition.pos_list();

        if pos_list.len() == 0 {
            pos_list = cached_pos_list
        }

        for pos in pos_list.iter() {
            if allowed_pos_list.iter().any(|x| x == pos) {
                filtered_definition_list.push(definition_list.iter().cloned().collect());
            }
        }

        cached_pos_list = pos_list
    }

    // if filtered_definition_list is empty then we return all of the definitions
    // set is_specific to false because there was no definition for the given pos list
    if filtered_definition_list.len() == 0 {
        is_specific = false;
        for definition in definition_list.iter() {
            let definition_list = definition.definition_list();
            filtered_definition_list.push(definition_list.iter().cloned().collect());
        }
    }

    (filtered_definition_list, is_specific)
}

pub fn select_reading_for_expression(conn: &Connection, expression: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let params = params![expression];

    let mut statement = conn.prepare(SELECT_READING_FOR_KEB)?;
    let reading_list: Vec<String> = statement.query_map(params, |row| Ok(row.get(0)?))?
        .map(|x| x.unwrap())
        .collect();

    Ok(reading_list)
}
