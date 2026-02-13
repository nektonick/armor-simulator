use crate::enemy::Enemy;

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
}