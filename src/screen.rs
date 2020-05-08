use minifb::{Key, WindowOptions, Window, Scale, KeyRepeat};

const CHAR_0: [u8; 5] = [
    0b01100000,
    0b10010000,
    0b10010000,
    0b10010000,
    0b01100000,
];

const CHAR_1: [u8; 5] = [
    0b00100000,
    0b01100000,
    0b00100000,
    0b00100000,
    0b01110000,
];

const SPRITE: [u8; 5] = [
    0b00100100,
    0b00100100,
    0b00000000,
    0b10000001,
    0b01111110,
];

#[derive(Debug)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point {
            x,
            y,
        }
    }
}

pub struct Buffer {
    width: usize,
    height: usize,
    pixels: Vec<u32>,
    pub dirty: bool,
}

impl Buffer {
    pub fn new(width: usize, height: usize, pixels: Option<Vec<u32>>) -> Buffer {
        let mut empty_pixels = Vec::new();
        for _ in 0..(width * height) {
            empty_pixels.push(0u32);
        }

        Buffer {
            width,
            height,
            pixels: match pixels {
                Some(p) => p,
                None => empty_pixels,
            },
            dirty: true,
        }
    }

    pub fn blit(&mut self, buffer: &Buffer, offset: Point) {
        let draw_idx = offset.x + offset.y * self.width;

        for (idx, pixel) in buffer.pixels.iter().enumerate() {
            let target_x = idx % buffer.width;
            let target_y = (idx - target_x) / buffer.width;

            /*
            println!(
                "{}: {} + {}, {} + {}, {} * {}, {} * {}",
                idx, offset.x, target_x, offset.y, target_y,
                self.width, self.height, buffer.width, buffer.height
                );
            */

            if offset.x + target_x > buffer.width || offset.y + target_y > buffer.height {
                continue;
            }

            let draw_idx = offset.x + target_x + (offset.y + target_y) * self.width;

            self.pixels[draw_idx] = *pixel;
        }

        self.dirty = true;
    }

    pub fn clear(&mut self) {
        let mut empty_pixels = Vec::new();
        for _ in 0..(self.width * self.height) {
            empty_pixels.push(0u32);
        }

        self.pixels = empty_pixels;
    }
}

pub struct Screen {
    buffer: Buffer,
    pub game_buffer: Buffer,
    pub debug_buffer: Buffer,

    pub window: Window,
}

impl Screen {
    pub fn new(
            game_width: usize, game_height: usize,
            debug_width: usize, debug_height: usize) -> Screen {

        let total_width = game_width + debug_width;
        let total_height = game_height + debug_height;

        let buffer = Buffer::new(total_width, total_height, None);
        let game_buffer = Buffer::new(game_width, game_height, None);
        let debug_buffer = Buffer::new(debug_width, debug_height, None);

        // Prepare frame buffer
        let mut window = Window::new(
            "CHIP-8 - ESC to exit",
            total_width, total_height,
            WindowOptions {
                resize: false,
                scale: Scale::X4,
                ..WindowOptions::default()
            })
            .unwrap_or_else(|e| { panic!("{}", e); });

        Screen {
            buffer,
            game_buffer,
            debug_buffer,
            window,
        }
    }

    pub fn update(&mut self) {
        // Blit game_buffer and debug_buffer to buffer
        if self.game_buffer.dirty {
            println!("Draw game");
            self.buffer.blit(&self.game_buffer, Point::new(0, 0));
            self.game_buffer.dirty = false;
        }

        if self.debug_buffer.dirty {
            println!("Draw game");
            self.buffer.blit(&self.debug_buffer, Point::new(self.game_buffer.width, 0));
            self.debug_buffer.dirty = false;
        }

        if self.buffer.dirty {
            // Update window with buffer
            self.window.update_with_buffer(&self.buffer.pixels);

            // Clear dirty flag
            self.buffer.dirty = false;
        } else {
            // TODO: Update window
            self.window.update();
        }
    }
}
