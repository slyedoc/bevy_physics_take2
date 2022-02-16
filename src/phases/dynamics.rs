use bevy::{
    math::Vec3,
    prelude::*,
};

use crate::{Body, PhysicsTime};

pub fn dynamics_system(mut query: Query<&mut Body>, pt: Res<PhysicsTime>) {
    for mut body in query.iter_mut() {

        // Apply Gravity, it needs to be an impluse
        let mass = 1.0 / body.inv_mass;
        let gravey_impluse = Vec3::new(0.0, -10.0, 0.0) * mass * pt.time;
        body.apply_impulse_linear( gravey_impluse);
    }
}

