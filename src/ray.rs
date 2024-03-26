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
        let distance = shortestDistanceToSurface(self.origin, self.direction, 0., 100.);

        let color = if distance > 100. - EPSILON {
            let unit_direction = self.direction / self.direction.length();
            let a = 0.5 * (unit_direction.y + 1.0);

            let color = ((1.0 - a) * Vec3::ONE + a * Vec3::new(0.5, 0.7, 1.0)) * 255.;

            color.as_u16vec3()
        } else {
            U16Vec3::new(255, 0, 0)
        };

        color
    }
}

const EPSILON: f32 = 0.001;

fn shortestDistanceToSurface(camera: Vec3, direction: Vec3, start: f32, end: f32) -> f32 {
    let mut depth = start;

    for _i in 0..255 {
        let distance = sdf_scene(camera + depth * direction);

        if distance < EPSILON {
            return depth;
        }

        depth += distance;

        if depth >= end {
            return end;
        }
    }
    return end;
}

struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn sdf(&self, position: Vec3) -> f32 {
        (position - self.center).length() - self.radius
    }
}

fn sdf_scene(position: Vec3) -> f32 {
    // sphere at 0. 0.
    let sphere = Sphere {
        center: Vec3::new(0., 0., 0.),
        radius: 1.,
    };

    return sphere.sdf(position);
}
