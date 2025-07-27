use adw::{
    Application, HeaderBar, Window,
    gtk::{
        Box, Orientation,
        prelude::{BoxExt, GtkWindowExt},
    },
};
pub fn build_ui(app: &Application) {
    let window = create_window(app);

    window.present();
}

fn create_window(app: &Application) -> Window {
    let main_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .hexpand(true)
        .vexpand(true)
        .build();

    let header = HeaderBar::new();

    let vbox = Box::new(Orientation::Vertical, 0);
    vbox.append(&header);
    vbox.append(&main_box);

    Window::builder()
        .application(app)
        .default_width(1200)
        .default_height(800)
        .title("Software Store")
        .content(&vbox)
        .build()
}
