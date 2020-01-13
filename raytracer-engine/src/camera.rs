use crate::geometry::Point3;
use crate::geometry::Ray;
use crate::geometry::Vector3;

pub struct Camera {
    pub position: Point3,
    pub direction: Vector3,
    pub up_direction: Vector3,
    pub field_of_view: f64,
    pub x_resolution: u16,
    pub y_resolution: u16,
}

pub struct ViewRay {
    pub x: u16,
    pub y: u16,
    pub ray: Ray,
}

impl Camera {
    fn get_origin_and_sizes(&self) -> (Vector3, Vector3, Vector3) {
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

        (p_1_m, q_x, q_y)
    }

    pub fn get_ray(&self, x: u16, y: u16) -> Ray {
        let (p_1_m, q_x, q_y) = self.get_origin_and_sizes();

        let p = p_1_m
            .plus(&q_x.times((x as f64) + 1f64))
            .plus(&q_y.times((y as f64) + 1f64));

        Ray {
            origin: self.position,
            direction: p.normalize(),
        }
    }

    pub fn generate_viewport(&self) -> Vec<ViewRay> {
        let mut view_rays: Vec<ViewRay> = vec![];

        let (p_1_m, q_x, q_y) = self.get_origin_and_sizes();

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
