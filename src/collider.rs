use bevy::{prelude::Component, math::{Vec3, Mat3}};
use bevy_inspector_egui::Inspectable;

use crate::Aabb;

#[derive(Component, Inspectable)]
pub enum ColliderType {
    Sphere,
}

pub trait Collider {
    fn get_type(&self) -> ColliderType;
    fn get_center_of_mass(&self) -> Vec3;
    fn get_inertia_tensor(&self) -> Mat3;
    fn get_aabb(&self) -> Aabb;
}

#[derive(Component, Inspectable)]
pub struct ColliderSphere {
    pub radius: f32
}

impl Collider for ColliderSphere {
    fn get_type(&self) -> ColliderType {
        ColliderType::Sphere
    }

    fn get_center_of_mass(&self) -> Vec3 {
        Vec3::ZERO
    }

    fn get_inertia_tensor(&self) -> Mat3 {
        let i = 2.0 * self.radius * self.radius / 5.0;
        Mat3::from_diagonal(Vec3::splat(i) )
    }

    fn get_aabb(&self) -> Aabb {
        Aabb {
            minimums: Vec3::new(-self.radius, -self.radius, -self.radius),
            maximums: Vec3::new(self.radius, self.radius, self.radius),
        }
    }
}

impl ColliderSphere {
    pub fn new(radius: f32) -> Self {
        ColliderSphere {
            radius
        }
    }
}