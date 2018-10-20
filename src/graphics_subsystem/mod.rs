use winapi;
use winapi::shared::minwindef::DWORD;

use utils::clamp_i32_to_positive_i32;
use utils::clamp_isize_to_i32;
use utils::clamp_usize_to_positive_i32;
use utils::clamp_usize_to_positive_isize;

pub mod device_context;
pub mod draw;
pub mod object;

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

    pub fn offset(&self, off_x: isize, off_y: isize) -> Self {
        Point::new(
            (self.0.x as isize).saturating_add(off_x),
            (self.0.y as isize).saturating_add(off_y),
        )
    }
}

use std::fmt::{self, Debug, Formatter};

impl PartialEq<Self> for Point {
    fn eq(&self, rhs: &Self) -> bool {
        self.0.x == rhs.0.x && self.0.y == rhs.0.y
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "Point(x={:?}, y={:?})", self.x(), self.y())
    }
}

#[derive(Copy, Clone, Into)]
pub struct Size(pub(crate) winapi::shared::windef::SIZE);

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

impl Debug for Size {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "Size(w={:?}, h={:?})", self.cx(), self.cy())
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

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn top_left(&self) -> Point {
        self.pos
    }

    pub fn bottom_left(&self) -> Point {
        Point::new(
            self.pos.x(),
            self.pos
                .y()
                .saturating_add(clamp_usize_to_positive_isize(self.size.cy())),
        )
    }
    pub fn top_right(&self) -> Point {
        Point::new(
            self.pos
                .x()
                .saturating_add(clamp_usize_to_positive_isize(self.size.cx())),
            self.pos.y(),
        )
    }
    pub fn bottom_right(&self) -> Point {
        Point::new(
            self.pos
                .x()
                .saturating_add(clamp_usize_to_positive_isize(self.size.cx())),
            self.pos
                .y()
                .saturating_add(clamp_usize_to_positive_isize(self.size.cy())),
        )
    }

    pub fn deflate(&self, distance: usize) -> Self {
        Rect {
            pos: Point::new(
                self.pos
                    .x()
                    .saturating_sub(clamp_usize_to_positive_isize(distance)),
                self.pos
                    .y()
                    .saturating_sub(clamp_usize_to_positive_isize(distance)),
            ),
            size: Size::new(
                self.size
                    .cx()
                    .saturating_add(distance.saturating_add(distance)),
                self.size
                    .cy()
                    .saturating_add(distance.saturating_add(distance)),
            ),
        }
    }

    pub fn contains(&self, pt: Point) -> bool {
        let bottom_right = self.bottom_right();
        if pt.0.x >= self.pos.0.x && pt.0.x < bottom_right.0.x {
            if pt.0.y >= self.pos.0.y && pt.0.y < bottom_right.0.y {
                return true;
            }
        }
        false
    }
}

use winapi::shared::windef::RECT;
impl From<RECT> for Rect {
    fn from(v: RECT) -> Self {
        Self {
            pos: Point::new(v.left as isize, v.top as isize),
            size: Size::new(
                clamp_i32_to_positive_i32(v.right - v.left) as usize,
                clamp_i32_to_positive_i32(v.bottom - v.top) as usize,
            ),
        }
    }
}

impl From<Rect> for RECT {
    fn from(v: Rect) -> Self {
        let left_top = v.top_left();
        let right_bottom = v.bottom_right();
        Self {
            left: left_top.0.x,
            right: right_bottom.0.x,
            top: left_top.0.y,
            bottom: right_bottom.0.y,
        }
    }
}

#[derive(Copy, Clone, Into, PartialEq)]
pub struct RGBColor(winapi::shared::windef::COLORREF);

