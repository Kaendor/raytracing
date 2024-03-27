use glam::{vec3, U16Vec3, Vec3};

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn color(&self) -> U16Vec3 {
        let distance = shortestDistanceToSurface(self.origin, self.direction, 0., 100.);

        let color = if distance > 100. - EPSILON {
            let unit_direction = self.direction / self.direction.length();
            let a = 0.5 * (unit_direction.y + 1.0);

            let color = ((1.0 - a) * Vec3::ONE + a * Vec3::new(0.5, 0.7, 1.0)) * 255.;

            color.as_u16vec3()
        } else {
            let p = self.origin + distance * self.direction;

            let ambiant = vec3(0.2, 0.2, 0.2);
            let diffuse = vec3(0.7, 0.2, 0.2);
            let specular = vec3(1., 1., 1.);
            let shininess = 10.;

            phong(ambiant, diffuse, specular, shininess, p, self.origin)
        };

        color
    }
}

const EPSILON: f32 = 0.001;

fn estimate_normals(p: Vec3) -> Vec3 {
    vec3(
        sdf_scene(vec3(p.x + EPSILON, p.y, p.z)) - sdf_scene(vec3(p.x - EPSILON, p.y, p.z)),
        sdf_scene(vec3(p.x, p.y + EPSILON, p.z)) - sdf_scene(vec3(p.x, p.y - EPSILON, p.z)),
        sdf_scene(vec3(p.x, p.y, p.z + EPSILON)) - sdf_scene(vec3(p.x, p.y, p.z - EPSILON)),
    )
    .normalize()
}

fn phong_light_contrib(
    diffuse: Vec3,
    specular: Vec3,
    alpha: f32,
    target: Vec3,
    camera: Vec3,
    light: Vec3,
    intensity: Vec3,
) -> Vec3 {
    let n = estimate_normals(target);
    let l = (light - target).normalize();
    let v = (camera - target).normalize();
    let r = (-l - 2. * n.dot(-l) * n).normalize();

    let dot_ln = l.dot(n);
    let dot_rv = r.dot(v);

    if dot_ln < 0. {
        return Vec3::ZERO;
    }

    if dot_rv < 0. {
        return intensity * diffuse * dot_ln;
    }

    intensity * (diffuse * dot_ln + specular * dot_rv.powf(alpha))
}

fn phong(
    ambiant: Vec3,
    diffuse: Vec3,
    specular: Vec3,
    alpha: f32,
    target: Vec3,
    camera: Vec3,
) -> U16Vec3 {
    let ambiant_light = 0.5 * vec3(1., 1., 1.);
    let mut color = ambiant_light * ambiant;

    let light_pos = vec3(4., 2., 4.);
    let intensity = vec3(0.4, 0.4, 0.4);

    color += phong_light_contrib(
        diffuse, specular, alpha, target, camera, light_pos, intensity,
    );

    (color * 255.).as_u16vec3()
}

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
