use crate::frontend::build_ui;
use adw::{
    Application,
    prelude::{ApplicationExt, ApplicationExtManual},
};
mod frontend;

fn main() {
    let app = Application::builder()
        .application_id("org.tobaccolinux.servicemanager")
        .build();

    app.connect_activate(|app| {
        build_ui(app);
    });

    app.run();
}
