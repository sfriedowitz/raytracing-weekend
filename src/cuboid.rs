use crate::{
    aabb::AABB,
    hit::{Hit, HitRecord},
    hittable::{Hittable, HittableList},
    material::Material,
    ray::Ray,
    rectangle::{XYRectangle, XZRectangle, YZRectangle},
    vec::Vec3,
};

#[derive(Clone, Debug)]
pub struct Cuboid {
    min: Vec3,
    max: Vec3,
    sides: HittableList,
}

impl Cuboid {
    pub fn new(min: Vec3, max: Vec3, material: Material) -> Self {
        let sides = vec![
            XYRectangle::new(min.x, max.x, min.y, max.y, max.z, material.clone()).into(),
            XYRectangle::new(min.x, max.x, min.y, max.y, min.z, material.clone()).into(),
            XZRectangle::new(min.x, max.x, min.z, max.z, max.y, material.clone()).into(),
            XZRectangle::new(min.x, max.x, min.z, max.z, min.y, material.clone()).into(),
            YZRectangle::new(min.y, max.y, min.z, max.z, max.x, material.clone()).into(),
            YZRectangle::new(min.y, max.y, min.z, max.z, min.x, material).into(),
        ];
        Self { min, max, sides }
    }

    pub fn min(&self) -> Vec3 {
        self.min
    }

    pub fn max(&self) -> Vec3 {
        self.max
    }

    pub fn sides(&self) -> &[Hittable] {
        &self.sides
    }
}

impl Hit for Cuboid {
    fn hit(&self, r: &Ray, s_min: f64, s_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, s_min, s_max)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(AABB::new(self.min, self.max))
    }
}
