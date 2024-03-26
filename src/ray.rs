use glam::{U16Vec3, Vec3};

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, delta: f32) -> Vec3 {
        self.origin + delta * self.direction
    }

    pub fn color(&self) -> U16Vec3 {
        let unit_direction = self.direction / self.direction.length();
        let a = 0.5 * (unit_direction.y + 1.0);

        let color = ((1.0 - a) * Vec3::ONE + a * Vec3::new(0.5, 0.7, 1.0)) * 255.;

        color.as_u16vec3()
    }
}
