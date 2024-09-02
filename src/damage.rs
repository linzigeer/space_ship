use bevy::prelude::Component;

///碰撞伤害，凡是能给别的物体带去碰撞伤害的实体应该拥有这个组件
#[derive(Component, Debug, Default)]
pub struct Damage {
    pub damage: f32,
}

impl Damage {
    pub fn new(damage: f32) -> Self {
        Self { damage }
    }
}