
use super::semitone::Semitone;
use super::scale::{Scale, Degree, Octaved};

#[derive(Debug)]
pub struct Sequencer {
    steps: Vec<Degree>,
    current_step: usize,
    scale: Scale,
}

impl Sequencer {
    pub fn new() -> Sequencer {
        let mut steps = Vec::new();
        steps.resize(8, 0);
        Sequencer {
            steps: steps,

            current_step: 0,
            scale: Scale::new()
        }
    }

    pub fn next(&mut self) {
        self.current_step += 1;
        if self.current_step >= self.steps.len() {
            self.current_step = 0;
        }
    }

    pub fn step(&self) -> Degree {
        self.steps[self.current_step]
    }

    pub fn set_step(&mut self, index: usize, step: Degree) {
        self.steps[index] = step;
    }

    pub fn step_at(&self, index: usize) -> Degree {
        self.steps[index]
    }

    pub fn note(&self) -> Option<Octaved<Semitone>> {
        let step = self.step();
        if step > 0 {
            let mut octave = 5;
            let note = self.scale.create_note(step);
            if note < self.scale.root() {
                octave += 1;
            }
            Some(Octaved{ value: note, octave: octave })
        } else {
            None
        }
    }

    pub fn transpose(&mut self, new_root: Semitone) {
        self.scale.transpose(new_root);
    }

    pub fn modulate(&mut self, new_mode: u8) {
        self.scale.modulate(new_mode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {

    }
}