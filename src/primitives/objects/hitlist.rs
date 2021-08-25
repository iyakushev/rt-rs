use crate::primitives::traits::{HitRecord, Solid};


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

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}


impl Solid for CollisionList {
    fn hit(&self, ray: &crate::primitives::ray::Ray, pos_min: f64, pos_max: f64, record: &mut crate::primitives::traits::HitRecord) -> bool {
        let mut hit_anything = false;
        let mut tmp_record: HitRecord = Default::default();
        let mut closest_so_far = pos_max;

        for obj in &self.objects {
            if obj.hit(ray, pos_min, closest_so_far, &mut tmp_record) {
                hit_anything = true;
                closest_so_far = tmp_record.pos;
            }
        } 
        if hit_anything {
            *record = tmp_record;
        }
        hit_anything
    }
}
