use crate::Ray;
use crate::Point;
use crate::Vector3;

pub enum Camera {
    OrthographicCamera(OrthographicCamera),
    StandardCamera(StandardCamera),
}

pub struct OrthographicCamera {
    pub x_resolution: u16, // NEED TO BE PRIVATE
    pub y_resolution: u16,
}

pub struct StandardCamera {

}

pub trait GeneratingViewRays {
    fn generate_viewport(&self) -> Vec<ViewRay>;
}

impl GeneratingViewRays for OrthographicCamera {
    fn generate_viewport(&self) -> Vec<ViewRay> {
        let mut view_rays : Vec<ViewRay> = vec![];

        let x_shift = self.x_resolution as f64 / 2f64;
        let y_shift = self.y_resolution as f64 / 2f64;
    
        for y in 0..self.y_resolution {
            for x in 0..self.x_resolution {
                let ray = Ray {
                    origin: Point {
                        x: x as f64 - x_shift,
                        y: y as f64 - y_shift,
                        z: 0f64,
                    },
                    direction: Vector3 {
                        x: 0f64,
                        y: 0f64,
                        z: -1f64,
                    },
                };
                view_rays.push(ViewRay {x:x, y:y, ray:ray});
            }
        }
        view_rays
    }
}

pub struct ViewRay {
    pub x: u16,
    pub y: u16,
    pub ray: Ray,
}
