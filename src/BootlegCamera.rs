use glam::{vec2, vec3, Mat4, Vec2, Vec3};

#[derive(Debug)]
pub struct BootlegCamera {
    pub position: Vec3,
    pub up: Vec3,
    pub target: Vec3,
}

impl BootlegCamera {
    pub fn new(position: Vec3, up: Vec3, target: Vec3) -> Self {
        BootlegCamera {
            position,
            up,
            target,
        }
    }
    pub fn matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.target, self.up)
    }
}
