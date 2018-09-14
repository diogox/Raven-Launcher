pub mod base_event;

pub mod keyword_query;
pub mod item_enter; // TODO: Create trait 'Data'?
// This trait would allow the user to specify a 
// struct that implements it and can be taken into
// ItemEnterEvent. This would eliminate the 'What type
// do I deserialize this into' problem the user will surely
// encounter if it passes in different types of data into this event
// at different point in the extension's code.

//pub mod system_exit; // TODO: not implemented!
//pub mod preferences_update; // TODO: not implemented!
//pub mod preferences; // TODO: not implemented!

pub use self::keyword_query::KeywordQueryEvent;
pub use self::item_enter::ItemEnterEvent;
