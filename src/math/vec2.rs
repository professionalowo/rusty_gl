#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vec2<T: Copy> {
    x: T,
    y: T,
}

impl<T: Copy> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub const fn x(&self) -> T {
        self.x
    }

    pub const fn y(&self) -> T {
        self.y
    }

    pub const fn data(&self) -> [T; 2] {
        [self.x, self.y]
    }

    pub const fn size() -> usize {
        2
    }
}

impl<T: Copy + std::ops::Add<Output = T>> std::ops::Add for Vec2<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Copy + std::ops::Sub<Output = T>> std::ops::Sub for Vec2<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
