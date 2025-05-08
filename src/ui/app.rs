use glib::clone;
use gtk::{
    AlertDialog, Application,
    gio::prelude::{ApplicationExt, ApplicationExtManual},
    prelude::GtkWindowExt,
};

use super::main_window::FwInfoMainWindow;

const APP_ID: &str = "com.ddyzhang.fwinfo";

pub(crate) fn start_fwinfo() {
    gtk::gio::resources_register_include!("composite_templates.gresource")
        .expect("Failed to register resources.");

    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let window = FwInfoMainWindow::new(app);
    window.set_title(Some("FWInfo"));
    window.set_default_size(300, 100);

    let preflight_success = preflight_check(app, &window);

    if preflight_success {
        window.present();
    }
}

fn preflight_check(app: &Application, window: &FwInfoMainWindow) -> bool {
    let mut maybe_error: Option<String> = None;
    if !framework_lib::smbios::is_framework() {
        maybe_error = Some("This program must be run on a Framework laptop".to_string());
    }
    if !crate::utils::is_elevated() {
        maybe_error = Some("This program must be run as root".to_string());
    }
    match maybe_error {
        Some(err) => {
            let dialog = AlertDialog::builder()
                .modal(true)
                .buttons(["Ok"])
                .default_button(0)
                .message(err)
                .build();

            dialog.choose(
                Some(window),
                None::<&gtk::gio::Cancellable>,
                clone!(
                    #[weak]
                    app,
                    move |_| {
                        app.quit();
                    }
                ),
            );
            false
        }
        None => true,
    }
}
