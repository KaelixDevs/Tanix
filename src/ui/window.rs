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

use crate::audio::AudioStatus;

pub fn build_window(app: &Application, audio_status: AudioStatus) {
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

    // ---------------------------------------------------------
    // Welcome
    // ---------------------------------------------------------

    let welcome = StatusPage::builder()
        .title("Tanix")
        .description("TONEX + AmpliTube on Linux")
        .build();

    content.append(&welcome);

    // ---------------------------------------------------------
    // Applications
    // ---------------------------------------------------------

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

    // ---------------------------------------------------------
    // Separator
    // ---------------------------------------------------------

    let separator = Separator::new(Orientation::Horizontal);
    content.append(&separator);

    // ---------------------------------------------------------
    // Audio
    // ---------------------------------------------------------

    let audio = PreferencesGroup::builder()
        .title("Audio")
        .description("Your Linux audio configuration")
        .build();

    let (input_text, output_text, status_text) = match audio_status {
        AudioStatus::Connected { inputs, outputs } => {
            let input_text = inputs
                .first()
                .map(|device| format!("Input     {}", device.name))
                .unwrap_or_else(|| {
                    "Input     No input device detected".to_string()
                });

            let output_text = outputs
                .first()
                .map(|device| format!("Output    {}", device.name))
                .unwrap_or_else(|| {
                    "Output    No output device detected".to_string()
                });

            (
                input_text,
                output_text,
                "● PipeWire connected".to_string(),
            )
        }

        AudioStatus::Failed(error) => (
            "Input     Unable to detect devices".to_string(),
            "Output    Unable to detect devices".to_string(),
            format!("● PipeWire error: {error}"),
        ),
    };

    let input = Label::new(Some(&input_text));
    input.set_xalign(0.0);

    let output = Label::new(Some(&output_text));
    output.set_xalign(0.0);

    let status = Label::new(Some(&status_text));
    status.set_xalign(0.0);

    audio.add(&input);
    audio.add(&output);
    audio.add(&status);

    content.append(&audio);

    // ---------------------------------------------------------
    // Runtime
    // ---------------------------------------------------------

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

    // ---------------------------------------------------------
    // Window
    // ---------------------------------------------------------

    root.append(&header);
    root.append(&content);

    window.set_content(Some(&root));
    window.present();
}
