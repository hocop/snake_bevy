use bevy::prelude::*;

use crate::grid::components::*;


#[derive(Component, Debug, PartialEq, Clone, Copy, Default)]
#[require(LocalDirection, GlobalDirection)]
pub struct Head {
    pub has_eaten: bool
}


#[derive(Component, Debug, PartialEq)]
#[component(immutable)]
pub struct Body {
    pub prev: Entity,
}
