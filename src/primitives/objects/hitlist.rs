use crate::primitives::traits::Solid;
use super::HitRecord;


pub struct CollisionList {
    objects: Vec<Box<dyn Solid>>
}

impl CollisionList {
    pub fn new() -> Self {
        CollisionList { objects: Vec::new() }
    }

    pub fn push(&mut self, obj: Box<dyn Solid>) {
        self.objects.push(obj);
    }

    pub fn _clear(&mut self) {
        self.objects.clear();
    }
}


impl Solid for CollisionList {
    fn hit(&self, ray: &crate::primitives::ray::Ray, pos_min: f64, pos_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = pos_max;
        let mut last_record = None;

        for obj in &self.objects {
            if let Some(tmp_record) = obj.hit(ray, pos_min, closest_so_far) {
                closest_so_far = tmp_record.pos;
                last_record = Some(tmp_record);
            }
        }
        last_record
    }
}
