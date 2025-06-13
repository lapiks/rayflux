use crate::{common::{color::Color, Camera, Scene}, raytracer::{cpu::intersections::{intersect_scene, IntersectionInfos, StandardHit}, RaytracerImpl, RaytracerOutput}};

pub mod canvas;
pub mod ray;
pub mod shapes;
pub mod intersections;

pub use canvas::*;
use glam::{DVec3, UVec2};
pub use ray::*;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

pub struct CpuRaytracer {
    canvas: Canvas,
}

impl RaytracerImpl for CpuRaytracer {
    fn output(&self) -> RaytracerOutput {
        RaytracerOutput::Image
    }
}

impl CpuRaytracer {
    pub fn new(size: UVec2) -> Self {
        Self {
            canvas: Canvas::new(size.x, size.y),
        }
    }

    /// Render the content of the scene
    pub fn render(&mut self, scene: &Scene) {
        let canvas_size = self.canvas.size();

        self.canvas
            .pixels_mut()
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, color)| {
                let y = i / canvas_size.x as usize;
                let x = i - y * canvas_size.x as usize;

                let ray = Self::ray_for_pixel(
                    scene.camera(), 
                    x as f32, 
                    y as f32, 
                    canvas_size
                );
                *color += Self::raytrace(&ray, scene).unwrap_or(scene.camera().background())
            });
    }

    pub fn canvas(&self) -> &Canvas {
        &self.canvas
    }

    /// Generate a ray given a pixel position
    fn ray_for_pixel(camera: &Camera, x: f32, y: f32, image_size: UVec2) -> Ray {
        let eye = camera.position();
        let forward = (camera.target() - eye).normalize();
        let right = forward.cross(camera.up()).normalize();
        let up = right.cross(forward);

        let fov = camera.field_of_view();
        let aspect = camera.aspect_ratio();

        // Dimensions du plan image à z = -1 (plan de projection)
        let half_height = (fov / 2.0).tan();
        let half_width = aspect * half_height;

        let pixel_width = (half_width * 2.0) / image_size.x as f32;
        let pixel_height = (half_height * 2.0) / image_size.y as f32;

        // Coordonnées en espace NDC [-1, 1]
        let x_ndc = (x + 0.5) * pixel_width - half_width;
        let y_ndc = half_height - (y + 0.5) * pixel_height;

        // Direction du rayon dans l'espace monde
        let direction =
            (forward + right * x_ndc + up * y_ndc).normalize();

        Ray::new(eye.as_dvec3(), direction.as_dvec3())
    }

    /// Trace a ray through the scene and return the resulting color or None if no hit happened
    fn raytrace(&ray: &Ray, scene: &Scene) -> Option<Color> {
        let intersections = intersect_scene(&ray, &scene);
        match intersections.hit_index(StandardHit {}) {
            Some(index) => {
                let infos = IntersectionInfos::new(&intersections, index, &ray);
                Some(Color::RED)
            },
            None => None
        }
    }
}