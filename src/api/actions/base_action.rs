use std::sync::{
    Arc,
    Mutex,
};
use std::fmt::Debug;
use serde_traitobject::{
    Serialize,
    Deserialize,
};
use relm_core::Sender;
use downcast_rs::Downcast;

use ::ui::launcher_msg::LauncherMsg;

pub trait BaseAction: Debug + Serialize + Deserialize + Send + Downcast {

    /// `true` if Raven's window should 
    /// remain open once all actions are 
    /// completed.
    /// 
    /// If not implemented, returns 
    /// `false` by default.
    fn keep_app_open(&self) -> bool { false }

    /// Runs the action
    // TODO: Create Error type for this.
    fn run(self, sender: &Arc< Mutex<Sender<LauncherMsg>> >) -> Result<(), ()>;
}

impl_downcast!(BaseAction);