pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];
pub fn score(dice: Dice, category: Category) -> u8 {
    let times = |n: u8| dice.into_iter().filter(|i| i == &n).sum::<u8>();
    let mut dice = dice;
    dice.sort();
    match category {
        Category::Ones => times(1),
        Category::Twos => times(2),
        Category::Threes => times(3),
        Category::Fours => times(4),
        Category::Fives => times(5),
        Category::Sixes => times(6),
        Category::FullHouse => {
            if dice[0] < dice[4]
                && ((dice[0] == dice[1] && dice[2] == dice[4])
                    || (dice[0] == dice[2] && dice[3] == dice[4]))
            {
                dice.into_iter().sum()
            } else {
                0
            }
        }
        Category::FourOfAKind => {
            if dice[0] == dice[3] || dice[1] == dice[4] {
                4 * dice[2]
            } else {
                0
            }
        }
        Category::LittleStraight => {
            if dice == [1, 2, 3, 4, 5] {
                30
            } else {
                0
            }
        }
        Category::BigStraight => {
            if dice == [2, 3, 4, 5, 6] {
                30
            } else {
                0
            }
        }
        Category::Choice => dice.into_iter().sum(),
        Category::Yacht => {
            if dice[0] == dice[4] {
                50
            } else {
                0
            }
        }
    }
}
