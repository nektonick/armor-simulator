use crate::armor::{Armor, RegularUpgrade};
use crate::encounter::{EncounterGenerator, process_encounter};
use crate::loot::{Loot, LootGenerator};

mod armor;
mod biome;
mod encounter;
mod enemy;
mod fight;
mod loot;

fn main() {
    let mut game_data = GameData::new();

    loop {
        process_game_loop_iteration(&mut game_data);
    }
}

struct GameData {
    armor: Armor,
    resources: Loot,
    encounter_generator: EncounterGenerator,
    loot_generator: LootGenerator,
}

impl GameData {
    fn new() -> Self {
        Self {
            armor: Armor::new(),
            resources: Loot::new(),
            encounter_generator: EncounterGenerator::new(),
            loot_generator: LootGenerator::new(),
        }
    }
}

fn process_game_loop_iteration(game_data: &mut GameData) {
    let encounter = game_data.encounter_generator.generate_next_encounter();
    if let encounter::Result::Failed(reason) = process_encounter(&game_data.armor, &encounter) {
        println!("Encounter failed because of {reason:?}");
        return;
    }
    let loot = game_data
        .loot_generator
        .generate_loot_for_encounter(&encounter);
    game_data.resources += loot;
    println!("Your resources after encounter: {:?}", game_data.resources);
    process_upgrade(&mut game_data.armor, &mut game_data.resources);
}

fn process_upgrade(armor: &mut Armor, resources: &mut Loot) {
    // TODO: special upgrades
    loop {
        let Some(regular_upgrade) = select_upgrade() else {
            return;
        };

        let price = regular_upgrade.get_price();
        if resources.regular_resources < price {
            println!("Not enough resources");
            continue;
        }
        armor.install_upgrade(regular_upgrade);
        println!("Upgrade installed");
    }
}

fn select_upgrade() -> Option<RegularUpgrade> {
    println!("Select a upgrade:");
    todo!()
}
