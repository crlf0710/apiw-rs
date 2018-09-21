use winapi;

pub mod device_context;
pub mod draw;
pub mod object;

fn clamp_isize_to_i32(v: isize) -> i32 {
    use std::i32::{MAX, MIN};
    if v < MIN as _ {
        MIN
    } else if v > MAX as _ {
        MAX
    } else {
        v as _
    }
}

fn clamp_usize_to_positive_isize(v: usize) -> isize {
    use std::isize::{MAX};
    if v > MAX as _ {
        MAX
    } else {
        v as _
    }
}

fn clamp_usize_to_positive_i32(v: usize) -> i32 {
    use std::i32::{MAX};
    if v < 0 {
        0
    } else if v > MAX as _ {
        MAX
    } else {
        v as _
    }
}

#[derive(Copy, Clone, Into)]
pub struct Point(winapi::shared::windef::POINT);

impl Point {
    pub const ORIGIN: Point = Point(winapi::shared::windef::POINT { x: 0, y: 0 });

    pub fn new(x: isize, y: isize) -> Self {
        Point(winapi::shared::windef::POINT {
            x: clamp_isize_to_i32(x),
            y: clamp_isize_to_i32(y),
        })
    }

    pub fn x(&self) -> isize {
        self.0.x as _
    }

    pub fn y(&self) -> isize {
        self.0.y as _
    }
}

#[derive(Copy, Clone, Into)]
pub struct Size(winapi::shared::windef::SIZE);

impl Size {
    pub fn new(cx: usize, cy: usize) -> Self {
        Size(winapi::shared::windef::SIZE {
            cx: clamp_usize_to_positive_i32(cx),
            cy: clamp_usize_to_positive_i32(cy),
        })
    }

    pub fn cx(&self) -> usize {
        self.0.cx as _
    }

    pub fn cy(&self) -> usize {
        self.0.cy as _
    }
}


#[derive(Copy, Clone)]
pub struct Rect {
    pos: Point,
    size: Size,
}

impl Rect {
    pub fn new(pos: Point, size: Size) -> Self {
        Rect { pos, size }
    }

    pub fn left_top(&self) -> Point {
        self.pos
    }

    pub fn left_bottom(&self) -> Point {
        Point::new(self.pos.x(), self.pos.y().saturating_add(clamp_usize_to_positive_isize(self.size.cy())))
    }
    pub fn right_top(&self) -> Point {
        Point::new(self.pos.x().saturating_add(clamp_usize_to_positive_isize(self.size.cx())),
            self.pos.y())
    }
    pub fn right_bottom(&self) -> Point {
        Point::new(self.pos.x().saturating_add(clamp_usize_to_positive_isize(self.size.cx())), 
            self.pos.y().saturating_add(clamp_usize_to_positive_isize(self.size.cy())))
    }

    pub fn deflate(&self, distance: usize) -> Self {
        Rect {
            pos: Point::new(
                self.pos.x().saturating_sub(clamp_usize_to_positive_isize(distance)),
                self.pos.y().saturating_sub(clamp_usize_to_positive_isize(distance))),
            size: Size::new(
                self.size.cx().saturating_add(distance.saturating_add(distance)),
                self.size.cy().saturating_add(distance.saturating_add(distance))),
        }
    }
}

#[derive(Copy, Clone, Into)]
pub struct RGBColor(winapi::shared::windef::COLORREF);

macro_rules! winapi_rgb_value {
    ($r: expr, $g: expr, $b: expr) => {
        $r as ::winapi::shared::windef::COLORREF 
            | (($g as ::winapi::shared::windef::COLORREF) << 8)
            | (($b as ::winapi::shared::windef::COLORREF) << 16)    
    };
}

impl RGBColor {
    pub const BLACK: RGBColor = RGBColor(winapi_rgb_value!(0, 0, 0));
    pub const WHITE: RGBColor = RGBColor(winapi_rgb_value!(255, 255, 255));
    pub const GRAY: RGBColor = RGBColor(winapi_rgb_value!(128, 128, 128));
    pub const RED: RGBColor = RGBColor(winapi_rgb_value!(255, 0, 0));
    pub const GREEN: RGBColor = RGBColor(winapi_rgb_value!(0, 255, 0));
    pub const BLUE: RGBColor = RGBColor(winapi_rgb_value!(0, 0, 255));
}