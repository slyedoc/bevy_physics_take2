use bevy::prelude::*;

use crate::{
    sphere_sphere_dynamic, Body, BroadContact, ColliderSphere, ColliderType, CollisionDetection,
    Contact, PhysicsConfig, PhysicsTime,
};

pub fn narrow_system(
    query: Query<(&mut GlobalTransform, &mut Body, &ColliderType)>,
    spheres: Query<&ColliderSphere>,
    mut broad_contacts: EventReader<BroadContact>,
    mut contacts: EventWriter<Contact>,
    config: Res<PhysicsConfig>,
    pt: Res<PhysicsTime>,
) {
    // let mut pairs = query.iter_combinations_mut();
    // while let Some([(a, mut trans_a, mut body_a, type_a), (b, mut trans_b, mut body_b, type_b)]) =
    //     pairs.fetch_next()
    // {
    for pair in broad_contacts.iter() {
        unsafe {
            let (mut trans_a, mut body_a, type_a) = query.get_unchecked(pair.a).unwrap();
            let (mut trans_b, mut body_b, type_b) = query.get_unchecked(pair.b).unwrap();
            match (type_a, type_b) {
                (ColliderType::Sphere, ColliderType::Sphere) => {
                    let sphere_a = spheres.get(pair.a).unwrap();
                    let sphere_b = spheres.get(pair.b).unwrap();

                    match config.detection {
                        CollisionDetection::Static => {
                            let ab = trans_b.translation - trans_a.translation;
                            let radius_ab = sphere_a.radius + sphere_b.radius;
                            let radius_ab_sq = radius_ab * radius_ab;
                            let ab_len_sq = ab.length_squared();
                            if ab_len_sq <= radius_ab_sq {
                                let normal = ab.normalize();
                                let ab = trans_a.translation - trans_b.translation;
                                let separation_dist =
                                    ab.length() - (sphere_a.radius + sphere_b.radius);

                                // convert world space contacts to local space
                                contacts.send(Contact {
                                    a: pair.a,
                                    b: pair.b,
                                    world_point_a: trans_a.translation + (normal * sphere_a.radius),
                                    world_point_b: trans_b.translation - (normal * sphere_b.radius),
                                    normal,
                                    local_point_a: Vec3::ZERO,
                                    local_point_b: Vec3::ZERO,
                                    separation_dist,
                                    time_of_impact: 0.0,
                                });
                            }
                        }
                        CollisionDetection::Dynamic => {
                            if let Some((world_point_a, world_point_b, time_of_impact)) =
                                sphere_sphere_dynamic(
                                    sphere_a.radius,
                                    sphere_b.radius,
                                    &body_a,
                                    &body_b,
                                    pt.time,
                                )
                            {
                                // step bodies forward to get local space collision points
                                body_a.update(&mut trans_a, time_of_impact);
                                body_b.update(&mut trans_b, time_of_impact);

                                // convert world space contacts to local space
                                let local_point_a = body_a.world_to_local(&trans_a, world_point_a);
                                let local_point_b = body_b.world_to_local(&trans_b, world_point_b);

                                let normal =
                                    (trans_a.translation - trans_b.translation).normalize();

                                // unwind time step
                                body_a.update(&mut trans_a, -time_of_impact);
                                body_b.update(&mut trans_b, -time_of_impact);

                                // calculate the separation distance
                                let ab = trans_a.translation - trans_b.translation;
                                let separation_dist =
                                    ab.length() - (sphere_a.radius + sphere_b.radius);

                                contacts.send(Contact {
                                    a: pair.a,
                                    b: pair.b,
                                    world_point_a,
                                    world_point_b,
                                    local_point_a,
                                    local_point_b,
                                    normal,
                                    separation_dist,
                                    time_of_impact,
                                });
                            }
                        }
                    }
                }
            }
        }
    }
}
