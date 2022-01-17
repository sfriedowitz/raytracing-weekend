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
    pub fn new(min: Vec3, max: Vec3, material: impl Into<Material>) -> Self {
        let mat = material.into();

        let mut sides = HittableList::new();
        sides.push(XYRectangle::new(min.x, max.x, min.y, max.y, max.z, mat.clone()));
        sides.push(XYRectangle::new(min.x, max.x, min.y, max.y, min.z, mat.clone()));
        sides.push(XZRectangle::new(min.x, max.x, min.z, max.z, max.y, mat.clone()));
        sides.push(XZRectangle::new(min.x, max.x, min.z, max.z, min.y, mat.clone()));
        sides.push(YZRectangle::new(min.y, max.y, min.z, max.z, max.x, mat.clone()));
        sides.push(YZRectangle::new(min.y, max.y, min.z, max.z, min.x, mat.clone()));
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
