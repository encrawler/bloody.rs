extern crate rusb;
extern crate clap;

use mouse::Mouse;
use clap::Parser;
use cli::*;

mod mouse;
mod reqtype;
mod discovery;
mod opcode;
mod gtk;
mod cli;

fn create_mouse() -> Mouse {

    let devices = discovery::bloody_devices();
    let device = devices.first().unwrap();
    
    let description = device.device_descriptor().unwrap();
    println!("Found compatible device: {}", description.product_id());

    let handle = device.open().unwrap();
    let mut mouse = Mouse::new(handle);
    mouse.detach_driver_if_needed();
    mouse.claim();

    mouse
}

fn get_level() {
    let mouse = create_mouse();
    println!("Current mouse intensity: {}", mouse.get_backlight());
}

fn set_level(level: u8) {
    let mouse = create_mouse();
    mouse.set_backlight(level);

    if mouse.get_backlight() == level {
        println!("Intensity succesfully set to: {}", level);
    }
    else {
        println!("Failed to change intensity to {}", level);
    }
}

fn main() {
    let cli = Cli::parse();

    if cli.tray {
        return gtk::main(create_mouse());
    }

    match cli.command {
        Some(Action::Level) => get_level(),
        Some(Action::SetLevel { level }) => set_level(level),
        None => println!("type help to see command list")
    }           
}
