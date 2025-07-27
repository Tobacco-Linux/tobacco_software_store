use crate::frontend::build_ui;
use adw::{
    Application,
    prelude::{ApplicationExt, ApplicationExtManual},
};
mod backend;
mod frontend;

fn main() {
    let app = Application::builder()
        .application_id("org.tobaccolinux.software_store")
        .build();

    app.connect_activate(|app| {
        build_ui(app);
    });

    app.run();
}
