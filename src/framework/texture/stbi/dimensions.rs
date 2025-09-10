use super::format::Channels;

#[derive(Debug, Default)]
pub struct Dimensions {
    pub width: i32,
    pub height: i32,
}

impl Dimensions {
    pub const fn area(&self) -> i32 {
        self.width * self.height
    }

    pub const fn volume_with_channels(&self, Channels(channels): &Channels) -> i32 {
        *channels * self.area()
    }
}

impl From<(i32, i32)> for Dimensions {
    fn from((width, height): (i32, i32)) -> Self {
        Self { width, height }
    }
}

impl Into<(i32, i32)> for Dimensions {
    fn into(self) -> (i32, i32) {
        (self.width, self.height)
    }
}


impl From<[i32; 2]> for Dimensions {
    fn from([width, height]: [i32; 2]) -> Self {
        Self { width, height }
    }
}

impl Into<[i32; 2]> for Dimensions {
    fn into(self) -> [i32; 2] {
        [self.width, self.height]
    }
}
