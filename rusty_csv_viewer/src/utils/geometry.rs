use serde::{Deserialize, Serialize};

pub use self::alignment::{AlignInfo, AlignPreset, AlignShape, HAlignPreset, VAlignPreset};

pub mod alignment {
    pub trait AlignShape<With = Self> {
        type Output;

        fn align(&self, other: &With, lhs: AlignInfo, rhs: AlignInfo) -> Self::Output;
    }

    /// Alignment enums for horizontal and vertical alignment
    pub enum HAlignPreset {
        LEFT,
        CENTER,
        RIGHT,
        DEFAULT
    }

    pub enum VAlignPreset {
        TOP,
        CENTER,
        BOTTOM,
        DEFAULT
    }

    pub struct AlignPreset(HAlignPreset, VAlignPreset);

    /// Alignment is represented as a normalized 2D space
    /// Both the x and y-axis are inclusively within (0.0, 1.0)
    #[derive(PartialEq, Debug)]
    pub struct AlignInfo {
        pub x: f32,
        pub y: f32,
    }

    impl AlignInfo {
        pub fn new(x: f32, y: f32) -> Self {
            if (x >= 0.0 && x <= 1.0) && (y >= 0.0 && y <= 1.0) {
                return AlignInfo { x, y };
            }

            panic!("x and y must be between 0 and 1 inclusive");
        }
    }

    impl From<AlignPreset> for AlignInfo {
        fn from(f: AlignPreset) -> Self {
            let x: f32 = match f.0 {
                HAlignPreset::LEFT | HAlignPreset::DEFAULT => 0.0,
                HAlignPreset::CENTER => 0.5,
                HAlignPreset::RIGHT => 1.0
            };

            let y: f32 = match f.1 {
                VAlignPreset::TOP | VAlignPreset::DEFAULT => 0.0,
                VAlignPreset::CENTER => 0.5,
                VAlignPreset::BOTTOM => 1.0
            };

            AlignInfo::new(x, y)
        }
    }
}

pub trait TranslateShape<Rhs=Point<i32>> {
    type Output;

    fn translate(&mut self, rhs: &Rhs) -> &Self::Output;
}

#[derive(Debug,Default,Deserialize,Serialize,PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug,Default,PartialEq)]
pub struct Rect<T>
{
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T
}

impl TranslateShape for Point<i32> {
    type Output = Point<i32>;

    fn translate(&mut self, rhs: &Point<i32>) -> &Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}

impl TranslateShape for Rect<i32> {
    type Output = Rect<i32>;

    fn translate(&mut self, rhs: &Point<i32>) -> &Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}

impl AlignShape for Rect<i32> {
    type Output = Rect<i32>;

    fn align(&self, other: &Self, lhs: AlignInfo, rhs: AlignInfo) -> Self::Output {
        let mut work_rect: Self::Output = Rect{ x: 0, y: 0, .. *self };
        // Calculate the alignment origin for the lhs
        let src_align_org: Point<i32> = Point { x: (self.width as f32 * lhs.x) as i32, y: (self.height as f32 * lhs.y) as i32};
        // Calculate the alignment origin for the rhs
        let dst_align_org: Point<i32> = Point { x: (other.width as f32 * rhs.x) as i32, y: (other.height as f32 * rhs.y) as i32};
        // dst org from top left (0,0) = (rhs.x * other.width, rhs.y * other.height)
        // src org from top left (0,0) = (lhs.x * self.width,  lhs.y * self.height)

        // src top_left from align on src org + dst org =
        //      (other.x + (rhs.x * other.width), other.y + (rhs.y * other.height)) -
        //      (lhs.x * self.width, lhs.y * self.height)

        // bound the result between 0 and a large number since display coords start at 0,0
        work_rect.x = std::cmp::max(0, (other.x + dst_align_org.x) - src_align_org.x);
        work_rect.y = std::cmp::max(0, (other.y + dst_align_org.y) - src_align_org.y);

        work_rect
    }
}