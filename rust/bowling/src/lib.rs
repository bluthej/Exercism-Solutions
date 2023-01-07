use std::vec;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    frames: Vec<Vec<u16>>,
}

impl BowlingGame {
    pub fn new() -> Self {
        let frames = Vec::with_capacity(10);
        BowlingGame { frames }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_complete() {
            return Err(Error::GameComplete);
        }

        let frames = &mut self.frames;
        let mut n = 10;
        if !frames.is_empty() {
            let last_frame = frames.last().unwrap();
            if last_frame.len() == 1 && last_frame[0] < 10 {
                n -= last_frame[0];
            }
            if frames.len() == 10
                && last_frame.len() == 2
                && last_frame[0] == 10
                && last_frame[1] < 10
            {
                n -= last_frame[1];
            }
        }
        if pins > n {
            return Err(Error::NotEnoughPinsLeft);
        }

        if frames.is_empty()
            || (frames.len() < 10
                && (frames.last().unwrap().len() == 2 || frames.last().unwrap()[0] == 10))
        {
            frames.push(vec![pins]);
            return Ok(());
        }

        let last_frame = frames.last_mut().unwrap();
        last_frame.push(pins);
        return Ok(());
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_complete() {
            return None;
        }
        let mut score = 0;
        let mut i = 0;
        let rolls: Vec<&u16> = self.frames.iter().flatten().collect();
        let mut frames = self.frames.clone();
        let last_frame = frames.pop().unwrap();
        for frame in frames {
            if frame[0] == 10 {
                score += 10;
                for j in 1..=2 {
                    match rolls.get(i + j) {
                        Some(&m) => score += m,
                        None => (),
                    }
                }
                i += 1;
            } else if frame[0] + frame[1] == 10 {
                score += 10 + rolls[i + 2];
                i += 2;
            } else {
                score += frame[0] + frame[1];
                i += 2;
            }
        }
        score += last_frame.iter().sum::<u16>();
        Some(score)
    }

    fn is_complete(&self) -> bool {
        if self.frames.len() < 10 {
            return false;
        }
        let last_frame = self.frames.last().unwrap();
        if last_frame.len() < 2 {
            return false;
        }
        if last_frame[0] + last_frame[1] == 10 || last_frame[0] == 10 {
            return last_frame.len() == 3;
        }
        true
    }
}
