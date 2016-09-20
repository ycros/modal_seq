use portmidi::{MidiMessage, OutputPort, Error};
use core::result;
use core::ops::Add;
use super::semitone::{Semitone};
use super::scale::{Octaved};
use time::precise_time_ns;
use time::Duration;


const CHANNEL: u8 = 0;

struct InFlightNote {
    note: u8,
    expiration: u64,
}

pub struct NoteScheduler<'a> {
    port: &'a mut OutputPort,
    in_flight: Vec<InFlightNote>,
}

#[derive(Debug)]
pub enum NoteSchedulerError {
    MidiNoteOutput(Error)
}

impl From<Error> for NoteSchedulerError {
    fn from(pm_error: Error) -> Self {
        NoteSchedulerError::MidiNoteOutput(pm_error)
    }
}

type Result<T> = result::Result<T, NoteSchedulerError>;

impl<'a> NoteScheduler<'a> {
    pub fn new(port: &'a mut OutputPort) -> Self {
        NoteScheduler {
            port: port,
            in_flight: Vec::new(),
        }
    }

    pub fn play(&mut self, oct_semitone: Octaved<Semitone>) -> Result<()> {
        let midi_note = (oct_semitone.octave * 12) + (oct_semitone.value.value() - 1);
        self.play_raw_note(midi_note)
    }

    pub fn play_raw_note(&mut self, note: u8) -> Result<()> {
        let note_on = MidiMessage {
            status: 0x90 + CHANNEL,
            data1: note,
            data2: 100,
        };

        try!(self.port.write_message(note_on));

        let expiration = Duration::nanoseconds(precise_time_ns() as i64) // ugh
            .add(Duration::milliseconds(100))
            .num_nanoseconds()
            .unwrap() as u64;

        self.in_flight.push(InFlightNote { note: note, expiration: expiration });
        Ok(())
    }

    pub fn flush_notes(&mut self) -> Result<()> {
        let now = precise_time_ns();
        for in_flight in &self.in_flight {
            if in_flight.expiration <= now {
                let note_off = MidiMessage {
                    status: 0x80 + CHANNEL,
                    data1: in_flight.note,
                    data2: 100,
                };
                try!(self.port.write_message(note_off));
            }
        }
        self.in_flight.retain(|i| i.expiration > now);
        Ok(())
    }
}