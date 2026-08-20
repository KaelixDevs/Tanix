use adw::prelude::*;
use adw::{ActionRow, Application, ApplicationWindow, HeaderBar, PreferencesGroup, StatusPage};

use gtk::{Align, Box, Button, DropDown, Label, Orientation, Separator};

use crate::audio::{AudioDevice, AudioStatus};

pub fn build_window(app: &Application, audio_status: AudioStatus) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Tanix")
        .default_width(1100)
        .default_height(720)
        .build();

    // ---------------------------------------------------------
    // Header
    // ---------------------------------------------------------

    let header = HeaderBar::builder().show_end_title_buttons(true).build();

    // ---------------------------------------------------------
    // Root layout
    // ---------------------------------------------------------

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
        .description("Select the input and output devices Tanix should use")
        .build();

    match audio_status {
        AudioStatus::Connected { inputs, outputs } => {
            let input_dropdown = create_device_dropdown(
                "Input Device",
                "Select the device your guitar is connected to",
                &inputs,
            );

            let output_dropdown = create_device_dropdown(
                "Output Device",
                "Select where Tanix should send audio",
                &outputs,
            );

            audio.add(&input_dropdown);
            audio.add(&output_dropdown);

            let status = Label::new(Some("● PipeWire connected"));
            status.set_xalign(0.0);
            status.add_css_class("dim-label");

            audio.add(&status);
        }

        AudioStatus::Failed(error) => {
            let error_row = ActionRow::builder()
                .title("PipeWire")
                .subtitle(&format!("Unable to detect audio devices: {error}"))
                .build();

            audio.add(&error_row);
        }
    }

    content.append(&audio);

    // ---------------------------------------------------------
    // Runtime
    // ---------------------------------------------------------

    let runtime = PreferencesGroup::builder()
        .title("Tanix Runtime")
        .description("Compatibility environment")
        .build();

    let wine = ActionRow::builder()
        .title("Wine")
        .subtitle("Not configured")
        .build();

    let backend = ActionRow::builder()
        .title("Audio Backend")
        .subtitle("PipeWire")
        .build();

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

fn create_device_dropdown(title: &str, subtitle: &str, devices: &[AudioDevice]) -> ActionRow {
    let row = ActionRow::builder().title(title).subtitle(subtitle).build();

    if devices.is_empty() {
        let dropdown = DropDown::from_strings(&["No devices detected"]);
        dropdown.set_sensitive(false);
        dropdown.set_hexpand(false);

        row.add_suffix(&dropdown);

        return row;
    }

    let device_names: Vec<&str> = devices.iter().map(|device| device.name.as_str()).collect();

    let dropdown = DropDown::from_strings(&device_names);
    dropdown.set_hexpand(false);

    let devices_for_callback = devices.to_vec();

    dropdown.connect_selected_notify(move |dropdown| {
        let selected = dropdown.selected() as usize;

        if let Some(device) = devices_for_callback.get(selected) {
            println!(
                "Tanix selected audio device: {} (id={}, class={})",
                device.name, device.id, device.media_class
            );
        }
    });

    row.add_suffix(&dropdown);

    row
}
