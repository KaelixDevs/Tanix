use adw::prelude::*;
use adw::{ActionRow, Application, ApplicationWindow, HeaderBar, PreferencesGroup, StatusPage};

use gtk::{Align, Box, Button, DropDown, Orientation, Separator};

use std::{cell::RefCell, rc::Rc};

use crate::{
    audio::{AudioDevice, AudioDirection, AudioStatus, AudioVendor, detect_audio_devices},
    config::TanixConfig,
};

pub fn build_window(app: &Application, audio_status: AudioStatus) {
    let config = Rc::new(RefCell::new(TanixConfig::load()));

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
        .description("Configure the hardware used by Tanix")
        .build();

    let refresh_button = Button::with_label("Refresh");

    let refresh_row = ActionRow::builder()
        .title("Audio Devices")
        .subtitle("Refresh available PipeWire hardware")
        .build();

    refresh_row.add_suffix(&refresh_button);
    audio.add(&refresh_row);

    match audio_status {
        AudioStatus::Connected { inputs, outputs } => {
            let hardware_inputs: Vec<AudioDevice> = inputs
                .into_iter()
                .filter(|device| device.is_hardware)
                .collect();

            let hardware_outputs: Vec<AudioDevice> = outputs
                .into_iter()
                .filter(|device| device.is_hardware)
                .collect();

            let input_row = create_device_row(
                "Input",
                "Guitar / instrument input",
                &hardware_inputs,
                &config,
                AudioDirection::Input,
            );

            let output_row = create_device_row(
                "Output",
                "Monitor / headphone output",
                &hardware_outputs,
                &config,
                AudioDirection::Output,
            );

            audio.add(&input_row);
            audio.add(&output_row);

            // -------------------------------------------------
            // Selected interface information
            // -------------------------------------------------

            let selected_device = selected_device(&hardware_inputs, &hardware_outputs, &config);

            if let Some(device) = selected_device {
                let hardware = PreferencesGroup::builder()
                    .title("Hardware Information")
                    .description("Information reported by PipeWire / ALSA")
                    .build();

                hardware.add(&info_row(
                    "Manufacturer",
                    device.vendor_name.as_deref().unwrap_or("Unknown"),
                ));

                hardware.add(&info_row(
                    "Product",
                    device
                        .product_name
                        .as_deref()
                        .or(device.device_name.as_deref())
                        .unwrap_or(&device.name),
                ));

                hardware.add(&info_row("Vendor", device.vendor.display_name()));

                hardware.add(&info_row("Bus", device.bus.as_deref().unwrap_or("Unknown")));

                hardware.add(&info_row("API", device.api.as_deref().unwrap_or("Unknown")));

                hardware.add(&info_row(
                    "Node",
                    device.node_name.as_deref().unwrap_or("Unknown"),
                ));

                hardware.add(&info_row("PipeWire ID", &device.id.to_string()));

                audio.add(&hardware);
            }

            // -------------------------------------------------
            // Sample rate
            // -------------------------------------------------

            let sample_rates = [
                "44100 Hz",
                "48000 Hz",
                "88200 Hz",
                "96000 Hz",
                "176400 Hz",
                "192000 Hz",
            ];

            let sample_rate_row = ActionRow::builder()
                .title("Sample Rate")
                .subtitle("Audio engine sample rate")
                .build();

            let sample_rate_dropdown = DropDown::from_strings(&sample_rates);

            let current_rate = config.borrow().audio.sample_rate;

            let rate_index = match current_rate {
                44_100 => 0,
                48_000 => 1,
                88_200 => 2,
                96_000 => 3,
                176_400 => 4,
                192_000 => 5,
                _ => 1,
            };

            sample_rate_dropdown.set_selected(rate_index);

            let config_clone = Rc::clone(&config);

            sample_rate_dropdown.connect_selected_notify(move |dropdown| {
                let rates = [44_100, 48_000, 88_200, 96_000, 176_400, 192_000];

                let index = dropdown.selected() as usize;

                if let Some(&rate) = rates.get(index) {
                    let mut config = config_clone.borrow_mut();

                    config.audio.sample_rate = rate;

                    if let Err(error) = config.save() {
                        eprintln!("Failed to save sample rate: {error}");
                    }
                }
            });

            sample_rate_row.add_suffix(&sample_rate_dropdown);

            audio.add(&sample_rate_row);

            // -------------------------------------------------
            // Buffer
            // -------------------------------------------------

            let buffer_sizes = [
                "32 samples",
                "64 samples",
                "128 samples",
                "256 samples",
                "512 samples",
                "1024 samples",
            ];

            let buffer_row = ActionRow::builder()
                .title("Buffer Size")
                .subtitle("Lower values reduce latency")
                .build();

            let buffer_dropdown = DropDown::from_strings(&buffer_sizes);

            let current_buffer = config.borrow().audio.buffer_size;

            let buffer_index = match current_buffer {
                32 => 0,
                64 => 1,
                128 => 2,
                256 => 3,
                512 => 4,
                1024 => 5,
                _ => 2,
            };

            buffer_dropdown.set_selected(buffer_index);

            let config_clone = Rc::clone(&config);

            buffer_dropdown.connect_selected_notify(move |dropdown| {
                let buffers = [32, 64, 128, 256, 512, 1024];

                let index = dropdown.selected() as usize;

                if let Some(&buffer) = buffers.get(index) {
                    let mut config = config_clone.borrow_mut();

                    config.audio.buffer_size = buffer;

                    if let Err(error) = config.save() {
                        eprintln!("Failed to save buffer size: {error}");
                    }
                }
            });

            buffer_row.add_suffix(&buffer_dropdown);

            audio.add(&buffer_row);

            // -------------------------------------------------
            // Latency
            // -------------------------------------------------

            let latency = calculate_latency(
                config.borrow().audio.sample_rate,
                config.borrow().audio.buffer_size,
            );

            let latency_row = ActionRow::builder()
                .title("Estimated Latency")
                .subtitle(&format!("{latency:.2} ms one-way buffer latency"))
                .build();

            audio.add(&latency_row);

            let status = ActionRow::builder()
                .title("PipeWire")
                .subtitle("Connected")
                .build();

            status.add_prefix(&status_icon(true));

            audio.add(&status);

            if hardware_inputs.is_empty() && hardware_outputs.is_empty() {
                let warning = ActionRow::builder()
                    .title("No hardware interfaces detected")
                    .subtitle("PipeWire is working, but no physical audio interface was found.")
                    .build();

                audio.add(&warning);
            }
        }

        AudioStatus::Failed(error) => {
            let error_row = ActionRow::builder()
                .title("PipeWire unavailable")
                .subtitle(&error)
                .build();

            error_row.add_prefix(&status_icon(false));

            audio.add(&error_row);
        }
    }

    refresh_button.connect_clicked(move |_| match detect_audio_devices() {
        AudioStatus::Connected { inputs, outputs } => {
            println!(
                "Tanix detected {} inputs and {} outputs",
                inputs.len(),
                outputs.len()
            );
        }

        AudioStatus::Failed(error) => {
            eprintln!("Audio refresh failed: {error}");
        }
    });

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
        .subtitle("PipeWire / ALSA")
        .build();

    runtime.add(&wine);
    runtime.add(&backend);

    content.append(&runtime);

    root.append(&header);
    root.append(&content);

    window.set_content(Some(&root));
    window.present();
}

