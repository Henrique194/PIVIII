use std::io::Write;

use dialoguer::Select;
use sdl2::controller::{Axis, Button};
use sdl2::event::Event;
use sdl2::rwops::RWops;
use serialport::{ClearBuffer, SerialPort};

const BUTTONS: [Button; 8] = [
    Button::LeftShoulder,
    Button::RightShoulder,
    Button::Back,
    Button::Start,
    Button::A,
    Button::B,
    Button::X,
    Button::Y,
];

const TRIGGERS: [Axis; 2] = [
    Axis::TriggerLeft,
    Axis::TriggerRight
];

fn send_msg_arduino(serial_port: &mut Box<dyn SerialPort>, msg: &str) -> Result<(), String> {
    let mut buf = msg.as_bytes().to_vec();
    let mut end_msg = "/".as_bytes().to_vec();
    buf.append(&mut end_msg);

    serial_port.write_all(buf.as_slice()).unwrap();
    serial_port.flush().unwrap();
    serial_port.clear(ClearBuffer::All).unwrap();

    Ok(())
}

fn main() -> Result<(), String> {
    let ports: Vec<String> = serialport::available_ports()
        .map_err(|e| e.to_string())?
        .iter()
        .map(|p| p.port_name.clone())
        .collect();

    if ports.is_empty() {
        return Err("Nenhuma porta disponivel".to_owned());
    }

    let selection = Select::new()
        .with_prompt("Escolha uma porta:")
        .items(&ports)
        .interact()
        .unwrap();

    let mut serial_port = serialport::new(&ports[selection], 115200)
        .open()
        .map_err(|e| e.to_string())?;

    let sdl = sdl2::init()?;

    let rw = RWops::from_file("controller_mapping.txt", "rb")?;
    let joystick = sdl.game_controller()?;
    joystick.load_mappings_from_rw(rw).map_err(|e| e.to_string())?;
    let _game_controller = joystick.open(0).map_err(|e| e.to_string())?;

    let mut event_pump = sdl.event_pump()?;

    loop {
        let event = match event_pump.poll_event() {
            Some(event) => event,
            None => continue
        };

        match event {
            Event::ControllerButtonDown {button, ..}
            | Event::ControllerButtonUp {button, ..} => {
                if BUTTONS.contains(&button) {
                    send_msg_arduino(&mut serial_port, &button.string())?;
                }
            },
            Event::ControllerAxisMotion { axis, .. } => {
                if TRIGGERS.contains(&axis) {
                    send_msg_arduino(&mut serial_port, &axis.string())?;
                }
            }
            _ => ()
        }
    }
}
