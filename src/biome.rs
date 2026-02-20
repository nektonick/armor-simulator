use crate::armor::Armor;
use std::collections::HashSet;

pub struct Biome {
    #[allow(unused)]
    modifiers: HashSet<Modifier>,
}

#[allow(unused)]
#[derive(Debug)]
pub enum Modifier {
    ExtremelyHot,
    ExtremelyCold,
}

pub fn process_biome(_armor: &Armor, _biome: &Biome) -> Result {
    todo!()
}

#[allow(unused)]
pub enum Result {
    Passed,
    Failed(FailureReason),
}

#[allow(unused)]
#[derive(Debug)]
pub enum FailureReason {
    Modified(Modifier),
}
