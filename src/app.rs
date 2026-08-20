use adw::Application;
use adw::prelude::*;

use crate::audio;
use crate::ui::window::build_window;

const APP_ID: &str = "io.tanix.Tanix";

pub fn run() {
    let application = Application::builder().application_id(APP_ID).build();

    application.connect_activate(|app| {
        let audio_status = audio::detect_audio_devices();
        build_window(app, audio_status);
    });

    application.run();
}
