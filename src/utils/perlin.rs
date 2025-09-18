use rand::Rng;

use crate::utils::Point3;

const POINT_COUNT: usize = 256;

/// Perlin noise generator. To be used with
/// textures and perhaps procedural generation
#[derive(Debug, Clone)]
pub struct Perlin {
    rand_float: [f64; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Perlin {
        let mut rng = rand::rng();
        let rand_float: [f64; POINT_COUNT] = rng.random();

        let mut perm_x: [i32; POINT_COUNT] = [0; _];
        let mut perm_y: [i32; POINT_COUNT] = [0; _];
        let mut perm_z: [i32; POINT_COUNT] = [0; _];

        perlin_generate_perm(&mut perm_x);
        perlin_generate_perm(&mut perm_y);
        perlin_generate_perm(&mut perm_z);

        Perlin {
            rand_float,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let i = ((4.0 * p.x()) as i32 & 255) as usize;
        let j = ((4.0 * p.y()) as i32 & 255) as usize;
        let k = ((4.0 * p.z()) as i32 & 255) as usize;

        return self.rand_float[(self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize];
    }
}

fn perlin_generate_perm(p: &mut [i32; POINT_COUNT]) {
    for i in 0..POINT_COUNT {
        p[i] = i as i32;
    }

    permute(p);
}

fn permute(p: &mut [i32; POINT_COUNT]) {
    let mut rng = rand::rng();

    for i in (1..POINT_COUNT - 1).rev() {
        let target = rng.random_range(0..=i);
        let tmp = p[i];
        p[i] = p[target];
        p[target] = tmp;
    }
}
