#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Channels(pub i32);

impl From<i32> for Channels {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<Channels> for i32 {
    fn from(Channels(value): Channels) -> Self {
        value
    }
}
