use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;


pub struct ItemEnterEvent {
    data: Option<String>, // ? Make this T?
}

impl ItemEnterEvent {

    pub fn new() -> Self {

        ItemEnterEvent {
            data: None,
        }
    }

    pub fn with_data(data: &str) -> Self {

        ItemEnterEvent {
            data: Some(data.to_owned()), 
        }
    }

    pub fn data(&mut self, data: &str) {

        self.data = Some( data.to_owned() );
    }

    /// Returns whatever was passed to 
    /// the ExtensionCustomAction action.
    pub fn get_data<T>(&self) -> Option<T>
        where T: Serialize + DeserializeOwned {

        if self.data.is_none() {
            return None;
        }
        let data_json = self.data.clone().unwrap();
        let data: Option<T> = serde_json::from_str(&data_json).ok();
        data
    }
}


use super::base_event::BaseEvent;
impl BaseEvent for ItemEnterEvent {}