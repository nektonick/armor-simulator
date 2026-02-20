#[allow(unused)]
#[derive(Clone, Debug)]
pub struct Enemy {
    pub r#type: EnemyType,
    pub veteran: bool,
}

#[allow(unused)]
#[derive(Clone, Debug)]
pub enum EnemyType {
    Wolf,
    Goblin,
    Bandit,
}
