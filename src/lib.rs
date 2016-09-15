#![feature(plugin)]
#![feature(try_from)]
#![plugin(clippy)]

// #![cfg_attr(test, plugin(quickcheck_macros))]

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

extern crate core;

mod semitone;

use semitone::Semitone;
use core::convert::TryInto;


type Degree = u8; // 1..7 typically
type Step = u8; // 1..2 typically

#[derive(Copy, Clone)]
pub struct Octaved<T: Copy> {
    value: T,
    octave: u8,
}

#[derive(Debug)]
pub struct Scale<'a> {
    root: Semitone,
    map: &'a [Step],
    notes: Vec<Semitone>,
}

pub const MAJOR_SCALE: [Step; 7] = [2, 2, 1, 2, 2, 2, 1];
pub const MINOR_SCALE: [Step; 7] = [2, 1, 2, 2, 1, 2, 2];

impl<'a> Scale<'a> {
    pub fn new(root: Semitone, map: &[Step]) -> Scale {
        let notes = (0..map.len() as usize)
            .map(|i| Self::internal_create_note(map, root, i))
            .collect();
        Scale {
            root: root,
            map: map,
            notes: notes,
        }
    }

    pub fn degrees(&self) -> Degree {
        self.map.len() as u8
    }

    pub fn notes(&self) -> &[Semitone] {
        &self.notes
    }

    fn internal_create_note(map: &[Step], root: Semitone, degree: usize) -> Semitone {
        if degree == 0 {
            root
        } else {
            let preresult = map.iter().take(degree).sum::<u8>() + root.value();
            let result = if preresult > 12 {
                // wrap
                preresult - 12
            } else {
                preresult
            };
            result.try_into().expect("Tried to create out-of-range Semitone! Bug.")
        }
    }

    pub fn create_note(&self, degree: Degree) -> Semitone {
        self.notes[degree as usize - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::convert::TryInto;

    #[test]
    fn create_note_major() {
        let map = MAJOR_SCALE;
        let s = Scale::new("C".try_into().unwrap(), &map);
        assert_eq!(s.create_note(1), "C");
        assert_eq!(s.create_note(2), "D");
        assert_eq!(s.create_note(3), "E");
        assert_eq!(s.create_note(4), "F");
        assert_eq!(s.create_note(5), "G");
        assert_eq!(s.create_note(6), "A");
        assert_eq!(s.create_note(7), "B");
    }

    #[test]
    fn create_note_minor() {
        let map = MINOR_SCALE;
        let s = Scale::new(1.try_into().unwrap(), &map);
        assert_eq!(s.create_note(1), "C");
        assert_eq!(s.create_note(2), "D");
        assert_eq!(s.create_note(3), "D#");
        assert_eq!(s.create_note(4), "F");
        assert_eq!(s.create_note(5), "G");
        assert_eq!(s.create_note(6), "G#");
        assert_eq!(s.create_note(7), "A#");
    }

    #[test]
    fn create_note_a_minor() {
        let map = MINOR_SCALE;
        let s = Scale::new("A".try_into().unwrap(), &map);
        assert_eq!(s.create_note(1), "A");
        assert_eq!(s.create_note(2), "B");
        assert_eq!(s.create_note(3), "C");
        assert_eq!(s.create_note(4), "D");
        assert_eq!(s.create_note(5), "E");
        assert_eq!(s.create_note(6), "F");
        assert_eq!(s.create_note(7), "G");
    }
}
