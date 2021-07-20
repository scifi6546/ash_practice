use sukakpak::nalgebra::{Matrix4, Vector3};
#[derive(Debug, Clone, PartialEq)]
pub struct Transform {
    position: Vector3<f32>,
    pitch: f32,
    yaw: f32,
    roll: f32,
}
impl Transform {
    pub fn mat(&self) -> Matrix4<f32> {
        let rotation = Matrix4::from_euler_angles(self.roll, self.pitch, self.yaw);
        let translation = Matrix4::new_translation(&(-1.0 * self.position));
        rotation * translation
    }
}
impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vector3::new(0.0, -0.5, -10.0),
            pitch: 30.0,
            yaw: 0.0,
            roll: 0.0,
        }
    }
}
pub struct Camera {
    position: Vector3<f32>,
    pitch: f32,
    yaw: f32,
    roll: f32,
    fov: f32,
    aspect_ratio: f32,
    near_clip: f32,
    far_clip: f32,
}
impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vector3::new(0.0, 0.0, 0.0),
            pitch: 0.0,
            yaw: 0.0,
            roll: 0.0,
            fov: 3.14,
            aspect_ratio: 1.0,
            near_clip: 0.1,
            far_clip: 100.0,
        }
    }
}
impl Camera {
    pub fn to_vec(&self, transform: &Transform) -> Vec<u8> {
        let perspective_mat =
            Matrix4::new_perspective(self.fov, self.aspect_ratio, self.near_clip, self.far_clip);
        let rotation = Matrix4::from_euler_angles(self.roll, self.pitch, self.yaw);
        let translation = Matrix4::new_translation(&(-1.0 * self.position));
        (perspective_mat * rotation * translation * transform.mat())
            .as_slice()
            .iter()
            .map(|f| f.to_ne_bytes())
            .flatten()
            .collect()
    }
}
