use crate::armor::SpecialUpgrade;
use crate::encounter::Encounter;
use std::collections::HashSet;

pub struct LootGenerator {}

impl LootGenerator {
    pub fn new() -> Self {
        todo!()
    }

    pub fn generate_loot_for_encounter(&mut self, encounter: &Encounter) -> Loot {
        todo!()
    }
}

pub struct Loot {
    pub special_upgrades: HashSet<SpecialUpgrade>,
    pub regular_resources: RegularResourceCount,
}

pub struct RegularResourceCount(pub u32);
