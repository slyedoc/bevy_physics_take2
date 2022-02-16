//use bevy_polyline::{Polyline, PolylineBundle, PolylineMaterial, PolylinePlugin};

pub fn draw_contacts_system() {

}


use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::{Body, PhysicsTime, BroadContact, Contact};


#[derive(Inspectable, Default, Debug, Copy, Clone)]
// TODO: Make this disable so user knows they can't change anything
pub struct PhysicsReport {
    time: f32,
    #[inspectable()]
    bodies: usize,
    manifolds: usize,
    broad_contacts: usize,
    narrow_contacts: usize,
    constraint: usize,
}

pub fn report_system(
    pt: Res<PhysicsTime>,
    bodies: Query<(&Body, &Transform)>,
    mut collision_pairs: EventReader<BroadContact>,
    mut contacts: EventReader<Contact>,
    //manifolds: Query<&Manifold>,
    //constraint_penetrations: Query<&ConstraintPenetration>,
    mut report: ResMut<PhysicsReport>,
) {
    report.time = pt.time;
    report.bodies = bodies.iter().count();
    //report.manifolds = manifolds.iter().count();
    report.broad_contacts = collision_pairs.iter().count();
    report.narrow_contacts = contacts.iter().count();
    //report.constraint = constraint_penetrations.iter().count();
}
