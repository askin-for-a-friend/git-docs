mod pages;
mod widgets;

use gtk::{prelude::*, Application, ApplicationWindow};

fn main() {
    let app = Application::new(
        Some("org.git.documents"),
        Default::default(),
    );

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.set_title(Some("Git Docs"));

    window.present();
}
