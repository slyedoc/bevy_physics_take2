//use bevy_polyline::{Polyline, PolylineBundle, PolylineMaterial, PolylinePlugin};
mod aabb;
mod report;

pub use aabb::*;
pub use report::*;

use bevy::{ecs::schedule::ShouldRun, prelude::*, render::primitives::Aabb};

use crate::{Phases, PhysicsConfig};
pub struct PhysicsDebugPlugin;

impl Plugin for PhysicsDebugPlugin {
    fn build(&self, app: &mut App) {
        let aabb = Aabb::default();
        app.add_plugin(GlobalAabbMaterialPlugin)
            .init_resource::<PhysicsReport>()
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new()
                    .label(Phases::Debug)
                    .after(Phases::UpdatePosition)
                    .with_run_criteria(run_debug)
                    .with_system(report_system),
            );
    }
}

fn run_debug(config: Res<PhysicsConfig>) -> ShouldRun {
    if config.debug {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}
