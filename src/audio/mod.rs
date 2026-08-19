use pipewire::{
    context::ContextBox,
    main_loop::MainLoopBox,
};

use std::{
    sync::mpsc,
    thread,
};

#[derive(Debug, Clone)]
pub enum AudioStatus {
    Connected,
    Failed(String),
}

pub fn check_pipewire() -> AudioStatus {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let result = (|| -> Result<(), Box<dyn std::error::Error>> {
            let main_loop = MainLoopBox::new(None)?;
            let context = ContextBox::new(&main_loop.loop_(), None)?;
            let _core = context.connect(None)?;

            tx.send(AudioStatus::Connected)?;

            Ok(())
        })();

        if let Err(error) = result {
            let _ = tx.send(AudioStatus::Failed(error.to_string()));
        }
    });

    match rx.recv() {
        Ok(status) => status,
        Err(error) => AudioStatus::Failed(error.to_string()),
    }
}
