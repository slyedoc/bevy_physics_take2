#[allow(clippy::type_complexity)]
mod body;
mod bounds;
mod collider;
mod contact;
mod debug;
mod intersect;
mod phases;

pub use body::*;
pub use bounds::*;
pub use collider::*;
pub use contact::*;
pub use debug::*;
pub use intersect::*;
pub use phases::*;

use bevy::{ecs::schedule::ShouldRun, prelude::*, transform::TransformSystem};
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_polyline::PolylinePlugin;

#[derive(Inspectable)]
pub struct PhysicsConfig {
    pub enabled: bool,
    pub debug: bool,
    pub detection: CollisionDetection,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        PhysicsConfig {
            enabled: true,
            debug: true,
            detection: CollisionDetection::Dynamic,
        }
    }
}

#[derive(Inspectable)]
pub enum CollisionDetection {
    Static,
    Dynamic, //Continuous,
}

#[derive(Default)]
pub struct PhysicsTime {
    pub time: f32,
}

/// The names of system labels for run order
#[derive(SystemLabel, Clone, Hash, Debug, Eq, PartialEq)]
pub enum Phases {
    Setup,
    Dynamics,
    Broad,
    Narrow,
    Resolve,
    UpdatePosition,
    Debug,
}

// Not using Fixed time right now, want to develop hot path used right now
//const FIXED_TIMESTEP: f64 = 1.0 / 60.0;
//.with_run_criteria(FixedTimestep::step(FIXED_TIMESTEP))
// Update update_time_system if you add this back

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PolylinePlugin)
            .init_resource::<PhysicsConfig>()
            .init_resource::<PhysicsTime>()
            .init_resource::<PhysicsReport>()
            .add_event::<BroadContact>()
            .add_event::<Contact>()
            .register_inspectable::<Body>()
            .register_inspectable::<Aabb>()
            .register_inspectable::<GlobalAabb>()
            .register_inspectable::<ColliderType>()
            .register_inspectable::<ColliderSphere>()
            
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new()
                    .label(Phases::Setup)
                    .after(TransformSystem::TransformPropagate)                    
                    .with_system(spawn_body.label("setup_1"))
                    .with_system(spawn::<ColliderSphere>.label("setup_2").after("setup_1"))
                    .with_system(update_body.label("setup_3").after("setup_2"))
                    .with_system(update_aabb.label("setup_3").after("setup_2"))
                    .with_system(update_time_system),
            )
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new()
                    .after(Phases::Setup)
                    .with_run_criteria(run_physics)
                    .with_system(dynamics_system.label(Phases::Dynamics))
                    .with_system(broadphase_system.label(Phases::Broad).after(Phases::Dynamics))
                    .with_system(narrow_system.label(Phases::Narrow).after(Phases::Broad))
                    .with_system(resolve_system.label(Phases::Resolve).after(Phases::Narrow))
                    .with_system(
                        update_body_system
                            .label(Phases::UpdatePosition)
                            .after(Phases::Resolve),
                    ),
            )
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new()
                    .label(Phases::Debug)
                    .after(Phases::UpdatePosition)
                    .with_run_criteria(run_debug)
                    .with_system(report_system)
                    .with_system(draw_contacts_system),
            );
    }
}

fn run_physics(config: Res<PhysicsConfig>) -> ShouldRun {
    if config.enabled {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn run_debug(config: Res<PhysicsConfig>) -> ShouldRun {
    if config.debug {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn update_time_system(time: Res<Time>, mut pt: ResMut<PhysicsTime>) {
    pt.time = time.delta_seconds();
}

#[allow(clippy::type_complexity)]
pub fn spawn_body(mut query: Query<&mut Body, Added<Body>>) {
    for mut body in query.iter_mut() {
        body.inv_mass = match body.mass {
            Mass::Static => 0.0,
            Mass::Value(v) => 1.0 / v,
        };
    }
}



pub fn update_body(mut query: Query<(&mut Body, &GlobalTransform)>) {
    for (mut body, trans) in query.iter_mut() {
        body.center_of_mass_world = trans.translation + trans.rotation * body.center_of_mass;
        let orientation = Mat3::from_quat(trans.rotation);
        body.inverse_inertia_tensor_world =
            orientation * body.inverse_inertia_tensor_local * orientation.transpose();
    }
}

pub fn update_aabb(mut query: Query<(&GlobalTransform, &Aabb, &mut GlobalAabb)>) {
    for (trans, aabb, mut global_aabb) in query.iter_mut() {

        // TODO: We dont account for rotation yet, but spheres dont need it
        global_aabb.minimums = trans.translation + aabb.minimums;
        global_aabb.maximums = trans.translation + aabb.maximums;
    }
}

/// Spawns a collider type for each collider entity
#[allow(clippy::type_complexity)]
pub fn spawn<T: 'static + Component + Collider>(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Body, &T), (With<Body>, Added<T>)>,
) {
    for (e, mut body, collider) in query.iter_mut() {
        body.center_of_mass = collider.get_center_of_mass();
        body.inertia_tensor = collider.get_inertia_tensor();
        body.inverse_inertia_tensor_local = body.inertia_tensor.inverse() * body.inv_mass;

        commands
            .entity(e)
            .insert(collider.get_type())
            .insert(collider.get_aabb())
            .insert( GlobalAabb::default()); // will be set by update_aabb
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
