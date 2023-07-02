#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    if goal > u8::max(capacity_1, capacity_2) {
        return None;
    }
    let (start_capacity, other_capacity, start_bucket, other_bucket) = match start_bucket {
        Bucket::One => (capacity_1, capacity_2, Bucket::One, Bucket::Two),
        Bucket::Two => (capacity_2, capacity_1, Bucket::Two, Bucket::One),
    };
    if other_capacity == goal {
        return Some(BucketStats {
            moves: 2,
            goal_bucket: other_bucket,
            other_bucket: start_capacity,
        });
    }
    let (mut start_state, mut other_state) = (start_capacity, 0);
    for moves in 1.. {
        if start_state == goal {
            return Some(BucketStats {
                moves,
                goal_bucket: start_bucket,
                other_bucket: other_state,
            });
        } else if other_state == goal {
            return Some(BucketStats {
                moves,
                goal_bucket: other_bucket,
                other_bucket: start_state,
            });
        } else if start_state == 0 && other_state == 0 {
            return None;
        }
        if other_state == other_capacity {
            other_state = 0;
        } else if start_state == 0 {
            start_state = start_capacity;
        } else {
            let diff = start_state.min(other_capacity - other_state);
            start_state -= diff;
            other_state += diff;
        }
    }
    None
}
