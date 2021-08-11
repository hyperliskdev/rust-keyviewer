use std::thread::JoinHandle;
use std::sync::mpsc::Receiver;
use device_query::Keycode;
use raylib::prelude::*;


/// Begin and run the graphics thread
pub fn begin_gfx(from_keylog: Receiver<Vec<Keycode>>) -> JoinHandle<()> {


    std::thread::spawn(move || {

        // Init Raylib
        let (mut rl, thread) = raylib::init()
            .size(640, 480)
            .title("Hello, World")
            .build();


        
        while !rl.window_should_close() {
            let mut d = rl.begin_drawing(&thread);
            
            
            d.clear_background(Color::RED);
        
            
        }



        
    })
}

