use crate::vector::Vector;

#[derive(Debug, Copy, Clone)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: u32,
    pub h: u32
}

impl Rect {
    pub fn new(x: f32, y: f32, w: u32, h: u32) -> Rect {
        Rect {x, y, w, h}
    }

    pub fn sdl2(&self) -> sdl2::rect::Rect {
        sdl2::rect::Rect::new(self.x as i32, self.y as i32, self.w, self.h)
    }

    pub fn has_intersection(&self, other: Rect) -> bool {
        let left = self.x;
        let right = self.x + self.w as f32;
        let top = self.y;
        let bottom = self.y + self.h as f32;

        let o_left = other.x;
        let o_right = other.x + other.w as f32;
        let o_top = other.y;
        let o_bottom = other.y + other.h as f32;

        if right <= o_left || o_right <= left {
            return false;
        }

        if top >= o_bottom || o_top >= bottom {
            return false;
        }

        return true;
    }

    pub fn apply_vector(&mut self, v: Vector) {
        self.x += v.x();
        self.y += v.y();
    }

    pub fn after_depth(mut self, d: u32) -> Rect {
        self.y += self.h as f32 - d as f32;
        self.h = d;

        self
    }
}

pub struct GeometryComponent {
    rect: Rect
}

impl GeometryComponent {
    pub fn new(x: f32, y: f32, width: u32, height: u32) -> GeometryComponent {
        GeometryComponent {
            rect: Rect::new(x, y, width, height)
        }
    }

    pub fn rect(&self) -> &Rect {
        &self.rect
    }

    pub fn rect_mut(&mut self) -> &mut Rect {
        &mut self.rect
    }
}