pub trait BaseAction {

    /// `true` if Raven's window should 
    /// remain open once all actions are 
    /// completed.
    /// 
    /// If not implemented, returns 
    /// `false` by default.
    fn keep_app_open(&self) -> bool { false }

    /// Runs the action
    // TODO: Create Error type for this.
    fn run(self) -> Result<(), ()>;
}