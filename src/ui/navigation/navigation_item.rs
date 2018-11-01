pub trait NavigationItem {

    fn select(&self);
    fn deselect(&self);
}


use gtk::{
    WidgetExt,
    StyleContextExt,
};

impl NavigationItem for gtk::Box {

    fn select(&self) {
        self.get_style_context()
            .unwrap()
            .add_class("item-box");
        self.get_style_context()
            .unwrap()
            .add_class("selected");
    }

    fn deselect(&self) {
        self.get_style_context()
            .unwrap()
            .remove_class("selected");
    }
}