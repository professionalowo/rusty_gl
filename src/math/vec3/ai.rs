use super::Vec3;
use assimp::Vector3D;

impl From<Vector3D> for Vec3<f32> {
    fn from(v: Vector3D) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}
