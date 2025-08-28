use super::Vec3;

impl From<assimp::Vector3D> for Vec3<f32> {
    fn from(v: assimp::Vector3D) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}
