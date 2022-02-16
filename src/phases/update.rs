use bevy::prelude::*;

use crate::{Body, PhysicsTime};

pub fn update_body_system(
    mut query: Query<(&mut GlobalTransform, &mut Body)>,
    pt: Res<PhysicsTime>,
) {
    for (mut t, mut body) in query.iter_mut() {
        body.update(&mut t, pt.time);
    }
}
