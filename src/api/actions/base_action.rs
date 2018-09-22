use std::fmt::Debug;
use serde_traitobject::{
    Serialize,
    Deserialize,
};

pub trait BaseAction: Debug + Serialize + Deserialize {

    /// `true` if Raven's window should 
    /// remain open once all actions are 
    /// completed.
    /// 
    /// If not implemented, returns 
    /// `false` by default.
    fn keep_app_open(&self) -> bool { false }

    /// Runs the action
    // TODO: Create Error type for this.
    fn run(&self) -> Result<(), ()>;
}