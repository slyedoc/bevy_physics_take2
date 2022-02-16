use bevy::prelude::*;
use crate::{Body, Contact};

pub fn resolve_system(
    mut contacts: EventReader<Contact>,
    query: Query<(&mut Body, &mut GlobalTransform)>,
) {
    for contact in contacts.iter() {
        unsafe {

            let (mut body_a, mut trans_a) = query.get_unchecked(contact.a).unwrap();
            let (mut body_b, mut trans_b) = query.get_unchecked(contact.b).unwrap();

            let elasticity = body_a.elasticity * body_b.elasticity;
            let total_inv_mass = body_a.inv_mass + body_b.inv_mass;
        
            let ra = contact.world_point_a - body_a.center_of_mass_world;
            let rb = contact.world_point_b - body_b.center_of_mass_world;
        
            let angular_j_a = (body_a.inverse_inertia_tensor_world * ra.cross(contact.normal)).cross(ra);
            let angular_j_b = (body_b.inverse_inertia_tensor_world * rb.cross(contact.normal)).cross(rb);
            let angular_factor = (angular_j_a + angular_j_b).dot(contact.normal);
        
            // Get the world space velocity of the motion and rotation
            let vel_a = body_a.linear_velocity + body_a.angular_velocity.cross(ra);
            let vel_b = body_b.linear_velocity + body_b.angular_velocity.cross(rb);
        
            // Calculate the collion impulse
            let vab = vel_a - vel_b;
            let impluse_j = -(1.0 + elasticity) * vab.dot(contact.normal) / (total_inv_mass + angular_factor);
            let impluse_vec_j = contact.normal * impluse_j;

            body_a.apply_impulse(contact.world_point_a, impluse_vec_j);
            body_b.apply_impulse(contact.world_point_b, -impluse_vec_j);

            // Calculate the friction impulse
            let friction = body_a.friction * body_b.friction;

            // Find the normal direction of the velocity with respoect to the normal of the collison
            let velocity_normal = contact.normal * contact.normal.dot(vab);
            let velocity_tangent = vab - velocity_normal;
        
            // Get the tangent velocities relative to the other body
            let relative_velocity_tangent = velocity_tangent.normalize();
        
            let inertia_a = (body_a.inverse_inertia_tensor_world * ra.cross(relative_velocity_tangent)).cross(ra);
            let inertia_b = (body_b.inverse_inertia_tensor_world * rb.cross(relative_velocity_tangent)).cross(rb);
            let inv_inertia = (inertia_a + inertia_b).dot(relative_velocity_tangent);
        
            // calculat the tangential impluse for friction
            let reduced_mass = 1.0 / (total_inv_mass + inv_inertia);
            let impluse_friction = velocity_tangent * (reduced_mass * friction);
        
            // TODO: Book didnt have this if check, but I was getitng velocity_tangent of zero leading to
            // a Vec3 Nan when normalized if perfectly lined up on ground
            if !impluse_friction.is_nan() {
                 // apply kinetic friction
                 body_a.apply_impulse(contact.world_point_a, -impluse_friction);
                 body_b.apply_impulse(contact.world_point_b, impluse_friction);
            }

            // Lets also move our colliding object to just outside of each other
            if contact.time_of_impact == 0.0 {
                let a_move_weight = body_a.inv_mass / total_inv_mass;
                let b_move_weight = body_b.inv_mass / total_inv_mass;

                let distance = contact.world_point_b - contact.world_point_a;

                trans_a.translation += distance * a_move_weight;
                trans_b.translation -= distance * b_move_weight;
            }
        }
    }
}

