use rand::Rng;

use crate::utils::{Point3, Vec3};

const POINT_COUNT: usize = 256;

/// Perlin noise generator. To be used with
/// textures and perhaps procedural generation
#[derive(Debug, Clone)]
pub struct Perlin {
    rand_vec: [Vec3; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Perlin {
        let mut rand_vec: [Vec3; POINT_COUNT] = [Vec3::origin(); POINT_COUNT];

        for i in 0..POINT_COUNT {
            rand_vec[i] = Vec3::random_unit_vector();
        }

        let mut perm_x: [i32; POINT_COUNT] = [0; _];
        let mut perm_y: [i32; POINT_COUNT] = [0; _];
        let mut perm_z: [i32; POINT_COUNT] = [0; _];

        perlin_generate_perm(&mut perm_x);
        perlin_generate_perm(&mut perm_y);
        perlin_generate_perm(&mut perm_z);

        Perlin {
            rand_vec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    /// This function generates Perlin noise using the array weight method
    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::origin(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let di_num = di as i32;
                    let dj_num = dj as i32;
                    let dk_num = dk as i32;
                    c[di][dj][dk] = self.rand_vec[(self.perm_x[((i + di_num) & 255) as usize]
                        ^ self.perm_y[((j + dj_num) & 255) as usize]
                        ^ self.perm_z[((k + dk_num) & 255) as usize])
                        as usize];
                }
            }
        }

        return perlin_interp(c, u, v, w);
    }

    /// Applies noise multiple times, to a certain depth
    pub fn turbulence(&self, p: &Point3, depth: usize) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p = 2.0 * temp_p.clone();
        }

        accum.abs()
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

fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    // Now Hermitian Smoothing is here
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);

    let mut accum = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let i_float = i as f64;
                let j_float = j as f64;
                let k_float = k as f64;

                let weight = Vec3::new(u - i_float, v - j_float, w - k_float);

                accum += (i_float * uu + (1.0 - i_float) * (1.0 - uu))
                    * (j_float * vv + (1.0 - j_float) * (1.0 - vv))
                    * (k_float * ww + (1.0 - k_float) * (1.0 - ww))
                    * c[i][j][k].dot(&weight);
            }
        }
    }
    return accum;
}
