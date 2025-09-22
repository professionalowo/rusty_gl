use crate::channels::Channels;

#[derive(Debug, Default, PartialEq)]
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

impl From<Dimensions> for (i32, i32) {
    fn from(Dimensions { width, height }: Dimensions) -> Self {
        (width, height)
    }
}

impl From<[i32; 2]> for Dimensions {
    fn from([width, height]: [i32; 2]) -> Self {
        Self { width, height }
    }
}

impl From<Dimensions> for [i32; 2] {
    fn from(Dimensions { width, height }: Dimensions) -> Self {
        [width, height]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DIM: Dimensions = Dimensions {
        width: 2,
        height: 3,
    };

    #[test]
    fn test_dimensions_area() {
        assert_eq!(6, DIM.area())
    }

    #[test]
    fn test_dimensions_volume_with_channels() {
        let ch = Channels(4);
        assert_eq!(24, DIM.volume_with_channels(&ch));
    }

    #[test]
    fn test_dimensions_from_tuple() {
        const TUP: (i32, i32) = (2, 3);
        assert_eq!(DIM, Dimensions::from(TUP));
        assert_eq!(DIM, TUP.into());
    }

    #[test]
    fn test_dimensions_from_array() {
        const ARR: [i32; 2] = [2, 3];
        assert_eq!(DIM, Dimensions::from(ARR));
        assert_eq!(DIM, ARR.into())
    }
}
