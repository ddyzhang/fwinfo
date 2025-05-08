use glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::{CompositeTemplate, glib};
use gtk::Label;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/ddyzhang/fwinfo/window.ui.xml")]
pub struct FwInfoMainWindow {
    #[template_child]
    pub label: TemplateChild<Label>,
}

#[glib::object_subclass]
impl ObjectSubclass for FwInfoMainWindow {
    const NAME: &'static str = "FwInfoMainWindow";
    type Type = super::FwInfoMainWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for FwInfoMainWindow {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for FwInfoMainWindow {}

impl WindowImpl for FwInfoMainWindow {}

impl ApplicationWindowImpl for FwInfoMainWindow {}
