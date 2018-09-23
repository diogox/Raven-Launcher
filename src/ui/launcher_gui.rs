use ::api::items::result_item::ResultItem;

pub trait LauncherGUI: Send {
    fn show_app(&self);
    fn hide_app(&self);
    fn show_results(&self, results: Vec< Box<ResultItem> >);
    fn clear_results(&self);
    fn clear_input(&self);
}