fn create_device_row(
    title: &str,
    subtitle: &str,
    devices: &[AudioDevice],
    config: &Rc<RefCell<TanixConfig>>,
    direction: AudioDirection,
) -> ActionRow {
    let row = ActionRow::builder().title(title).subtitle(subtitle).build();

    if devices.is_empty() {
        let dropdown = DropDown::from_strings(&["No hardware detected"]);

        dropdown.set_sensitive(false);
        row.add_suffix(&dropdown);

        return row;
    }

    let names: Vec<String> = devices.iter().map(device_display_name).collect();

    let refs: Vec<&str> = names.iter().map(String::as_str).collect();

    let dropdown = DropDown::from_strings(&refs);

    let saved_id = match direction {
        AudioDirection::Input => config.borrow().audio.input_node_id,

        AudioDirection::Output => config.borrow().audio.output_node_id,
    };

    if let Some(saved_id) = saved_id {
        if let Some(index) = devices.iter().position(|device| device.id == saved_id) {
            dropdown.set_selected(index as u32);
        }
    }

    let devices_for_callback = devices.to_vec();

    let config_clone = Rc::clone(config);

    let direction_clone = direction.clone();

    dropdown.connect_selected_notify(move |dropdown| {
        let index = dropdown.selected() as usize;

        let Some(device) = devices_for_callback.get(index) else {
            return;
        };

        let mut config = config_clone.borrow_mut();

        match direction_clone {
            AudioDirection::Input => {
                config.audio.input_node_id = Some(device.id);
            }

            AudioDirection::Output => {
                config.audio.output_node_id = Some(device.id);
            }
        }

        if let Err(error) = config.save() {
            eprintln!("Failed to save audio device: {error}");
        }

        println!(
            "Selected {}: {} ({})",
            device.direction_label(),
            device.name,
            device.vendor.display_name()
        );
    });

    row.add_suffix(&dropdown);

    row
}

fn selected_device(
    inputs: &[AudioDevice],
    outputs: &[AudioDevice],
    config: &Rc<RefCell<TanixConfig>>,
) -> Option<AudioDevice> {
    let config = config.borrow();

    let input = config
        .audio
        .input_node_id
        .and_then(|id| inputs.iter().find(|device| device.id == id));

    let output = config
        .audio
        .output_node_id
        .and_then(|id| outputs.iter().find(|device| device.id == id));

    input.cloned().or_else(|| output.cloned())
}

fn info_row(title: &str, value: &str) -> ActionRow {
    ActionRow::builder().title(title).subtitle(value).build()
}

fn device_display_name(device: &AudioDevice) -> String {
    let vendor = device.vendor.display_name();

    let product = device
        .product_name
        .as_deref()
        .or(device.device_name.as_deref());

    match device.vendor {
        AudioVendor::Generic => match product {
            Some(product) => {
                format!("{product} • {}", device.name)
            }

            None => device.name.clone(),
        },

        _ => match product {
            Some(product) if !product.to_lowercase().contains(&vendor.to_lowercase()) => {
                format!("{vendor} • {product}")
            }

            _ => {
                format!("{vendor} • {}", device.name)
            }
        },
    }
}

fn calculate_latency(sample_rate: u32, buffer_size: u32) -> f64 {
    (buffer_size as f64 / sample_rate as f64) * 1000.0
}

fn status_icon(connected: bool) -> gtk::Image {
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
            AudioDirection::Input => "input",
            AudioDirection::Output => "output",
        }
    }
}
