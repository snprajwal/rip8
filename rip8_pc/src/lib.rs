mod beep;

use rip8_core::*;
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
};

const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (DISPLAY_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (DISPLAY_HEIGHT as u32) * SCALE;
// The CHIP-8 spec does not specify the clock speed,
// but 8 cycles per frame limits the rendering to a
// maximum of 60 sprites per second. This simulates
// the original CHIP-8 behaviour of waiting for the
// vertical blank interrupt.
const TPS: usize = 8;

pub fn run(rom: &[u8]) {
    let ctx = sdl2::init().expect("failed to initialise SDL2 context");
    let video = ctx.video().expect("failed to create video subsystem");
    let audio = beep::init_audio(&ctx).expect("failed to create audio subsystem");
    let mut event_pump = ctx.event_pump().expect("failed to get event pump");

    let window = video
        .window(
            "RIP-8 - the Rust CHIP-8 emulator",
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        )
        .position_centered()
        .build()
        .expect("failed to build window");
    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .expect("failed to build canvas");
    canvas.clear();
    canvas.present();

    let mut chip = Chip::new();
    chip.load(&rom);

    let mut wait = false;
    let mut beep_delay: u8 = 0;
    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return,
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    if let Some(k) = keymap(key) {
                        wait = chip.key_down(k);
                        if wait {
                            // Process just the current instruction
                            chip.tick();
                        }
                    }
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    if let Some(k) = keymap(key) {
                        chip.key_up(k);
                        wait = false;
                        break;
                    }
                }
                _ => (),
            }
        }
        // Do not execute any more instructions
        // until the pressed key is released
        if wait {
            continue;
        }

        for _ in 0..TPS {
            chip.tick();
        }
        // The timers run at the refresh rate, not the clock speed
        let beep = chip.tick_timers();
        if beep {
            // The beep is too short to be heard in a single cycle.
            // Three cycles are required at minimum for it to be heard.
            beep_delay = 3;
            audio.resume();
        } else if beep_delay == 0 {
            audio.pause();
        } else {
            beep_delay -= 1;
        }

        draw(&chip, &mut canvas);
    }
}

fn draw(chip: &Chip, canvas: &mut Canvas<Window>) {
    // Draw black to clear canvas
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let display = chip.display();
    // Draw white, see which points need to be drawn
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for (i, &px) in display.iter().enumerate() {
        if px {
            let x = (i % DISPLAY_WIDTH) as i32 * SCALE as i32;
            let y = (i / DISPLAY_WIDTH) as i32 * SCALE as i32;

            // Draw a rectangle at (x, y)
            canvas
                .fill_rect(Rect::new(x, y, SCALE, SCALE))
                .expect("failed to fill rectangle");
        }
    }

    canvas.present();
}

fn keymap(key: Keycode) -> Option<usize> {
    match key {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xC),

        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        #[cfg(not(feature = "colemak"))]
        Keycode::E => Some(0x6),
        #[cfg(not(feature = "colemak"))]
        Keycode::R => Some(0xD),
        #[cfg(feature = "colemak")]
        Keycode::F => Some(0x6),
        #[cfg(feature = "colemak")]
        Keycode::P => Some(0xD),

        Keycode::A => Some(0x7),
        #[cfg(not(feature = "colemak"))]
        Keycode::S => Some(0x8),
        #[cfg(not(feature = "colemak"))]
        Keycode::D => Some(0x9),
        #[cfg(not(feature = "colemak"))]
        Keycode::F => Some(0xE),
        #[cfg(feature = "colemak")]
        Keycode::R => Some(0x8),
        #[cfg(feature = "colemak")]
        Keycode::S => Some(0x9),
        #[cfg(feature = "colemak")]
        Keycode::T => Some(0xE),

        Keycode::Z => Some(0xA),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xB),
        Keycode::V => Some(0xF),
        _ => None,
    }
}
