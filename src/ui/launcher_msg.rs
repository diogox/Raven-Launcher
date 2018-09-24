use ::api::items::ExtensionResultItem;

#[derive(Msg)]
pub enum LauncherMsg {
    ShowResults(Vec< ExtensionResultItem >),
    ClearInput,
    Hide,
    Quit,
}