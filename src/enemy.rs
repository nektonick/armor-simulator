pub struct Enemy {
    pub r#type: EnemyType,
    pub veteran: bool,
}

pub enum EnemyType {
    Wolf,
    Goblin,
    Bandit,
}
