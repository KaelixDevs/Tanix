use pipewire as pw;

use std::{
    cell::Cell,
    rc::Rc,
    sync::mpsc,
    thread,
};

#[derive(Debug, Clone)]
pub struct AudioDevice {
    pub id: u32,
    pub name: String,
    pub media_class: String,
}

#[derive(Debug, Clone)]
pub enum AudioStatus {
    Connected {
        inputs: Vec<AudioDevice>,
        outputs: Vec<AudioDevice>,
    },
    Failed(String),
}

pub fn detect_audio_devices() -> AudioStatus {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        pw::init();

        let result = (|| -> Result<(), Box<dyn std::error::Error>> {
            let main_loop = pw::main_loop::MainLoopRc::new(None)?;
            let context = pw::context::ContextRc::new(&main_loop, None)?;
            let core = context.connect_rc(None)?;
            let registry = core.get_registry_rc()?;

            let inputs: Rc<std::cell::RefCell<Vec<AudioDevice>>> =
                Rc::new(std::cell::RefCell::new(Vec::new()));

            let outputs: Rc<std::cell::RefCell<Vec<AudioDevice>>> =
                Rc::new(std::cell::RefCell::new(Vec::new()));

            let inputs_clone = Rc::clone(&inputs);
            let outputs_clone = Rc::clone(&outputs);

            let _registry_listener = registry
                .add_listener_local()
                .global(move |global| {
                    let Some(props) = global.props.as_ref() else {
                        return;
                    };

                    let Some(media_class) = props.get("media.class") else {
                        return;
                    };

                    if !media_class.starts_with("Audio/") {
                        return;
                    }

                    let name = props
                        .get("node.description")
                        .or_else(|| props.get("node.nick"))
                        .or_else(|| props.get("node.name"))
                        .unwrap_or("Unnamed Audio Device")
                        .to_string();

                    let device = AudioDevice {
                        id: global.id,
                        name,
                        media_class: media_class.to_string(),
                    };

                    match media_class {
                        "Audio/Source" => {
                            inputs_clone.borrow_mut().push(device);
                        }

                        "Audio/Sink" => {
                            outputs_clone.borrow_mut().push(device);
                        }

                        "Audio/Duplex" => {
                            inputs_clone.borrow_mut().push(device.clone());
                            outputs_clone.borrow_mut().push(device);
                        }

                        _ => {}
                    }
                })
                .register();

            let done = Rc::new(Cell::new(false));
            let done_clone = Rc::clone(&done);
            let loop_clone = main_loop.clone();

            let pending = core.sync(0)?;

            let _core_listener = core
                .add_listener_local()
                .done(move |id, seq| {
                    if id == pw::core::PW_ID_CORE && seq == pending {
                        done_clone.set(true);
                        loop_clone.quit();
                    }
                })
                .register();

            while !done.get() {
                main_loop.run();
            }

            let inputs = inputs.borrow().clone();
            let outputs = outputs.borrow().clone();

            tx.send(AudioStatus::Connected { inputs, outputs })?;

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
