#![feature(plugin)]
#![feature(try_from)]
#![plugin(clippy)]

// #![cfg_attr(test, plugin(quickcheck_macros))]

#[cfg(test)]
#[macro_use]
extern crate quickcheck;
extern crate core;
#[macro_use(s)]
extern crate ndarray;
extern crate portmidi;
extern crate rustc_serialize;
extern crate docopt;
extern crate time;
extern crate monome;
#[macro_use]
extern crate log;
extern crate env_logger;

mod semitone;
mod scale;
mod sequencer;
mod midi;

use docopt::Docopt;
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::io;
use std::io::{Write};
use core::convert::TryInto;
use portmidi::{PortMidi, OutputPort};

use monome::{Monome, MonomeEvent, MonomeAction};

const USAGE: &'static str = "
Modal Seq.

Usage:
  modal_seq <device-id>
  modal_seq (-h | --help)

Options:
  -h --help   Show this screen

Omitting <midi output port> will list all ports.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_device_id: i32,
}

fn print_devices(pm: &PortMidi) {
    for dev in pm.devices().unwrap() {
        if dev.is_output() {
            println!("{}", dev);
        }
    }
}

fn main() {
    env_logger::init().unwrap();
    let monome = Monome::new().unwrap();
    let midi_context = PortMidi::new().unwrap();

    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|err| {
        print_devices(&midi_context);
        err.exit();
    });

    let midi_output_port = midi_context.device(args.arg_device_id)
        .and_then(|dev| midi_context.output_port(dev, 1024))
        .unwrap();

    play(midi_output_port, monome);
}

fn handle_transpose(seq: &mut sequencer::Sequencer, x: u8, y: u8) -> bool {
    let xval = 7 - (15-x);
    let sletter = if y == 6 {
        match xval {
            0 => "C",
            1 => "D",
            2 => "E",
            3 => "F",
            4 => "G",
            5 => "A",
            6 => "B",
            _ => ""
        }
    } else {
        match xval {
            1 => "C#",
            2 => "D#",
            4 => "F#",
            5 => "G#",
            6 => "A#",
            _ => ""
        }
    };
    if sletter != "" {
        seq.transpose(sletter.try_into().unwrap());
        true
    } else {
        false
    }
}

fn play(mut port: OutputPort, mut monome: Monome) {
    let mut seq = sequencer::Sequencer::new();
    let mut note_sched = midi::NoteScheduler::new(&mut port);

    monome.send(&MonomeAction::LedAll(false)).unwrap();
    // TODO: render initial state properly
    monome.send(&MonomeAction::LedMap(0, 0, &[0, 0, 0, 0, 0, 0, 0, 255])).unwrap();

    let mut next_note_expiration = Instant::now();

    loop {
        if let Some(event) = monome.poll().unwrap() {
            match event {
                MonomeEvent::Key(x, y, s) => {
                    if s {
                        if x <= 7 {
                            let step_val = 7 - y;
                            info!("step: {}, val: {}", x, step_val);
                            seq.set_step(x as usize, step_val);
                            monome.send(&MonomeAction::LedCol(x, 0, 1 << y)).unwrap();
                        } else if y == 7 {
                            let mval = 15-x;
                            seq.modulate(mval);
                            monome.send(&MonomeAction::LedRow(8, 7, 1 << (7 - mval))).unwrap();
                        } else if y == 5 || y == 6 {
                            if handle_transpose(&mut seq, x, y) {
                                monome.send(&MonomeAction::LedRow(8, 6, 0)).unwrap();
                                monome.send(&MonomeAction::LedRow(8, 5, 0)).unwrap();
                                monome.send(&MonomeAction::LedSet(x, y, true)).unwrap();
                            }
                        }
                    }
                }
            }
        }

        note_sched.flush_notes().unwrap();

        let now = Instant::now();
        if now >= next_note_expiration {
            next_note_expiration = now + Duration::from_millis(130);
            match seq.note() {
                Some(note) => {
                    note_sched.play(note).unwrap();
                }
                None => { () }
            }
            io::stdout().flush().unwrap();
            seq.next();
        }

        sleep(Duration::from_millis(1));
    }
}
