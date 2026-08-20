use adw::prelude::*;
use adw::{ActionRow, Application, ApplicationWindow, HeaderBar, PreferencesGroup, StatusPage};

use gtk::{Align, Box, Button, DropDown, Orientation, Separator};

use crate::audio::{AudioDevice, AudioStatus};

pub fn build_window(app: &Application, audio_status: AudioStatus) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Tanix")
        .default_width(1100)
        .default_height(720)
        .build();

    let header = HeaderBar::builder().show_end_title_buttons(true).build();

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

    content.append(&Separator::new(Orientation::Horizontal));

    // ---------------------------------------------------------
    // Audio
    // ---------------------------------------------------------

    let audio = PreferencesGroup::builder()
        .title("Audio Interface")
        .description("Select the hardware Tanix should use for guitar audio")
        .build();

    match audio_status {
        AudioStatus::Connected { inputs, outputs } => {
            let hardware_inputs: Vec<AudioDevice> = inputs
                .iter()
                .filter(|device| device.is_hardware)
                .cloned()
                .collect();

            let hardware_outputs: Vec<AudioDevice> = outputs
                .iter()
                .filter(|device| device.is_hardware)
                .cloned()
                .collect();

            let input_row =
                create_device_row("Input", "Guitar / instrument input", &hardware_inputs);

            let output_row =
                create_device_row("Output", "Monitor / headphone output", &hardware_outputs);

            audio.add(&input_row);
            audio.add(&output_row);

            let status = ActionRow::builder()
                .title("PipeWire")
                .subtitle("Connected")
                .build();

            status.add_prefix(&create_status_indicator(true));
            audio.add(&status);

            if hardware_inputs.is_empty() && hardware_outputs.is_empty() {
                let warning = ActionRow::builder()
                    .title("No hardware interfaces detected")
                    .subtitle("Tanix can see PipeWire, but no physical audio interface was found.")
                    .build();

                audio.add(&warning);
            }
        }

        AudioStatus::Failed(error) => {
            let error_row = ActionRow::builder()
                .title("PipeWire unavailable")
                .subtitle(&error)
                .build();

            error_row.add_prefix(&create_status_indicator(false));
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

fn create_device_row(title: &str, subtitle: &str, devices: &[AudioDevice]) -> ActionRow {
    let row = ActionRow::builder().title(title).subtitle(subtitle).build();

    if devices.is_empty() {
        let dropdown = DropDown::from_strings(&["No hardware detected"]);
        dropdown.set_sensitive(false);

        row.add_suffix(&dropdown);

        return row;
    }

    let names: Vec<String> = devices.iter().map(device_display_name).collect();

    let name_refs: Vec<&str> = names.iter().map(String::as_str).collect();

    let dropdown = DropDown::from_strings(&name_refs);

    let devices_for_callback = devices.to_vec();

    dropdown.connect_selected_notify(move |dropdown| {
        let index = dropdown.selected() as usize;

        if let Some(device) = devices_for_callback.get(index) {
            println!(
                "Tanix selected {}: {} (PipeWire node {})",
                device.direction_label(),
                device.name,
                device.id
            );
        }
    });

    row.add_suffix(&dropdown);

    row
}

fn device_display_name(device: &AudioDevice) -> String {
    let manufacturer = device
        .vendor_name
        .as_deref()
        .or(device.device_name.as_deref());

    let product = device.product_name.as_deref();

    match (manufacturer, product) {
        (Some(manufacturer), Some(product)) if !product.contains(manufacturer) => {
            format!("{manufacturer} • {product}")
        }

        (Some(manufacturer), _) => {
            format!("{manufacturer} • {}", device.name)
        }

        (_, Some(product)) => product.to_string(),

        _ => device.name.clone(),
    }
}

fn create_status_indicator(connected: bool) -> gtk::Image {
    let icon = if connected {
        "emblem-ok-symbolic"
    } else {
        "dialog-error-symbolic"
    };

    gtk::Image::from_icon_name(icon)
}

impl AudioDevice {
    fn direction_label(&self) -> &'static str {
        match self.direction {
            crate::audio::AudioDirection::Input => "input",
            crate::audio::AudioDirection::Output => "output",
        }
    }
}
