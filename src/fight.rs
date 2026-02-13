use crate::armor::Armor;
use crate::enemy::Enemy;

pub fn process_fight(character_armor: &Armor, enemy: &Enemy) -> FightResult{
    todo!()
}

pub enum FightResult {
    CharacterWin,
    EnemyWin,
}