use std::io::Write;
use std::{thread, time};

use sdl2::controller::Button;
use sdl2::event::Event;
use sdl2::rwops::RWops;
use serialport::ClearBuffer;

const BUTTONS: [Button; 6] = [
    Button::LeftShoulder,
    Button::RightShoulder,
    Button::Back,
    Button::Start,
    Button::A,
    Button::B,
];

pub fn run() -> Result<(), String> {
    let sdl = sdl2::init()?;
    let joystick = sdl.game_controller()?;
    let rw = RWops::from_file("controller_mapping.txt", "rb")?;
    joystick.load_mappings_from_rw(rw).unwrap();
    let _game_controller = joystick.open(0).unwrap();
    let mut event_pump = sdl.event_pump()?;

    loop {
        let event = match event_pump.poll_event() {
            Some(event) => event,
            None => continue
        };

        if let Event::ControllerButtonDown { button: button_down, .. } = event {
            if BUTTONS.contains(&button_down) {
                write_serial(&button_down.string());
            }

            loop {
                let event = match event_pump.poll_event() {
                    Some(event) => event,
                    None => continue
                };

                if let Event::ControllerButtonUp { button: button_up, .. } = event {
                    if button_up == button_down {
                        break;
                    }
                }

                println!("Segurando botão");
            }

            println!("Saiu do loop");
            if BUTTONS.contains(&button_down) {
                write_serial(&button_down.string());
            }
        }
    }

    Ok(())
}

fn write_serial(message: &str) {
    let mut serial_port = serialport::new("COM4", 115200).open().unwrap();

    let mut buf = message.as_bytes().to_vec();
    let mut end_msg = "/".as_bytes().to_vec();
    buf.append(&mut end_msg);

    serial_port.write_all(buf.as_slice()).unwrap();
    serial_port.flush().unwrap();
    serial_port.clear(ClearBuffer::All).unwrap();
}

fn main() -> Result<(), String> {
    run()
}
