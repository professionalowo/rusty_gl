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
