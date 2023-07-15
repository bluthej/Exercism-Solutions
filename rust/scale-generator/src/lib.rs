// You should change this.
//
// Depending on your implementation, there are a variety of potential errors
// which might occur. They aren't checked by the test suite in order to
// allow the greatest freedom of implementation, but real libraries should
// provide useful, descriptive errors so that downstream code can react
// appropriately.
//
// One common idiom is to define an Error enum which wraps all potential
// errors. Another common idiom is to use a helper type such as failure::Error
// which does more or less the same thing but automatically.
#[derive(Debug)]
pub struct Error;

pub struct Scale {
    tonic: String,
    intervals: Vec<char>,
}

const TONICS_WITH_FLATS: [&str; 12] = [
    "F", "Bb", "Eb", "Ab", "Db", "Gb", "d", "g", "c", "f", "bb", "eb",
];
const FLATS: [&str; 12] = [
    "A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab",
];
const SHARPS: [&str; 12] = [
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        Ok(Scale {
            tonic: tonic.to_string(),
            intervals: intervals.chars().collect(),
        })
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        let intervals = vec!['m'; 12];
        Ok(Scale {
            tonic: tonic.to_string(),
            intervals,
        })
    }

    pub fn enumerate(&self) -> Vec<String> {
        let possible_notes = if TONICS_WITH_FLATS.contains(&self.tonic.as_str()) {
            FLATS
        } else {
            SHARPS
        };
        let mut iter = possible_notes
            .iter()
            .cycle()
            .skip_while(|&n| n.to_uppercase() != self.tonic.to_uppercase());
        let mut notes = vec![iter.next().unwrap().to_string()];
        for interval in &self.intervals {
            let n = "mMA".find(*interval).unwrap();
            let next = iter.nth(n).unwrap();
            notes.push(next.to_string());
        }
        notes
    }
}
