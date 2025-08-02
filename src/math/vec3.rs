#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vec3<T: Copy> {
    x: T,
    y: T,
    z: T,
}

impl<T: Copy> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub const fn x(&self) -> T {
        self.x
    }

    pub const fn y(&self) -> T {
        self.y
    }

    pub const fn z(&self) -> T {
        self.z
    }

    pub const fn data(&self) -> [T; 3] {
        [self.x, self.y, self.z]
    }

    pub const fn size() -> usize {
        3
    }
}

impl<T: Copy + std::ops::Add<Output = T>> std::ops::Add for Vec3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Copy + std::ops::Sub<Output = T>> std::ops::Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
