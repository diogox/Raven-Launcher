use gtk::{
    Widget,
    WidgetExt,
    Cast,
    FixedExt,
    Label,
};

pub trait FixedUtils {
    fn set_position_in_fixed(&self, pos: (i32, i32));
}

impl FixedUtils for Label {

    fn set_position_in_fixed(&self, pos: (i32, i32)) {
        let parent = self.get_parent()
            .unwrap()
            .downcast::<gtk::Fixed>()
            .unwrap();

        let (x, y) = pos;
        parent.move_(self, -1, 5);
    }
}