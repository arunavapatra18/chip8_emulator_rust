use chip8_core::*;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use std::fs::File;
use std::io::Read;

use std::env;

const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;
const TICKS_PER_FRAME: usize = 10;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        print!("Usage: cargo run path/to/game");
        return;
    }

    // Setup SDL
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Chip-8 Emulator", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();

    // SDL2 Event Pump polls for every loop
    let mut event_pump = sdl_context.event_pump().unwrap();

    //------------INITIALIZE EMU--------------//
    let mut chip8 = Emu::new();

    // Open the ROM file, read into a buffer, and load the buffer.
    let mut rom = File::open(&args[1]).expect("Unable to open file");
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).unwrap();
    chip8.load(&buffer);

    // Main gameloop
    'gameloop: loop {
        for evt in event_pump.poll_iter() {     // Checks if any events have been triggered
            match evt {
                Event::Quit{..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {      // Handles Quit
                    break 'gameloop;
                },

                Event::KeyDown{keycode: Some(key), ..} => {                            // Handles Keydown                                          
                    if let Some(k) = key2btn(key) {
                        chip8.keypress(k, true);
                    }
                },

                Event::KeyUp{keycode: Some(key), ..} => {                              // Handles Keyup
                    if let Some(k) = key2btn(key) {
                        chip8.keypress(k, false);
                    }
                },
                _ => ()
            }
        }

        // Clock cycle
        for _ in 0..TICKS_PER_FRAME {
            chip8.tick();
        }

        chip8.tick_timers();
        
        // Draw screen
        draw_screen(&chip8, &mut canvas);
    }
}

// Draw screen
fn draw_screen(emu: &Emu, canvas: &mut Canvas<Window>) {
    // Clear canvas as black
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let screen_buf = emu.get_display();

    // Now set draw color white, iterate through each point and see if it should be drawn
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for (i, pixel) in screen_buf.iter().enumerate() {
        if *pixel {
            // Convert our 1D array's index into a 2D (x,y) position
            let x = (i % SCREEN_WIDTH) as u32;
            let y = (i / SCREEN_WIDTH) as u32;

            // Draw a rectangle at (x,y), scaled up by our SCALE value
            let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
            canvas.fill_rect(rect).unwrap();
        }
    }
    canvas.present();
}

fn key2btn(key: Keycode) -> Option<usize> {
    match key {
        Keycode:: Num1 => Some(0x1),
        Keycode:: Num2 => Some(0x2),
        Keycode:: Num3 => Some(0x3),
        Keycode:: Num4 => Some(0xC),
        Keycode:: Q =>    Some(0x4),
        Keycode:: W =>    Some(0x5),
        Keycode:: E =>    Some(0x6),
        Keycode:: R =>    Some(0xD),
        Keycode:: A =>    Some(0x7),
        Keycode:: S =>    Some(0x8),
        Keycode:: D =>    Some(0x9),
        Keycode:: F =>    Some(0xE),
        Keycode:: Z =>    Some(0xA),
        Keycode:: X =>    Some(0x0),
        Keycode:: C =>    Some(0xB),
        Keycode:: V =>    Some(0xF),
        _ => None,
    }
}   
