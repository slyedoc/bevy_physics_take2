use bevy::{prelude::Entity, math::Vec3};

#[derive(Debug)]
pub struct BroadContact {
    pub a: Entity,
    pub b: Entity,
}

#[derive(Debug)]
pub struct Contact {
    pub a: Entity,
    pub b: Entity,
    pub world_point_a: Vec3,
    pub world_point_b: Vec3,
    pub local_point_a: Vec3,
    pub local_point_b: Vec3,
    pub normal: Vec3,
    pub separation_dist: f32,
    pub time_of_impact: f32,
}