use std::collections::VecDeque;

pub struct RailFence {
    rails: u32,
}

impl RailFence {
    pub fn new(rails: u32) -> Self {
        Self { rails }
    }

    pub fn encode(&self, text: &str) -> String {
        let nrails = self.rails as usize;
        let mut vrail = vec![Vec::new(); nrails];
        for (c, r) in text.chars().zip(zigzag(nrails)) {
            vrail[r].push(c);
        }
        vrail.into_iter().flatten().collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let nrails = self.rails as usize;
        let mut vrail = vec![VecDeque::new(); nrails];
        for (c, r) in cipher.chars().zip(zigzag(nrails)) {
            vrail[r].push_back(c);
        }

        let mut chars = cipher.chars();
        for rail in &mut vrail {
            for l in rail.iter_mut() {
                if let Some(c) = chars.next() {
                    *l = c;
                }
            }
        }

        let mut res = String::with_capacity(cipher.len());
        for (_, r) in cipher.chars().zip(zigzag(nrails)) {
            if let Some(l) = vrail[r].pop_front() {
                res.push(l);
            }
        }
        res
    }
}

fn zigzag(n: usize) -> impl Iterator<Item = usize> {
    (0..n - 1).chain((1..n).rev()).cycle()
}
