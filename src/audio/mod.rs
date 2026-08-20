use pipewire as pw;

use std::{cell::Cell, rc::Rc, sync::mpsc, thread};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AudioDirection {
    Input,
    Output,
}

#[derive(Debug, Clone)]
pub struct AudioDevice {
    pub id: u32,

    // PipeWire / node information
    pub name: String,
    pub node_name: Option<String>,
    pub media_class: String,

    // Hardware information when PipeWire exposes it
    pub device_name: Option<String>,
    pub vendor_name: Option<String>,
    pub product_name: Option<String>,
    pub vendor_id: Option<String>,
    pub product_id: Option<String>,
    pub bus: Option<String>,
    pub api: Option<String>,
    pub serial: Option<String>,

    // Whether this appears to represent actual hardware
    pub is_hardware: bool,

    // Direction this node can be used for
    pub direction: AudioDirection,
}

#[derive(Debug, Clone)]
pub enum AudioStatus {
    Connected {
        inputs: Vec<AudioDevice>,
        outputs: Vec<AudioDevice>,
    },
    Failed(String),
}

fn prop(props: &pw::spa::utils::dict::DictRef, key: &str) -> Option<String> {
    props.get(key).map(str::to_string)
}

fn is_hardware_device(
    api: &Option<String>,
    bus: &Option<String>,
    vendor_id: &Option<String>,
    product_id: &Option<String>,
    device_name: &Option<String>,
) -> bool {
    api.is_some()
        || bus.is_some()
        || vendor_id.is_some()
        || product_id.is_some()
        || device_name.is_some()
}

fn build_audio_device(
    global: &pw::registry::GlobalObject<&pw::spa::utils::dict::DictRef>,
    media_class: &str,
    props: &pw::spa::utils::dict::DictRef,
    direction: AudioDirection,
) -> AudioDevice {
    let node_name = prop(props, "node.name");

    let device_name = prop(props, "device.name");
    let vendor_name = prop(props, "device.vendor.name");
    let product_name = prop(props, "device.product.name");
    let vendor_id = prop(props, "device.vendor.id");
    let product_id = prop(props, "device.product.id");
    let bus = prop(props, "device.bus");
    let api = prop(props, "device.api");
    let serial = prop(props, "device.serial");

    let name = prop(props, "node.description")
        .or_else(|| prop(props, "device.description"))
        .or_else(|| prop(props, "device.nick"))
        .or_else(|| product_name.clone())
        .or_else(|| node_name.clone())
        .unwrap_or_else(|| "Unnamed Audio Device".to_string());

    let is_hardware = is_hardware_device(&api, &bus, &vendor_id, &product_id, &device_name);

    AudioDevice {
        id: global.id,
        name,
        node_name,
        media_class: media_class.to_string(),
        device_name,
        vendor_name,
        product_name,
        vendor_id,
        product_id,
        bus,
        api,
        serial,
        is_hardware,
        direction,
    }
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

            let inputs = Rc::new(std::cell::RefCell::new(Vec::<AudioDevice>::new()));
            let outputs = Rc::new(std::cell::RefCell::new(Vec::<AudioDevice>::new()));

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

                    let media_class = media_class.to_string();

                    let is_audio_source = media_class == "Audio/Source";
                    let is_audio_sink = media_class == "Audio/Sink";
                    let is_audio_duplex = media_class == "Audio/Duplex";

                    if !is_audio_source && !is_audio_sink && !is_audio_duplex {
                        return;
                    }

                    if is_audio_source || is_audio_duplex {
                        let device =
                            build_audio_device(global, &media_class, props, AudioDirection::Input);

                        inputs_clone.borrow_mut().push(device);
                    }

                    if is_audio_sink || is_audio_duplex {
                        let device =
                            build_audio_device(global, &media_class, props, AudioDirection::Output);

                        outputs_clone.borrow_mut().push(device);
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

            let mut inputs = inputs.borrow().clone();
            let mut outputs = outputs.borrow().clone();

            // Prefer hardware devices over virtual/software nodes.
            inputs.sort_by_key(|device| !device.is_hardware);
            outputs.sort_by_key(|device| !device.is_hardware);

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
