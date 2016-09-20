
pub type Degree = u8; // 1..7 typically
type Step = u8; // 1..2 typically

#[derive(Copy, Clone)]
pub struct Octaved<T: Copy> {
    pub value: T,
    pub octave: u8,
}

#[derive(Debug)]
pub struct Scale {
    root: Semitone,
    mode: u8,
}

const MODE_PATTERN: [Step; 7] = [2, 2, 1, 2, 2, 2, 1];

impl Scale {
    pub fn new() -> Scale {
        let root = Semitone::new(1).unwrap();
        Scale {
            root: root,
            mode: 0,
        }
    }

    pub fn modulate(&mut self, new_mode: u8) {
        if new_mode <= 7 {
            self.mode = new_mode;
        } else {
            panic!("Scale mode out of range.");
        }
    }

    pub fn transpose(&mut self, new_root: Semitone) {
        self.root = new_root;
    }

    pub fn create_note(&self, degree: Degree) -> Semitone {
        if degree == 0 {
            panic!("scale degree cannot be 0");
        }

        if degree == 1 {
            self.root
        } else {
            let scale = &MODE_PATTERN;
            let map_iter = scale.iter().cycle().skip(self.mode as usize);
            let preresult = map_iter.take(degree as usize - 1).sum::<u8>() + self.root.value();
            let result = if preresult > 12 {
                // wrap
                preresult - 12
            } else {
                preresult
            };
            result.try_into().expect("Tried to create out-of-range Semitone! Bug.")
        }
    }

    pub fn root(&self) -> Semitone {
        self.root
    }
}

use semitone::Semitone;
use core::convert::TryInto;

#[cfg(test)]
mod tests {
    use super::*;
    use core::convert::TryInto;

    #[test]
    fn create_note_major() {
        let s = Scale::new();
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
        let mut s = Scale::new();
        s.modulate(5);
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
        let mut s = Scale::new();
        s.transpose("A".try_into().unwrap());
        s.modulate(5);
        assert_eq!(s.create_note(1), "A");
        assert_eq!(s.create_note(2), "B");
        assert_eq!(s.create_note(3), "C");
        assert_eq!(s.create_note(4), "D");
        assert_eq!(s.create_note(5), "E");
        assert_eq!(s.create_note(6), "F");
        assert_eq!(s.create_note(7), "G");
    }
}
