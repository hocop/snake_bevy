use bevy::prelude::*;

use crate::actions::HUDAction;


// ==================== Common ================
#[derive(Component, Debug, Default, PartialEq)]
#[component(immutable)]
pub enum ButtonStyle {
    #[default]
    Idle,
    Active,
    Hover,
}

// ================= Simple button ==================
#[derive(Component, Debug)]
#[require(Button, ButtonStyle)]
pub struct SimpleButton {
    pub action: HUDAction
}

impl SimpleButton {
    pub fn new(action: HUDAction) -> Self {
        Self {action}
    }
}

// ================== Multiple choice ==================
#[derive(Component, Debug)]
#[require(Button, ButtonStyle)]
pub struct MultipleChoice<T> {
    pub choice: T
}


// ================== Toggle ==================
#[derive(Component, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
#[require(Button, ButtonStyle)]
pub enum Toggle {
    FastForward,
    SomeOtherToggle, // Add more toggles as needed
}
