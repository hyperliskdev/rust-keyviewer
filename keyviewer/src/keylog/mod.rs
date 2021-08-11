use std::thread::JoinHandle;
use std::sync::mpsc::Sender;

use device_query::{DeviceQuery, DeviceState, Keycode};

/// Begin and run the graphics thread
pub fn begin_keylog(to_keylog: Sender<Vec<Keycode>>) -> JoinHandle<()> {
    std::thread::spawn(move || {

        // to_keylog.send(KeyLogEvent::KeyPressed(...))
        let device_state = DeviceState::new();

        let mut keys: Vec<Keycode>;
        
        loop {

            keys = device_state.get_keys();
        
        to_keylog.send(keys);
    }

    })
}