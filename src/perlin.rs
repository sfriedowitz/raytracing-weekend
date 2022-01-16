use rand::Rng;

use crate::vec::Vec3;

const POINT_COUNT: usize = 256;

fn permute(p: &mut [usize], n: usize) {
    let mut rng = rand::thread_rng();
    for i in (0..n as usize).rev() {
        let target = rng.gen_range(0..i + 1);
        p.swap(i, target);
    }
}

fn interpolate(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    let mut accum = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                accum += (i as f64 * uu + (1 - i) as f64 * (1.0 - uu))
                    * (j as f64 * vv + (1 - j) as f64 * (1.0 - vv))
                    * (k as f64 * ww + (1 - k) as f64 * (1.0 - ww))
                    * c[i][j][k].dot(weight);
            }
        }
    }
    accum
}

#[derive(Clone, Debug)]
pub struct Perlin {
    rand_vecs: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Self {
        Self {
            rand_vecs: Self::generate_rand_vecs(),
            perm_x: Self::generate_perm_vec(),
            perm_y: Self::generate_perm_vec(),
            perm_z: Self::generate_perm_vec(),
        }
    }

    pub fn turb(&self, point: Vec3, depth: usize) -> f64 {
        let mut accum = 0.0;
        let mut temp_point = point;
        let mut weight = 1.0;
        for _ in 0..depth {
            accum += weight * self.noise(temp_point);
            weight *= 0.5;
            temp_point *= 2.0;
        }
        f64::abs(accum)
    }

    fn noise(&self, point: Vec3) -> f64 {
        let u = point.x - f64::floor(point.x);
        let v = point.y - f64::floor(point.y);
        let w = point.z - f64::floor(point.z);

        let i = f64::floor(point.x) as usize;
        let j = f64::floor(point.y) as usize;
        let k = f64::floor(point.z) as usize;

        let mut c = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.rand_vecs[self.perm_x[(i + di) & (POINT_COUNT - 1)]
                        ^ self.perm_y[(j + dj) & (POINT_COUNT - 1)]
                        ^ self.perm_z[(k + dk) & (POINT_COUNT - 1)]]
                }
            }
        }

        interpolate(&c, u, v, w)
    }

    fn generate_rand_vecs() -> Vec<Vec3> {
        let mut rng = rand::thread_rng();
        let mut p = Vec::with_capacity(POINT_COUNT);
        for _ in 0..POINT_COUNT {
            p.push(
                Vec3::new(
                    -1.0 + 2.0 * rng.gen::<f64>(),
                    -1.0 + 2.0 * rng.gen::<f64>(),
                    -1.0 + 2.0 * rng.gen::<f64>(),
                )
                .normalize(),
            );
        }
        p
    }

    fn generate_perm_vec() -> Vec<usize> {
        let mut p = Vec::with_capacity(POINT_COUNT);
        for i in 0..POINT_COUNT {
            p.push(i);
        }
        permute(&mut p, POINT_COUNT);
        p
    }
}
