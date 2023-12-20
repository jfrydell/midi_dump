use std::io::stdin;

use midir::{Ignore, MidiInput};

fn main() {
    // Create input instance
    let mut midi_in = MidiInput::new("midir reading input").unwrap();
    midi_in.ignore(Ignore::None);

    // Get all input ports and list numbers for each
    let in_ports = midi_in.ports();
    eprintln!("Found {} input ports", in_ports.len());
    if in_ports.is_empty() {
        eprintln!("Cannot continue without input port");
        return;
    }
    for p in in_ports.iter() {
        eprintln!(" - {}", midi_in.port_name(p).unwrap());
    }

    // Add listeners to each input port
    let mut listeners = vec![];
    for (i, port) in in_ports.into_iter().enumerate() {
        // Create new device for this port and verify that port name is still the same
        let new_midi_in = MidiInput::new("midir reading input").unwrap();
        let new_port = &new_midi_in.ports()[i];
        assert!(new_midi_in.port_name(new_port).unwrap() == midi_in.port_name(&port).unwrap());
        eprintln!(
            "Connection open, reading input from '{}'",
            new_midi_in.port_name(new_port).unwrap()
        );
        // Connect to input
        let conn = new_midi_in
            .connect(
                new_port,
                "midi-dump-read-input",
                move |_, message, _| {
                    print_message(message);
                },
                (),
            )
            .unwrap();
        // Add to array so we don't disconnect
        listeners.push(conn);
    }

    // Wait for "exit" on stdin
    let mut input = String::new();
    loop {
        stdin().read_line(&mut input).expect("stdin input failed");
        if input.starts_with("exit") {
            eprintln!("Exiting...");
            break;
        }
    }
}

/// Print a parsed MIDI message to stdout
fn print_message(message: &[u8]) {
    let mut message = message.to_vec();
    if message.is_empty() {
        return;
    }

    // Parse out channel and send that first
    let channel = message[0] & 0x0f;
    if message[0] < 0xf0 {
        message[0] &= 0xf0;
    }
    print!("{} ", channel);
    // Parse actual message
    match message.as_slice() {
        [0x90, note, velocity] => {
            println!("note_on {note} {velocity}");
        }
        [0x80, note, velocity] => {
            println!("note_off {note} {velocity}");
        }
        [0xb0, control, value] => {
            println!("control_change {control} {value}");
        }
        [0xe0, lsb, msb] => {
            println!("pitch_wheel_change {lsb} {msb}");
        }
        x => {
            println!(
                "unknown {}",
                x.iter().map(|x| format!("{:02x}", x)).collect::<String>()
            );
        }
    }
}
