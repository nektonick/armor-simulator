use crate::loot::RegularResourceCount;
use std::collections::HashSet;

pub struct Armor {
    pub regular_upgrades: HashSet<RegularUpgrade>,
    #[allow(unused)]
    pub special_upgrades: HashSet<SpecialUpgrade>,
}

impl Armor {
    pub fn new() -> Self {
        Self {
            regular_upgrades: Default::default(),
            special_upgrades: Default::default(),
        }
    }
    pub fn install_upgrade(&mut self, upgrade: RegularUpgrade) {
        self.regular_upgrades.insert(upgrade);
    }
}

#[allow(unused)]
#[derive(Eq, Hash, PartialEq)]
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

#[allow(unused)]
#[derive(Eq, Hash, PartialEq)]
pub enum TemperatureIsolation {
    Heat,
    Cold,
}

#[allow(unused)]
#[derive(Eq, Hash, PartialEq)]
pub enum PhysicalDamageResistance {
    Blunt,
    Piercing,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum SpecialUpgrade {}
