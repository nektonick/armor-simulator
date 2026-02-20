use crate::loot::RegularResourceCount;
use std::collections::HashSet;

pub struct Armor {
    pub regular_upgrades: HashSet<RegularUpgrade>,
    pub special_upgrades: HashSet<SpecialUpgrade>,
}

pub enum RegularUpgrade {
    TemperatureIsolation(TemperatureIsolation),
    PhysicalDamageResistance(PhysicalDamageResistance),
    ReducedWeight,
}

impl RegularUpgrade {
    pub fn get_price(&self) -> RegularResourceCount {
        match self {
            RegularUpgrade::TemperatureIsolation(_) => RegularResourceCount(1),
            RegularUpgrade::PhysicalDamageResistance(_) => RegularResourceCount(1),
            RegularUpgrade::ReducedWeight => RegularResourceCount(1),
        }
    }
}

pub enum TemperatureIsolation {
    Heat,
    Cold,
}

pub enum PhysicalDamageResistance {
    Blunt,
    Piercing,
}

pub enum SpecialUpgrade {}
