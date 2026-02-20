use crate::armor::SpecialUpgrade;
use crate::encounter::Encounter;
use std::collections::HashSet;
use std::ops::AddAssign;

pub struct LootGenerator {}

impl LootGenerator {
    pub fn new() -> Self {
        todo!()
    }

    pub fn generate_loot_for_encounter(&mut self, _encounter: &Encounter) -> Loot {
        todo!()
    }
}

#[derive(Debug)]
pub struct Loot {
    pub special_upgrades: HashSet<SpecialUpgrade>,
    pub regular_resources: RegularResourceCount,
}

impl Loot {
    pub fn new() -> Self {
        Self {
            special_upgrades: Default::default(),
            regular_resources: RegularResourceCount(0),
        }
    }
}

impl AddAssign for Loot {
    fn add_assign(&mut self, rhs: Self) {
        self.special_upgrades.extend(rhs.special_upgrades);
        self.regular_resources.0 += rhs.regular_resources.0;
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RegularResourceCount(pub u32);
