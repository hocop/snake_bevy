use bevy::prelude::*;


#[derive(Component, Debug, PartialEq)]
#[component(immutable)]
#[require(HasEaten, LocalDirection)]
pub struct Head {
    pub next: Entity,
}


#[derive(Component, Debug, PartialEq, Default)]
#[component(immutable)]
pub enum LocalDirection {
    Left,
    #[default]
    Forward,
    Right,
}

#[derive(Component, Debug, PartialEq, Deref, DerefMut, Default)]
pub struct HasEaten(pub bool);


#[derive(Component, Debug, PartialEq)]
#[component(immutable)]
pub struct Body {
    pub prev: Entity,
    pub next: Entity,
}


#[derive(Component, Debug, PartialEq)]
#[component(immutable)]
pub struct Tail {
    pub prev: Entity,
    pub head: Entity,
}
