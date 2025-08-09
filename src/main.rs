use ray_tracing::{
    environment::Camera,
    objects::{HitList, Sphere},
    util::Point3,
};

fn main() {
    // Image

    let mut world = HitList::new();

    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    // Make cam mutable to change its behaviors
    let cam = Camera::new(16.0 / 9.0, 1080);

    cam.render(&world);
}
