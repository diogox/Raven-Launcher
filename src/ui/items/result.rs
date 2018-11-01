use gtk;
use gtk::{
    Builder,
    OrientableExt,
    WidgetExt,
    LabelExt,
};

pub struct ResultItemWidget {
    pub item_frame: gtk::Box,
    pub item_box: gtk::EventBox,
    pub item_container: gtk::Box,
    pub icon: gtk::Image,
    pub title: gtk::Label,
    pub description: gtk::Label,
    pub shortcut: gtk::Label,
}

impl ResultItemWidget {

    pub fn new() -> Self {
        
        // Build from glade file
        let item = Builder::new_from_string(include_str!("result_item.glade"));

        // Get Frame
        let item_frame = item.get_object("item-frame")
            .unwrap();

        // Get Box
        let item_box = item.get_object("item-box")
            .unwrap();

        // Get Container
        let item_container = item.get_object("item-container")
            .unwrap();

        // Get Icon
        let icon = item.get_object("item-icon")
            .unwrap();

        // Get Title
        let title = item.get_object("item-name")
            .unwrap();

        // Get Title
        let description = item.get_object("item-descr")
            .unwrap();

        // Get Shortcut
        let shortcut = item.get_object("item-shortcut")
            .unwrap();

        ResultItemWidget {
            item_frame,
            item_box,
            item_container,
            icon,
            title,
            description,
            shortcut,
        }
    }

    pub fn set_title(&self, title: &str) {
        self.title.set_label(title);
    }

    pub fn set_description(&self, description: &str) {
        self.description.set_label(description);
    }
}