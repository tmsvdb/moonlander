use piston_window::*;
//use gfx_device_gl::{Resources, Output, CommandBuffer};
//use gfx::Resources;
//use gfx_graphics::GfxGraphics;

pub struct Object {
    x: f64,
    y: f64,
    //sprite: Option<Texture<Resources>>,
}

#[allow(dead_code)]
impl Object {
    pub fn new() -> Object {
        Object {x : 0.0, y : 0.0}//, sprite: None}
    }
    pub fn mov(&mut self, x: f64, y: f64) {
        self.x += x;
        self.y += y;
    }
    pub fn mov_to(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }
/*
    pub fn render(&self, g: &mut GfxGraphics<Resources>, view: math::Matrix2d) {
        match self.sprite {
        None => {
            rectangle([1.0, 0.0, 0.0, 1.0], rectangle::square(0.0, 0.0, 100.0), view.trans(self.x, self.y).trans(-50.0, -50.0), g);
        }
        Some(ref sprite) => {
            image(sprite, view.trans(self.x, self.y).trans(-50.0, -50.0), g);
        }
    }
    pub fn set_sprite(&mut self, sprite: Texture<Resources>) {
        self.sprite = Some(sprite);
    }*/
}
