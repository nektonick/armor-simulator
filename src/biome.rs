use std::collections::HashSet;

pub struct Biome {
    modifiers: HashSet<Modifier>,
}

pub enum Modifier {
    ExtremelyHot,
    ExtremelyCold,
}
