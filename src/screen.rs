extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

const COLOR_OFF: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 0xFF,
};
const COLOR_ON: Color = Color {
    r: 0xFF,
    g: 0xFF,
    b: 0xFF,
    a: 0xFF,
};
pub const SCALE: usize = 10;
pub const SCREEN_WIDTH_BASE: usize = 64;
pub const SCREEN_WIDTH: usize = SCREEN_WIDTH_BASE * SCALE;
pub const SCREEN_HEIGHT_BASE: usize = 32;
pub const SCREEN_HEIGHT: usize = SCREEN_HEIGHT_BASE * SCALE;
pub const NUM_PIXELS: usize = SCREEN_WIDTH_BASE * SCREEN_HEIGHT_BASE;

pub struct Screen {
    pixels: [bool; NUM_PIXELS],
    canvas: sdl2::render::WindowCanvas,
}

impl Screen {
    pub fn new(ctx: &sdl2::Sdl) -> Screen {
        let video_subsystem = ctx.video().unwrap();
        let window = video_subsystem.window("Chip8", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
            .position_centered()
            .build()
            .map_err(|e| e.to_string()).unwrap();
        let mut canvas = window.into_canvas().
            build().
            map_err(|e| e.to_string()).unwrap();
        canvas.set_draw_color(COLOR_ON);
        let mut screen = Screen { pixels: [false; NUM_PIXELS], canvas: canvas, };
        screen.clear_screen();
        return screen;
    }

    pub fn clear_screen(&mut self) {
        self.pixels = [false; NUM_PIXELS];
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn draw(&mut self) {
        self.canvas.set_draw_color(COLOR_OFF);
        self.canvas.clear();
        self.canvas.set_draw_color(COLOR_ON);
        for number in 0..(NUM_PIXELS) {
            if self.pixels[number] {
                let xcoord = ((number % SCREEN_WIDTH_BASE) * SCALE) as i32;
                let ycoord = ((number / SCREEN_WIDTH_BASE) * SCALE) as i32;
                let rect = Rect::new(xcoord, ycoord, SCALE as u32, SCALE as u32);
                self.canvas.fill_rect(rect).unwrap();
            }
        }
        self.canvas.present();
    }

    pub fn get_pixel(&mut self, idx: usize) -> bool {
        return self.pixels[idx];
    }

    pub fn set_pixel(&mut self, idx: usize, on: bool) {
        self.pixels[idx] = on;
    }
}