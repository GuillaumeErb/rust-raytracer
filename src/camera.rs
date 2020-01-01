use crate::Point;
use crate::Ray;
use crate::Vector3;

pub enum Camera {
    OrthographicCamera(OrthographicCamera),
    StandardCamera(StandardCamera),
}

pub struct OrthographicCamera {
    pub x_resolution: u16,
    pub y_resolution: u16,
}

pub struct StandardCamera {
    pub position: Point,
    pub direction: Vector3,
    pub up_direction: Vector3,
    pub field_of_view: f64,
    pub x_resolution: u16,
    pub y_resolution: u16,
}

impl Camera {
    pub fn x_resolution(&self) -> u16 {
        match self {
            Camera::OrthographicCamera(ref cam) => cam.x_resolution,
            Camera::StandardCamera(ref cam) => cam.x_resolution,
        }
    }

    pub fn y_resolution(&self) -> u16 {
        match self {
            Camera::OrthographicCamera(ref cam) => cam.y_resolution,
            Camera::StandardCamera(ref cam) => cam.y_resolution,
        }
    }
}

impl GeneratingViewRays for Camera {
    fn generate_viewport(&self) -> Vec<ViewRay> {
        match self {
            Camera::OrthographicCamera(ref cam) => cam.generate_viewport(),
            Camera::StandardCamera(ref cam) => cam.generate_viewport(),
        }
    }
}

pub trait GeneratingViewRays {
    fn generate_viewport(&self) -> Vec<ViewRay>;
}

impl GeneratingViewRays for OrthographicCamera {
    fn generate_viewport(&self) -> Vec<ViewRay> {
        let mut view_rays: Vec<ViewRay> = vec![];

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
                view_rays.push(ViewRay {
                    x: x,
                    y: y,
                    ray: ray,
                });
            }
        }
        view_rays
    }
}

impl GeneratingViewRays for StandardCamera {
    fn generate_viewport(&self) -> Vec<ViewRay> {
        let mut view_rays: Vec<ViewRay> = vec![];

        let t_n = self.direction.normalize();
        let b_n = self.direction.cross(&self.up_direction).normalize();
        let v_n = t_n.cross(&b_n);
        let g_x = (self.field_of_view / 2f64).tan();
        let g_y = g_x * (self.y_resolution as f64) / (self.x_resolution as f64);
        let q_x = b_n.times(2f64 * g_x / (self.x_resolution as f64));
        let q_y = v_n.times(2f64 * g_y / (self.y_resolution as f64));
        let p_1_m = t_n
            .minus(&b_n.times(1f64 * g_x))
            .minus(&v_n.times(1f64 * g_y));

        for x in 0..self.x_resolution {
            for y in 0..self.y_resolution {
                let p = p_1_m
                    .plus(&q_x.times((x as f64) + 1f64))
                    .plus(&q_y.times((y as f64) + 1f64));
                let ray = Ray {
                    origin: self.position,
                    direction: p.normalize(),
                };
                view_rays.push(ViewRay {
                    x: x,
                    y: y,
                    ray: ray,
                });
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
