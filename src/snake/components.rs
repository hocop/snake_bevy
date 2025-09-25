use bevy::prelude::*;

use crate::grid::components::*;


#[derive(Component, Debug, PartialEq, Clone, Copy)]
#[component(immutable)]
#[require(HasEaten, LocalDirection, GlobalDirection)]
pub struct Head {
    pub next: Entity,
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
