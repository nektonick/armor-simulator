use crate::armor::Armor;
use crate::enemy::Enemy;

pub fn process_fight(_character_armor: &Armor, _enemy: &Enemy) -> Result {
    todo!()
}

#[allow(unused)]
pub enum Result {
    CharacterWin,
    EnemyWin,
}