macro_rules! winapi_rgb_value {
    ($r:expr, $g:expr, $b:expr) => {
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
    pub const MAGENTA: RGBColor = RGBColor(winapi_rgb_value!(255, 0, 255));

    pub fn new(red: u8, green: u8, blue: u8) -> RGBColor {
        RGBColor(winapi_rgb_value!(red, green, blue))
    }
}

#[derive(Copy, Clone, Into)]
pub struct BinaryROP(winapi::ctypes::c_int);

#[allow(non_upper_case_globals)]
impl BinaryROP {
    const INTERNAL_OP_0: (u8, u16) = (0x0, 0);
    const INTERNAL_OP_P: (u8, u16) = (0x1, 12);
    const INTERNAL_OP_DPna: (u8, u16) = (0x2, 2);
    const INTERNAL_OP_DPa: (u8, u16) = (0x3, 8);
    const INTERNAL_OP_PDna: (u8, u16) = (0x4, 4);
    const INTERNAL_OP_DPno: (u8, u16) = (0x5, 11);
    const INTERNAL_OP_DPo: (u8, u16) = (0x6, 14);
    const INTERNAL_OP_PDno: (u8, u16) = (0x7, 13);
    const INTERNAL_OP_D: (u8, u16) = (0x8, 10);
    const INTERNAL_OP_Dn: (u8, u16) = (0x9, 5);
    const INTERNAL_OP_Pn: (u8, u16) = (0xA, 3);
    const INTERNAL_OP_DPan: (u8, u16) = (0xB, 7);
    const INTERNAL_OP_DPon: (u8, u16) = (0xC, 1);
    const INTERNAL_OP_DPxn: (u8, u16) = (0xD, 9);
    const INTERNAL_OP_1: (u8, u16) = (0xE, 15);
    const INTERNAL_OP_DPx: (u8, u16) = (0xF, 6);

    pub const R2_BLACK: BinaryROP = BinaryROP((Self::INTERNAL_OP_0).1 as i32 + 1);
    pub const R2_COPYPEN: BinaryROP = BinaryROP((Self::INTERNAL_OP_P).1 as i32 + 1);
    pub const R2_MASKNOTPEN: BinaryROP = BinaryROP((Self::INTERNAL_OP_DPna).1 as i32 + 1);
    pub const R2_MASKPEN: BinaryROP = BinaryROP((Self::INTERNAL_OP_DPa).1 as i32 + 1);
    pub const R2_MASKPENNOT: BinaryROP = BinaryROP((Self::INTERNAL_OP_PDna).1 as i32 + 1);
    pub const R2_MERGENOTPEN: BinaryROP = BinaryROP((Self::INTERNAL_OP_DPno).1 as i32 + 1);
    pub const R2_MERGEPEN: BinaryROP = BinaryROP((Self::INTERNAL_OP_DPo).1 as i32 + 1);
    pub const R2_MERGEPENNOT: BinaryROP = BinaryROP((Self::INTERNAL_OP_PDno).1 as i32 + 1);
    pub const R2_NOP: BinaryROP = BinaryROP((Self::INTERNAL_OP_D).1 as i32 + 1);
    pub const R2_NOT: BinaryROP = BinaryROP((Self::INTERNAL_OP_Dn).1 as i32 + 1);
    pub const R2_NOTCOPYPEN: BinaryROP = BinaryROP((Self::INTERNAL_OP_Pn).1 as i32 + 1);
    pub const R2_NOTMASKPEN: BinaryROP = BinaryROP((Self::INTERNAL_OP_DPan).1 as i32 + 1);
    pub const R2_NOTMERGEPEN: BinaryROP = BinaryROP((Self::INTERNAL_OP_DPon).1 as i32 + 1);
    pub const R2_NOTXORPEN: BinaryROP = BinaryROP((Self::INTERNAL_OP_DPxn).1 as i32 + 1);
    pub const R2_WHITE: BinaryROP = BinaryROP((Self::INTERNAL_OP_1).1 as i32 + 1);
    pub const R2_XORPEN: BinaryROP = BinaryROP((Self::INTERNAL_OP_DPx).1 as i32 + 1);
}

#[derive(Copy, Clone, Into)]
pub struct TenaryROP(winapi::shared::minwindef::DWORD);

#[allow(non_upper_case_globals)]
impl TenaryROP {
    const INTERNAL_OP_0: (u8, u16) = (0x00, 0x0042);
    const INTERNAL_OP_DPSoon: (u8, u16) = (0x01, 0x0289);
    const INTERNAL_OP_DPSona: (u8, u16) = (0x02, 0x0C89);
    const INTERNAL_OP_PSon: (u8, u16) = (0x03, 0x00AA);
    const INTERNAL_OP_SDPona: (u8, u16) = (0x04, 0x0C88);
    const INTERNAL_OP_DPon: (u8, u16) = (0x05, 0x00A9);
    const INTERNAL_OP_PDSxnon: (u8, u16) = (0x06, 0x0865);
    const INTERNAL_OP_PDSaon: (u8, u16) = (0x07, 0x02C5);
    const INTERNAL_OP_SDPnaa: (u8, u16) = (0x08, 0x0F08);
    const INTERNAL_OP_PDSxon: (u8, u16) = (0x09, 0x0245);
    const INTERNAL_OP_DPna: (u8, u16) = (0x0A, 0x0329);
    const INTERNAL_OP_PSDnaon: (u8, u16) = (0x0B, 0x0B2A);
    const INTERNAL_OP_SPna: (u8, u16) = (0x0C, 0x0324);
    const INTERNAL_OP_PDSnaon: (u8, u16) = (0x0D, 0x0B25);
    const INTERNAL_OP_PDSonon: (u8, u16) = (0x0E, 0x08A5);
    const INTERNAL_OP_Pn: (u8, u16) = (0x0F, 0x0001);
    const INTERNAL_OP_PDSona: (u8, u16) = (0x10, 0x0C85);
    const INTERNAL_OP_DSon: (u8, u16) = (0x11, 0x00A6);
    const INTERNAL_OP_SDPxnon: (u8, u16) = (0x12, 0x0868);
    const INTERNAL_OP_SDPaon: (u8, u16) = (0x13, 0x02C8);
    const INTERNAL_OP_DPSxnon: (u8, u16) = (0x14, 0x0869);
    const INTERNAL_OP_DPSaon: (u8, u16) = (0x15, 0x02C9);
    const INTERNAL_OP_PSDPSanaxx: (u8, u16) = (0x16, 0x5CCA);
    const INTERNAL_OP_SSPxDSxaxn: (u8, u16) = (0x17, 0x1D54);
    const INTERNAL_OP_SPxPDxa: (u8, u16) = (0x18, 0x0D59);
    const INTERNAL_OP_SDPSanaxn: (u8, u16) = (0x19, 0x1CC8);
    const INTERNAL_OP_PDSPaox: (u8, u16) = (0x1A, 0x06C5);
    const INTERNAL_OP_SDPSxaxn: (u8, u16) = (0x1B, 0x0768);
    const INTERNAL_OP_PSDPaox: (u8, u16) = (0x1C, 0x06CA);
    const INTERNAL_OP_DSPDxaxn: (u8, u16) = (0x1D, 0x0766);
    const INTERNAL_OP_PDSox: (u8, u16) = (0x1E, 0x01A5);
    const INTERNAL_OP_PDSoan: (u8, u16) = (0x1F, 0x0385);
    const INTERNAL_OP_DPSnaa: (u8, u16) = (0x20, 0x0F09);
    const INTERNAL_OP_SDPxon: (u8, u16) = (0x21, 0x0248);
    const INTERNAL_OP_DSna: (u8, u16) = (0x22, 0x0326);
    const INTERNAL_OP_SPDnaon: (u8, u16) = (0x23, 0x0B24);
    const INTERNAL_OP_SPxDSxa: (u8, u16) = (0x24, 0x0D55);
    const INTERNAL_OP_PDSPanaxn: (u8, u16) = (0x25, 0x1CC5);
    const INTERNAL_OP_SDPSaox: (u8, u16) = (0x26, 0x06C8);
    const INTERNAL_OP_SDPSxnox: (u8, u16) = (0x27, 0x1868);
    const INTERNAL_OP_DPSxa: (u8, u16) = (0x28, 0x0369);
    const INTERNAL_OP_PSDPSaoxxn: (u8, u16) = (0x29, 0x16CA);
    const INTERNAL_OP_DPSana: (u8, u16) = (0x2A, 0x0CC9);
    const INTERNAL_OP_SSPxPDxaxn: (u8, u16) = (0x2B, 0x1D58);
    const INTERNAL_OP_SPDSoax: (u8, u16) = (0x2C, 0x0784);
    const INTERNAL_OP_PSDnox: (u8, u16) = (0x2D, 0x060A);
    const INTERNAL_OP_PSDPxox: (u8, u16) = (0x2E, 0x064A);
    const INTERNAL_OP_PSDnoan: (u8, u16) = (0x2F, 0x0E2A);
    const INTERNAL_OP_PSna: (u8, u16) = (0x30, 0x032A);
    const INTERNAL_OP_SDPnaon: (u8, u16) = (0x31, 0x0B28);
    const INTERNAL_OP_SDPSoox: (u8, u16) = (0x32, 0x0688);
    const INTERNAL_OP_Sn: (u8, u16) = (0x33, 0x0008);
    const INTERNAL_OP_SPDSaox: (u8, u16) = (0x34, 0x06C4);
    const INTERNAL_OP_SPDSxnox: (u8, u16) = (0x35, 0x1864);
    const INTERNAL_OP_SDPox: (u8, u16) = (0x36, 0x01A8);
    const INTERNAL_OP_SDPoan: (u8, u16) = (0x37, 0x0388);
    const INTERNAL_OP_PSDPoax: (u8, u16) = (0x38, 0x078A);
    const INTERNAL_OP_SPDnox: (u8, u16) = (0x39, 0x0604);
    const INTERNAL_OP_SPDSxox: (u8, u16) = (0x3A, 0x0644);
    const INTERNAL_OP_SPDnoan: (u8, u16) = (0x3B, 0x0E24);
    const INTERNAL_OP_PSx: (u8, u16) = (0x3C, 0x004A);
    const INTERNAL_OP_SPDSonox: (u8, u16) = (0x3D, 0x18A4);
    const INTERNAL_OP_SPDSnaox: (u8, u16) = (0x3E, 0x1B24);
    const INTERNAL_OP_PSan: (u8, u16) = (0x3F, 0x00EA);
    const INTERNAL_OP_PSDnaa: (u8, u16) = (0x40, 0x0F0A);
    const INTERNAL_OP_DPSxon: (u8, u16) = (0x41, 0x0249);
    const INTERNAL_OP_SDxPDxa: (u8, u16) = (0x42, 0x0D5D);
    const INTERNAL_OP_SPDSanaxn: (u8, u16) = (0x43, 0x1CC4);
    const INTERNAL_OP_SDna: (u8, u16) = (0x44, 0x0328);
    const INTERNAL_OP_DPSnaon: (u8, u16) = (0x45, 0x0B29);
    const INTERNAL_OP_DSPDaox: (u8, u16) = (0x46, 0x06C6);
    const INTERNAL_OP_PSDPxaxn: (u8, u16) = (0x47, 0x076A);
    const INTERNAL_OP_SDPxa: (u8, u16) = (0x48, 0x0368);
    const INTERNAL_OP_PDSPDaoxxn: (u8, u16) = (0x49, 0x16C5);
    const INTERNAL_OP_DPSDoax: (u8, u16) = (0x4A, 0x0789);
    const INTERNAL_OP_PDSnox: (u8, u16) = (0x4B, 0x0605);
    const INTERNAL_OP_SDPana: (u8, u16) = (0x4C, 0x0CC8);
    const INTERNAL_OP_SSPxDSxoxn: (u8, u16) = (0x4D, 0x1954);
    const INTERNAL_OP_PDSPxox: (u8, u16) = (0x4E, 0x0645);
    const INTERNAL_OP_PDSnoan: (u8, u16) = (0x4F, 0x0E25);
    const INTERNAL_OP_PDna: (u8, u16) = (0x50, 0x0325);
    const INTERNAL_OP_DSPnaon: (u8, u16) = (0x51, 0x0B26);
    const INTERNAL_OP_DPSDaox: (u8, u16) = (0x52, 0x06C9);
    const INTERNAL_OP_SPDSxaxn: (u8, u16) = (0x53, 0x0764);
    const INTERNAL_OP_DPSonon: (u8, u16) = (0x54, 0x08A9);
    const INTERNAL_OP_Dn: (u8, u16) = (0x55, 0x0009);
    const INTERNAL_OP_DPSox: (u8, u16) = (0x56, 0x01A9);
    const INTERNAL_OP_DPSoan: (u8, u16) = (0x57, 0x0389);
    const INTERNAL_OP_PDSPoax: (u8, u16) = (0x58, 0x0785);
    const INTERNAL_OP_DPSnox: (u8, u16) = (0x59, 0x0609);
    const INTERNAL_OP_DPx: (u8, u16) = (0x5A, 0x0049);
    const INTERNAL_OP_DPSDonox: (u8, u16) = (0x5B, 0x18A9);
    const INTERNAL_OP_DPSDxox: (u8, u16) = (0x5C, 0x0649);
    const INTERNAL_OP_DPSnoan: (u8, u16) = (0x5D, 0x0E29);
    const INTERNAL_OP_DPSDnaox: (u8, u16) = (0x5E, 0x1B29);
    const INTERNAL_OP_DPan: (u8, u16) = (0x5F, 0x00E9);
    const INTERNAL_OP_PDSxa: (u8, u16) = (0x60, 0x0365);
    const INTERNAL_OP_DSPDSaoxxn: (u8, u16) = (0x61, 0x16C6);
    const INTERNAL_OP_DSPDoax: (u8, u16) = (0x62, 0x0786);
    const INTERNAL_OP_SDPnox: (u8, u16) = (0x63, 0x0608);
    const INTERNAL_OP_SDPSoax: (u8, u16) = (0x64, 0x0788);
    const INTERNAL_OP_DSPnox: (u8, u16) = (0x65, 0x0606);
    const INTERNAL_OP_DSx: (u8, u16) = (0x66, 0x0046);
    const INTERNAL_OP_SDPSonox: (u8, u16) = (0x67, 0x18A8);
    const INTERNAL_OP_DSPDSonoxxn: (u8, u16) = (0x68, 0x58A6);
    const INTERNAL_OP_PDSxxn: (u8, u16) = (0x69, 0x0145);
    const INTERNAL_OP_DPSax: (u8, u16) = (0x6A, 0x01E9);
    const INTERNAL_OP_PSDPSoaxxn: (u8, u16) = (0x6B, 0x178A);
    const INTERNAL_OP_SDPax: (u8, u16) = (0x6C, 0x01E8);
    const INTERNAL_OP_PDSPDoaxxn: (u8, u16) = (0x6D, 0x1785);
    const INTERNAL_OP_SDPSnoax: (u8, u16) = (0x6E, 0x1E28);
    const INTERNAL_OP_PDSxnan: (u8, u16) = (0x6F, 0x0C65);
    const INTERNAL_OP_PDSana: (u8, u16) = (0x70, 0x0CC5);
    const INTERNAL_OP_SSDxPDxaxn: (u8, u16) = (0x71, 0x1D5C);
    const INTERNAL_OP_SDPSxox: (u8, u16) = (0x72, 0x0648);
    const INTERNAL_OP_SDPnoan: (u8, u16) = (0x73, 0x0E28);
    const INTERNAL_OP_DSPDxox: (u8, u16) = (0x74, 0x0646);
    const INTERNAL_OP_DSPnoan: (u8, u16) = (0x75, 0x0E26);
    const INTERNAL_OP_SDPSnaox: (u8, u16) = (0x76, 0x1B28);
    const INTERNAL_OP_DSan: (u8, u16) = (0x77, 0x00E6);
    const INTERNAL_OP_PDSax: (u8, u16) = (0x78, 0x01E5);
    const INTERNAL_OP_DSPDSoaxxn: (u8, u16) = (0x79, 0x1786);
    const INTERNAL_OP_DPSDnoax: (u8, u16) = (0x7A, 0x1E29);
    const INTERNAL_OP_SDPxnan: (u8, u16) = (0x7B, 0x0C68);
    const INTERNAL_OP_SPDSnoax: (u8, u16) = (0x7C, 0x1E24);
    const INTERNAL_OP_DPSxnan: (u8, u16) = (0x7D, 0x0C69);
    const INTERNAL_OP_SPxDSxo: (u8, u16) = (0x7E, 0x0955);
    const INTERNAL_OP_DPSaan: (u8, u16) = (0x7F, 0x03C9);
    const INTERNAL_OP_DPSaa: (u8, u16) = (0x80, 0x03E9);
    const INTERNAL_OP_SPxDSxon: (u8, u16) = (0x81, 0x0975);
    const INTERNAL_OP_DPSxna: (u8, u16) = (0x82, 0x0C49);
    const INTERNAL_OP_SPDSnoaxn: (u8, u16) = (0x83, 0x1E04);
    const INTERNAL_OP_SDPxna: (u8, u16) = (0x84, 0x0C48);
    const INTERNAL_OP_PDSPnoaxn: (u8, u16) = (0x85, 0x1E05);
    const INTERNAL_OP_DSPDSoaxx: (u8, u16) = (0x86, 0x17A6);
    const INTERNAL_OP_PDSaxn: (u8, u16) = (0x87, 0x01C5);
    const INTERNAL_OP_DSa: (u8, u16) = (0x88, 0x00C6);
    const INTERNAL_OP_SDPSnaoxn: (u8, u16) = (0x89, 0x1B08);
    const INTERNAL_OP_DSPnoa: (u8, u16) = (0x8A, 0x0E06);
    const INTERNAL_OP_DSPDxoxn: (u8, u16) = (0x8B, 0x0666);
    const INTERNAL_OP_SDPnoa: (u8, u16) = (0x8C, 0x0E08);
    const INTERNAL_OP_SDPSxoxn: (u8, u16) = (0x8D, 0x0668);
    const INTERNAL_OP_SSDxPDxax: (u8, u16) = (0x8E, 0x1D7C);
    const INTERNAL_OP_PDSanan: (u8, u16) = (0x8F, 0x0CE5);
    const INTERNAL_OP_PDSxna: (u8, u16) = (0x90, 0x0C45);
    const INTERNAL_OP_SDPSnoaxn: (u8, u16) = (0x91, 0x1E08);
    const INTERNAL_OP_DPSDPoaxx: (u8, u16) = (0x92, 0x17A9);
    const INTERNAL_OP_SPDaxn: (u8, u16) = (0x93, 0x01C4);
    const INTERNAL_OP_PSDPSoaxx: (u8, u16) = (0x94, 0x17AA);
    const INTERNAL_OP_DPSaxn: (u8, u16) = (0x95, 0x01C9);
    const INTERNAL_OP_DPSxx: (u8, u16) = (0x96, 0x0169);
    const INTERNAL_OP_PSDPSonoxx: (u8, u16) = (0x97, 0x588A);
    const INTERNAL_OP_SDPSonoxn: (u8, u16) = (0x98, 0x1888);
    const INTERNAL_OP_DSxn: (u8, u16) = (0x99, 0x0066);
    const INTERNAL_OP_DPSnax: (u8, u16) = (0x9A, 0x0709);
    const INTERNAL_OP_SDPSoaxn: (u8, u16) = (0x9B, 0x07A8);
    const INTERNAL_OP_SPDnax: (u8, u16) = (0x9C, 0x0704);
    const INTERNAL_OP_DSPDoaxn: (u8, u16) = (0x9D, 0x07A6);
    const INTERNAL_OP_DSPDSaoxx: (u8, u16) = (0x9E, 0x16E6);
    const INTERNAL_OP_PDSxan: (u8, u16) = (0x9F, 0x0345);
    const INTERNAL_OP_DPa: (u8, u16) = (0xA0, 0x00C9);
    const INTERNAL_OP_PDSPnaoxn: (u8, u16) = (0xA1, 0x1B05);
    const INTERNAL_OP_DPSnoa: (u8, u16) = (0xA2, 0x0E09);
    const INTERNAL_OP_DPSDxoxn: (u8, u16) = (0xA3, 0x0669);
    const INTERNAL_OP_PDSPonoxn: (u8, u16) = (0xA4, 0x1885);
    const INTERNAL_OP_PDxn: (u8, u16) = (0xA5, 0x0065);
    const INTERNAL_OP_DSPnax: (u8, u16) = (0xA6, 0x0706);
    const INTERNAL_OP_PDSPoaxn: (u8, u16) = (0xA7, 0x07A5);
    const INTERNAL_OP_DPSoa: (u8, u16) = (0xA8, 0x03A9);
    const INTERNAL_OP_DPSoxn: (u8, u16) = (0xA9, 0x0189);
    const INTERNAL_OP_D: (u8, u16) = (0xAA, 0x0029);
    const INTERNAL_OP_DPSono: (u8, u16) = (0xAB, 0x0889);
    const INTERNAL_OP_SPDSxax: (u8, u16) = (0xAC, 0x0744);
    const INTERNAL_OP_DPSDaoxn: (u8, u16) = (0xAD, 0x06E9);
    const INTERNAL_OP_DSPnao: (u8, u16) = (0xAE, 0x0B06);
    const INTERNAL_OP_DPno: (u8, u16) = (0xAF, 0x0229);
    const INTERNAL_OP_PDSnoa: (u8, u16) = (0xB0, 0x0E05);
    const INTERNAL_OP_PDSPxoxn: (u8, u16) = (0xB1, 0x0665);
    const INTERNAL_OP_SSPxDSxox: (u8, u16) = (0xB2, 0x1974);
    const INTERNAL_OP_SDPanan: (u8, u16) = (0xB3, 0x0CE8);
    const INTERNAL_OP_PSDnax: (u8, u16) = (0xB4, 0x070A);
    const INTERNAL_OP_DPSDoaxn: (u8, u16) = (0xB5, 0x07A9);
    const INTERNAL_OP_DPSDPaoxx: (u8, u16) = (0xB6, 0x16E9);
    const INTERNAL_OP_SDPxan: (u8, u16) = (0xB7, 0x0348);
    const INTERNAL_OP_PSDPxax: (u8, u16) = (0xB8, 0x074A);
    const INTERNAL_OP_DSPDaoxn: (u8, u16) = (0xB9, 0x06E6);
    const INTERNAL_OP_DPSnao: (u8, u16) = (0xBA, 0x0B09);
    const INTERNAL_OP_DSno: (u8, u16) = (0xBB, 0x0226);
    const INTERNAL_OP_SPDSanax: (u8, u16) = (0xBC, 0x1CE4);
    const INTERNAL_OP_SDxPDxan: (u8, u16) = (0xBD, 0x0D7D);
    const INTERNAL_OP_DPSxo: (u8, u16) = (0xBE, 0x0269);
    const INTERNAL_OP_DPSano: (u8, u16) = (0xBF, 0x08C9);
    const INTERNAL_OP_PSa: (u8, u16) = (0xC0, 0x00CA);
    const INTERNAL_OP_SPDSnaoxn: (u8, u16) = (0xC1, 0x1B04);
    const INTERNAL_OP_SPDSonoxn: (u8, u16) = (0xC2, 0x1884);
    const INTERNAL_OP_PSxn: (u8, u16) = (0xC3, 0x006A);
    const INTERNAL_OP_SPDnoa: (u8, u16) = (0xC4, 0x0E04);
    const INTERNAL_OP_SPDSxoxn: (u8, u16) = (0xC5, 0x0664);
    const INTERNAL_OP_SDPnax: (u8, u16) = (0xC6, 0x0708);
    const INTERNAL_OP_PSDPoaxn: (u8, u16) = (0xC7, 0x07AA);
    const INTERNAL_OP_SDPoa: (u8, u16) = (0xC8, 0x03A8);
    const INTERNAL_OP_SPDoxn: (u8, u16) = (0xC9, 0x0184);
    const INTERNAL_OP_DPSDxax: (u8, u16) = (0xCA, 0x0749);
    const INTERNAL_OP_SPDSaoxn: (u8, u16) = (0xCB, 0x06E4);
    const INTERNAL_OP_S: (u8, u16) = (0xCC, 0x0020);
    const INTERNAL_OP_SDPono: (u8, u16) = (0xCD, 0x0888);
    const INTERNAL_OP_SDPnao: (u8, u16) = (0xCE, 0x0B08);
    const INTERNAL_OP_SPno: (u8, u16) = (0xCF, 0x0224);
    const INTERNAL_OP_PSDnoa: (u8, u16) = (0xD0, 0x0E0A);
    const INTERNAL_OP_PSDPxoxn: (u8, u16) = (0xD1, 0x066A);
    const INTERNAL_OP_PDSnax: (u8, u16) = (0xD2, 0x0705);
    const INTERNAL_OP_SPDSoaxn: (u8, u16) = (0xD3, 0x07A4);
    const INTERNAL_OP_SSPxPDxax: (u8, u16) = (0xD4, 0x1D78);
    const INTERNAL_OP_DPSanan: (u8, u16) = (0xD5, 0x0CE9);
    const INTERNAL_OP_PSDPSaoxx: (u8, u16) = (0xD6, 0x16EA);
    const INTERNAL_OP_DPSxan: (u8, u16) = (0xD7, 0x0349);
    const INTERNAL_OP_PDSPxax: (u8, u16) = (0xD8, 0x0745);
    const INTERNAL_OP_SDPSaoxn: (u8, u16) = (0xD9, 0x06E8);
    const INTERNAL_OP_DPSDanax: (u8, u16) = (0xDA, 0x1CE9);
    const INTERNAL_OP_SPxDSxan: (u8, u16) = (0xDB, 0x0D75);
    const INTERNAL_OP_SPDnao: (u8, u16) = (0xDC, 0x0B04);
    const INTERNAL_OP_SDno: (u8, u16) = (0xDD, 0x0228);
    const INTERNAL_OP_SDPxo: (u8, u16) = (0xDE, 0x0268);
    const INTERNAL_OP_SDPano: (u8, u16) = (0xDF, 0x08C8);
    const INTERNAL_OP_PDSoa: (u8, u16) = (0xE0, 0x03A5);
    const INTERNAL_OP_PDSoxn: (u8, u16) = (0xE1, 0x0185);
    const INTERNAL_OP_DSPDxax: (u8, u16) = (0xE2, 0x0746);
    const INTERNAL_OP_PSDPaoxn: (u8, u16) = (0xE3, 0x06EA);
    const INTERNAL_OP_SDPSxax: (u8, u16) = (0xE4, 0x0748);
    const INTERNAL_OP_PDSPaoxn: (u8, u16) = (0xE5, 0x06E5);
    const INTERNAL_OP_SDPSanax: (u8, u16) = (0xE6, 0x1CE8);
    const INTERNAL_OP_SPxPDxan: (u8, u16) = (0xE7, 0x0D79);
    const INTERNAL_OP_SSPxDSxax: (u8, u16) = (0xE8, 0x1D74);
    const INTERNAL_OP_DSPDSanaxxn: (u8, u16) = (0xE9, 0x5CE6);
    const INTERNAL_OP_DPSao: (u8, u16) = (0xEA, 0x02E9);
    const INTERNAL_OP_DPSxno: (u8, u16) = (0xEB, 0x0849);
    const INTERNAL_OP_SDPao: (u8, u16) = (0xEC, 0x02E8);
    const INTERNAL_OP_SDPxno: (u8, u16) = (0xED, 0x0848);
    const INTERNAL_OP_DSo: (u8, u16) = (0xEE, 0x0086);
    const INTERNAL_OP_SDPnoo: (u8, u16) = (0xEF, 0x0A08);
    const INTERNAL_OP_P: (u8, u16) = (0xF0, 0x0021);
    const INTERNAL_OP_PDSono: (u8, u16) = (0xF1, 0x0885);
    const INTERNAL_OP_PDSnao: (u8, u16) = (0xF2, 0x0B05);
    const INTERNAL_OP_PSno: (u8, u16) = (0xF3, 0x022A);
    const INTERNAL_OP_PSDnao: (u8, u16) = (0xF4, 0x0B0A);
    const INTERNAL_OP_PDno: (u8, u16) = (0xF5, 0x0225);
    const INTERNAL_OP_PDSxo: (u8, u16) = (0xF6, 0x0265);
    const INTERNAL_OP_PDSano: (u8, u16) = (0xF7, 0x08C5);
    const INTERNAL_OP_PDSao: (u8, u16) = (0xF8, 0x02E5);
    const INTERNAL_OP_PDSxno: (u8, u16) = (0xF9, 0x0845);
    const INTERNAL_OP_DPo: (u8, u16) = (0xFA, 0x0089);
    const INTERNAL_OP_DPSnoo: (u8, u16) = (0xFB, 0x0A09);
    const INTERNAL_OP_PSo: (u8, u16) = (0xFC, 0x008A);
    const INTERNAL_OP_PSDnoo: (u8, u16) = (0xFD, 0x0A0A);
    const INTERNAL_OP_DPSoo: (u8, u16) = (0xFE, 0x02A9);
    const INTERNAL_OP_1: (u8, u16) = (0xFF, 0x0062);

    const INTERNAL_VALUE_NOMIRRORBITMAP: u32 = 0x80000000;

    pub const BLACKNESS: TenaryROP =
        TenaryROP((Self::INTERNAL_OP_0.0 as u32) << 16 | Self::INTERNAL_OP_0.1 as u32);
    pub const NOTSRCERASE: TenaryROP =
        TenaryROP((Self::INTERNAL_OP_DSon.0 as u32) << 16 | Self::INTERNAL_OP_DSon.1 as u32);
    pub const NOTSRCCOPY: TenaryROP =
        TenaryROP((Self::INTERNAL_OP_Sn.0 as u32) << 16 | Self::INTERNAL_OP_Sn.1 as u32);
    pub const SRCERASE: TenaryROP =
        TenaryROP((Self::INTERNAL_OP_SDna.0 as u32) << 16 | Self::INTERNAL_OP_SDna.1 as u32);
    pub const DSTINVERT: TenaryROP =
        TenaryROP((Self::INTERNAL_OP_Dn.0 as u32) << 16 | Self::INTERNAL_OP_Dn.1 as u32);
    pub const PATINVERT: TenaryROP =
        TenaryROP((Self::INTERNAL_OP_DPx.0 as u32) << 16 | Self::INTERNAL_OP_DPx.1 as u32);
    pub const SRCINVERT: TenaryROP =
        TenaryROP((Self::INTERNAL_OP_DSx.0 as u32) << 16 | Self::INTERNAL_OP_DSx.1 as u32);
    pub const SRCAND: TenaryROP =
        TenaryROP((Self::INTERNAL_OP_DSa.0 as u32) << 16 | Self::INTERNAL_OP_DSa.1 as u32);
    pub const MERGEPAINT: TenaryROP =
        TenaryROP((Self::INTERNAL_OP_DSno.0 as u32) << 16 | Self::INTERNAL_OP_DSno.1 as u32);
    pub const MERGECOPY: TenaryROP =
        TenaryROP((Self::INTERNAL_OP_PSa.0 as u32) << 16 | Self::INTERNAL_OP_PSa.1 as u32);
    pub const SRCCOPY: TenaryROP =
        TenaryROP((Self::INTERNAL_OP_S.0 as u32) << 16 | Self::INTERNAL_OP_S.1 as u32);
    pub const SRCPAINT: TenaryROP =
        TenaryROP((Self::INTERNAL_OP_DSo.0 as u32) << 16 | Self::INTERNAL_OP_DSo.1 as u32);
    pub const PATCOPY: TenaryROP =
        TenaryROP((Self::INTERNAL_OP_P.0 as u32) << 16 | Self::INTERNAL_OP_P.1 as u32);
    pub const PATPAINT: TenaryROP =
        TenaryROP((Self::INTERNAL_OP_DPSnoo.0 as u32) << 16 | Self::INTERNAL_OP_DPSnoo.1 as u32);
    pub const WHITENESS: TenaryROP =
        TenaryROP((Self::INTERNAL_OP_1.0 as u32) << 16 | Self::INTERNAL_OP_1.1 as u32);
}
