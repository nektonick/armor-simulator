use crate::armor::Armor;
use crate::biome::{Biome, process_biome};
use crate::encounter::Slot::{First, Second, Third};
use crate::enemy::Enemy;
use crate::{biome, fight};
use std::collections::HashMap;

pub struct EncounterGenerator {
    skipped_encounters: HashMap<Slot, Encounter>,
}

#[allow(unused)]
#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Slot {
    First,
    Second,
    Third,
}

impl EncounterGenerator {
    pub fn new() -> EncounterGenerator {
        todo!()
    }

    pub fn generate_next_encounter(&mut self) -> Encounter {
        todo!()
    }

    pub fn skip_encounter(&mut self, encounter: Encounter) -> core::result::Result<(), Encounter> {
        for slot in [First, Second, Third] {
            if self.skipped_encounters.contains_key(&slot) {
                continue;
            }
            self.skipped_encounters.insert(slot, encounter);
            return Ok(());
        }
        Err(encounter)
    }

    pub fn get_skipped_encounters(&self) -> &HashMap<Slot, Encounter> {
        &self.skipped_encounters
    }

    pub fn return_to_skipped_encounter(&mut self, slot: Slot) -> Option<Encounter> {
        self.skipped_encounters.remove(&slot)
    }
}

pub struct Encounter {
    pub enemies: Vec<Enemy>,
    pub biome: Biome,
}

pub fn process_encounter(armor: &Armor, encounter: &Encounter) -> Result {
    if let biome::Result::Failed(reason) = process_biome(armor, &encounter.biome) {
        return Result::Failed(FailureReason::Biome(reason));
    }

    for enemy in encounter.enemies.iter() {
        if matches!(fight::process_fight(armor, enemy), fight::Result::EnemyWin) {
            return Result::Failed(FailureReason::Enemy((*enemy).clone()));
        }
    }
    Result::Passed
}

pub enum Result {
    Passed,
    Failed(FailureReason),
}

#[allow(unused)]
#[derive(Debug)]
pub enum FailureReason {
    Biome(biome::FailureReason),
    Enemy(Enemy),
}
