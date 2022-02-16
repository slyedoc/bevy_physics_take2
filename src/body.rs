use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

const MAX_ANGULAR_VELOCITY: f32 = 30.0;


#[derive(Component, Inspectable)]
pub struct Body {
    pub linear_velocity: Vec3,
    pub angular_velocity: Vec3,

    #[inspectable(min = 0.0, max = 1.0)]
    pub elasticity: f32,
    pub friction: f32,
    pub mass: Mass,

    // will be set by collider
    pub inv_mass: f32,
    pub center_of_mass: Vec3,
    pub inertia_tensor: Mat3,
    pub inverse_inertia_tensor_local: Mat3,

    // set each frame
    pub center_of_mass_world: Vec3,
    pub inverse_inertia_tensor_world: Mat3,
}

impl Default for Body {
    fn default() -> Self {
        Body {
            linear_velocity: Vec3::default(),
            angular_velocity: Vec3::default(),
            elasticity: 1.0,
            friction: 0.5,
            mass: Mass::Static,
            inv_mass: 0.0,
            center_of_mass: Vec3::ZERO,
            center_of_mass_world: Vec3::ZERO,
            inertia_tensor: Mat3::IDENTITY,
            inverse_inertia_tensor_local: Mat3::IDENTITY,
            inverse_inertia_tensor_world: Mat3::IDENTITY,
        }
    }
}
impl Body {

    pub fn centre_of_mass_world(&self, t: &GlobalTransform) -> Vec3 {
        t.translation + t.rotation * self.center_of_mass
    }
    
    pub fn world_to_local(&self, t: &GlobalTransform, world_point: Vec3) -> Vec3 {
        let tmp = world_point - self.center_of_mass_world;
        let inv_orientation = t.rotation.conjugate();
        inv_orientation * tmp
    }

    pub fn apply_impulse_linear(&mut self, impulse: Vec3) {
        if self.inv_mass == 0.0 {
            return;
        }
        self.linear_velocity += impulse * self.inv_mass;
    }

    pub fn apply_impulse_angular(&mut self, impulse: Vec3) {
        if self.inv_mass == 0.0 {
            return;
        }

        self.angular_velocity += self.inverse_inertia_tensor_world * impulse;
        // clamp angular velocity
        if self.angular_velocity.length_squared() > MAX_ANGULAR_VELOCITY * MAX_ANGULAR_VELOCITY {
            self.angular_velocity = self.angular_velocity.normalize() * MAX_ANGULAR_VELOCITY;
        }
    }

    pub fn apply_impulse(&mut self, impulse_point: Vec3, impulse: Vec3) {
        if self.inv_mass == 0.0 {
            return;
        }
        // impulse_point is in world space location of the applied impulse
        // impulse is in world space direction and magnitude of the impulse
        self.apply_impulse_linear(impulse);

        // applying impluses must produce torgues though the center of mass
        let r = impulse_point - self.center_of_mass_world;
        let dl = r.cross(impulse); // this is in world space
        self.apply_impulse_angular(dl);
    }

    pub fn update(&mut self, transform: &mut GlobalTransform, dt: f32) {
        // apply linear velocity
        transform.translation += self.linear_velocity * dt;

        // we have an angular velocity around the centre of mass, this needs to be converted to
        // relative body translation. This way we can properly update the rotation of the model
        
        let position_com = self.centre_of_mass_world(transform);
        let com_to_position = transform.translation - position_com;
        
        
        // total torque is equal to external applied torques + internal torque (precession)
        // T = T_external + omega x I * omega
        // T_external = 0 because it was applied in the collision response function
        // T = Ia = w x I * w
        // a = I^-1 (w x I * w)
        let orientation = Mat3::from_quat(transform.rotation);
        let inertia_tensor = orientation * self.inertia_tensor * orientation.transpose();
        let alpha = inertia_tensor.inverse()
            * (self
                .angular_velocity
                .cross(inertia_tensor * self.angular_velocity));
        self.angular_velocity += alpha * dt;

        // update orientation
        let d_angle = self.angular_velocity * dt;
        let angle = d_angle.length();
        let inv_angle = angle.recip();
        let dq = if inv_angle.is_finite() {
            Quat::from_axis_angle(d_angle * inv_angle, angle)
        } else {
            Quat::IDENTITY
        };
        transform.rotation = (dq * transform.rotation).normalize();

        // now get the new body position
        transform.translation = position_com + dq * com_to_position;
    }
}

#[derive(Component, Inspectable)]
pub enum Mass {
    Static,
    Value(f32),
}




