use clap::{App, Arg, crate_authors, crate_description, crate_version, crate_name};
use std::sync::mpsc::{Sender, Receiver};
use crate::{gui::begin_gfx, keylog::begin_keylog};

mod gui;
mod keylog;



fn main() {
    let matches = App::new(crate_name!())
    .author(crate_authors!())
    .about(crate_description!())
    .version(crate_version!())
    .get_matches();

    // Get privs on device
    sudo::escalate_if_needed().expect("Could not escalate to root");

    // Create interthreaded channel
    let (chan_to_gfx, chan_from_keylog) = std::sync::mpsc::channel();

    // Begin the graphics thread
    let gfx_thread = begin_gfx(chan_from_keylog);
    let device_thread = begin_keylog(chan_to_gfx);

    // Very last call

    gfx_thread.join();
}
