// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]
use core::cmp::max;

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health > 0 {
            return None;
        }

        let health = 100;
        let level = self.level;
        let mana = if level >= 10 { Some(100) } else { None };
        Some(Player {
            health,
            mana,
            level,
        })
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            None => {
                self.health = self.health.saturating_sub(mana_cost);
                return 0;
            }
            Some(mana) => {
                if mana < mana_cost {
                    return 0;
                }
                self.mana = Some(mana - mana_cost);
                return 2 * mana_cost;
            }
        }
    }
}
