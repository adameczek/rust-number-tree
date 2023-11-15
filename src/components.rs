use bevy::prelude::*;

#[derive(Component, Debug, PartialEq, Clone, Copy, Reflect, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[reflect(Component, Default, PartialEq)]
pub struct NumberNode {
    pub value: i32,
}

#[derive(Component, Reflect)]
pub struct Value(i32);
