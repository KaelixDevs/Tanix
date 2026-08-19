use adw::prelude::*;
use adw::{
    Application,
    ApplicationWindow,
    HeaderBar,
    PreferencesGroup,
    StatusPage,
};

use gtk::{
    Align,
    Box,
    Button,
    Label,
    Orientation,
    Separator,
};

pub fn build_window(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Tanix")
        .default_width(1100)
        .default_height(720)
        .build();

    let header = HeaderBar::builder()
        .show_end_title_buttons(true)
        .build();

    let root = Box::new(Orientation::Vertical, 0);
    root.set_vexpand(true);

    let content = Box::new(Orientation::Vertical, 0);
    content.set_margin_top(32);
    content.set_margin_bottom(32);
    content.set_margin_start(40);
    content.set_margin_end(40);
    content.set_spacing(28);

    let welcome = StatusPage::builder()
        .title("Tanix")
        .description("TONEX + AmpliTube on Linux")
        .build();

    content.append(&welcome);

    let applications = PreferencesGroup::builder()
        .title("Your Rig")
        .description("Launch your Windows guitar software through Tanix")
        .build();

    let tonex_button = Button::with_label("Launch TONEX");
    tonex_button.add_css_class("suggested-action");
    tonex_button.set_hexpand(true);

    let amplitube_button = Button::with_label("Launch AmpliTube 5");
    amplitube_button.set_hexpand(true);

    let app_buttons = Box::new(Orientation::Horizontal, 12);
    app_buttons.set_halign(Align::Fill);
    app_buttons.append(&tonex_button);
    app_buttons.append(&amplitube_button);

    applications.add(&app_buttons);

    content.append(&applications);

    let separator = Separator::new(Orientation::Horizontal);
    content.append(&separator);

    let audio = PreferencesGroup::builder()
        .title("Audio")
        .description("Your Linux audio configuration")
        .build();

    let input = Label::new(Some("Input     Not configured"));
    input.set_xalign(0.0);

    let output = Label::new(Some("Output    Not configured"));
    output.set_xalign(0.0);

    let status = Label::new(Some("● PipeWire status will appear here"));
    status.set_xalign(0.0);

    audio.add(&input);
    audio.add(&output);
    audio.add(&status);

    content.append(&audio);

    let runtime = PreferencesGroup::builder()
        .title("Tanix Runtime")
        .description("Compatibility environment")
        .build();

    let wine = Label::new(Some("Wine       Not configured"));
    wine.set_xalign(0.0);

    let backend = Label::new(Some("Audio      PipeWire"));
    backend.set_xalign(0.0);

    runtime.add(&wine);
    runtime.add(&backend);

    content.append(&runtime);

    root.append(&header);
    root.append(&content);

    window.set_content(Some(&root));
    window.present();
}
