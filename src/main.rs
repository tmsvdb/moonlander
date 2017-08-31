extern crate sdl2;
use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use std::time::Duration;

fn main() {
    // pooo

    let sdl_context = sdl2::init().unwrap();
    //let video_subsystem = sdl_context.video().unwrap();
    let video_wrapper = VideoWrapper::new(&sdl_context);
    //let window = video_subsystem.window("SDL2", 640, 480).position_centered().build().unwrap();
    //let window = video_wrapper.window("SDL2", 640, 480);

    //let mut canvas = window.into_canvas().accelerated().build().unwrap();
    //let texture_creator = canvas.texture_creator();



    let mut timer = sdl_context.timer().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    //let sprite = BitmapSprite::load(&texture_creator, "assets/animate.bmp");
    //let temp_surface = sdl2::surface::Surface::load_bmp(Path::new("assets/animate.bmp")).unwrap();
    //let texture = texture_creator.create_texture_from_surface(&temp_surface).unwrap();

    let center = Point::new(320,240);
    let mut source_rect = Rect::new(0, 0, 128, 82);
    let mut dest_rect = Rect::new(0,0, 128, 82);
    dest_rect.center_on(center);

    let mut running = true;
    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    running = false;
                },
                _ => {}
            }
        }

        let ticks = timer.ticks();



        //std::thread::sleep(Duration::from_millis(100));
    }
}

pub struct VideoWrapper <'a>
{
    window:   sdl2::video::Window,
    sprites : Vec<sdl2::render::Texture<'a>>,
}

impl <'a> VideoWrapper <'a>
{
    /// create new VideoWrapper
    pub fn new (sdl_context: &sdl2::Sdl) -> VideoWrapper<'a>
    {
        let sub = sdl_context.video().unwrap();
        let win : sdl2::video::Window = sub.window("VideoWrapper", 640, 480).position_centered().build().unwrap();

        VideoWrapper {
            window: win,
            sprites: Vec::new(),
        }
    }

    pub fn load (&self, id: &str, path: &str)
    {
        let vw : VideoWrapper<'a> = &self;
        let mut can = vw.window.into_canvas().accelerated().build().unwrap();
        //can.set_draw_color(sdl2::pixels::Color::RGBA(0,0,0,255));
        let tc = can.texture_creator();
        let temp_surface = sdl2::surface::Surface::load_bmp(Path::new(path)).unwrap();
        let result = tc.create_texture_from_surface(&temp_surface);
        let texture = result.unwrap();
        &self.sprites.push(texture);
    }

    pub fn render (&self)
    {
        let canvas = &self.window.into_canvas().accelerated().build().unwrap();
        //source_rect.set_x((128 * ((ticks / 100) % 6) ) as i32);
        canvas.clear();
        for sprite in &self.sprites {
            canvas.copy(&sprite, None, None).unwrap();
        }
        //canvas.copy_ex(&sprite.texture, Some(source_rect), Some(dest_rect), 10.0, None, true, false).unwrap();
        canvas.present();
    }
}



/*

    TEXT RENDER CODE


extern crate sdl2;

use std::path::Path;
use std::time::{Duration, Instant};
use std::thread::sleep;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use sdl2::pixels::Color;

static SCREEN_WIDTH : u32 = 800;
static SCREEN_HEIGHT : u32 = 600;

// handle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

// Scale fonts to a reasonable size when they're too big (though they might look less smooth)
fn get_centered_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            println!("Scaling down! The text will look worse!");
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            println!("Scaling down! The text will look worse!");
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    let cx = (SCREEN_WIDTH as i32 - w) / 2;
    let cy = (SCREEN_HEIGHT as i32 - h) / 2;
    rect!(cx, cy, w, h)
}

fn run(font_path: &Path) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    let window = video_subsys.window("SDL2_TTF Example", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    // Load a font
    let mut font = ttf_context.load_font(font_path, 128).unwrap();
    font.set_style(sdl2::ttf::STYLE_BOLD);

    // render a surface, and convert it to a texture bound to the canvas
    let mut surface = font.render("Hello Rust!")
        .blended(Color::RGBA(255, 0, 0, 255)).unwrap();
    let mut texture = texture_creator.create_texture_from_surface(&surface).unwrap();

    canvas.set_draw_color(Color::RGBA(195, 217, 255, 255));
    canvas.clear();

    let TextureQuery { width, height, .. } = texture.query();

    // If the example text is too big for the screen, downscale it (and center irregardless)
    let padding = 64;
    let target = get_centered_rect(width, height, SCREEN_WIDTH - padding, SCREEN_HEIGHT - padding);

    canvas.copy(&texture, None, Some(target)).unwrap();
    canvas.present();

    let mut frames: u32 = 0;
    let mut fps: u32 = 0;
    let mut now = Instant::now();

    'mainloop: loop {
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::KeyDown {keycode: Some(Keycode::Escape), ..} |
                Event::Quit {..} => break 'mainloop,
                _ => {}
            }
        }

        frames += 1;

        if now.elapsed().as_secs() >= 1
        {
            fps = frames;
            frames = 0;
            now = Instant::now();
            surface = font.render(fps.to_string().as_str()).blended(Color::RGBA(255, 0, 0, 255)).unwrap();
            texture = texture_creator.create_texture_from_surface(&surface).unwrap();
        }

        canvas.clear();
        canvas.copy(&texture, None, Some(target)).unwrap();
        //canvas.copy(
            //&texture_creator.create_texture_from_surface(
        //        &font.render(fps.to_string().as_str()).blended(Color::RGBA(255, 0, 0, 255)).unwrap()
        //    ).unwrap(), None, Some(target)
        //).unwrap();
        canvas.present();

    }
}

fn main() {
    let path: &Path = Path::new("assets/font.ttf");
    run(path);
}
*/

/*

CONTROLLER CODE


extern crate sdl2;

use sdl2::event::{Event};
use sdl2::rect::{Rect};

pub mod controller_wrapper;
use controller_wrapper::ControllerWrapper;

fn main() {
    let ctx = sdl2::init().unwrap();
    let controller_wrapper = ControllerWrapper::new (&ctx);


    let mut events = ctx.event_pump().unwrap();

    // loop until we receive a QuitEvent
    'event : loop {

        for event in events.poll_iter() {
            match event {
                Event::ControllerAxisMotion{ axis, value: val, .. } => {
                    let dead_zone = 10000;
                    if val > dead_zone || val < -dead_zone {
                        println!("Axis {:?} moved to {}", axis, val);
                    }
                }
                Event::ControllerButtonDown{ button, .. } =>
                    println!("Button {:?} down", button),
                Event::ControllerButtonUp{ button, .. } =>
                    println!("Button {:?} up", button),
                Event::Quit{..} => break,
                _ => (),
            }
        }

        // poll_event returns the most recent event or NoEvent if nothing has happened
        for event in events.poll_iter() {
            match event {
                Event::Quit{..} => break 'event,
                _               => continue
            }
        }
    }
}
*/
