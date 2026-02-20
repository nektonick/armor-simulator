use crate::armor::Armor;
use crate::biome::{Biome, process_biome};
use crate::enemy::Enemy;
use crate::{biome, fight};

pub struct EncounterGenerator;

impl EncounterGenerator {
    pub fn new() -> EncounterGenerator {
        todo!()
    }

    pub fn generate_next_encounter(&mut self) -> Encounter {
        todo!()
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
