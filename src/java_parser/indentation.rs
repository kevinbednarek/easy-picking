#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct Indentation {
    column: Column,
    character: Character,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct Column(u32);

impl Column {
    pub const fn new(column: u32) -> Self {
        Self(column)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct Character(u32);

impl Character {
    pub const fn new(characters: u32) -> Self {
        Self(characters)
    }
}