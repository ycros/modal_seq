use core::convert::TryFrom;


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Semitone {
    value: u8, // 1..12
}

impl Semitone {
    pub fn new(value: u8) -> Result<Self, &'static str> {
        if value >= 1 && value <= 12 {
            Ok(Semitone { value: value })
        } else {
            Err("Value out of range for semitone (1-12)")
        }
    }

    pub fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "C" => Ok(Semitone { value: 1 }),
            "C#" => Ok(Semitone { value: 2 }),
            "D" => Ok(Semitone { value: 3 }),
            "D#" => Ok(Semitone { value: 4 }),
            "E" => Ok(Semitone { value: 5 }),
            "F" => Ok(Semitone { value: 6 }),
            "F#" => Ok(Semitone { value: 7 }),
            "G" => Ok(Semitone { value: 8 }),
            "G#" => Ok(Semitone { value: 9 }),
            "A" => Ok(Semitone { value: 10 }),
            "A#" => Ok(Semitone { value: 11 }),
            "B" => Ok(Semitone { value: 12 }),
            _ => Err("Invalid note string format (A-G) or (A#-G#)"),
        }
    }

    pub fn to_letter(&self) -> &'static str {
        match self.value {
            1 => "C",
            2 => "C#",
            3 => "D",
            4 => "D#",
            5 => "E",
            6 => "F",
            7 => "F#",
            8 => "G",
            9 => "G#",
            10 => "A",
            11 => "A#",
            12 => "B",
            _ => panic!("Invalid value in Semitone! Should never happen."),
        }
    }

    pub fn value(&self) -> u8 {
        self.value
    }
}

impl TryFrom<u8> for Semitone {
    type Err = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl<'a> TryFrom<&'a str> for Semitone {
    type Err = &'static str;

    fn try_from(value: &'a str) -> Result<Self, Self::Err> {
        Self::from_str(value)
    }
}

impl PartialEq<u8> for Semitone {
    fn eq(&self, other: &u8) -> bool {
        self.value == *other
    }
}

impl<'a> PartialEq<&'a str> for Semitone {
    fn eq(&self, other: &&'a str) -> bool {
        self.to_letter() == *other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    quickcheck! {
        fn value_range(x: u8) -> bool {
            if x >= 1 && x <= 12 {
                x == Semitone::new(x).unwrap().value()
            } else {
                "Value out of range for semitone (1-12)" == Semitone::new(x).unwrap_err()
            }
        }
    }
}
