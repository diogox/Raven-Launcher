#[derive(Serialize, Deserialize, Debug)]
pub struct KeywordQueryEvent {
    query: String, // TODO: Change this with a 'Query' struct
}

impl KeywordQueryEvent {

    pub fn with_query(query: &str) -> Self {

        KeywordQueryEvent {
            query: query.to_owned(),
        }
    }

    pub fn get_keyword(&self) -> &str {
        //self.query.get_keyword() // TODO
        "TODO"
    }

    pub fn get_query(&self) -> &str {
        &self.query
    }

    pub fn get_argument(&self) -> Option<&str> {
        //self.query.get_argument() // TODO
        None
    }
}


use super::base_event::BaseEvent;
impl BaseEvent for KeywordQueryEvent{}