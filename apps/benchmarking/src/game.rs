use core::str::FromStr;

use heapless::{String, Vec};
use numworks_utils::utils::draw_centered_string;

use crate::{
    eadk::{
        display::{self, push_rect_uniform},
        key, keyboard, Color, Point, Rect,
    },
    menu::{menu, MyOption, OptionType},
    utils::{
        draw_string_cfg, draw_tile, fill_screen, tiling, wait_for_no_keydown, ColorConfig, Tileset,
    },
};

// This dictates the principal colors that will be used
const COLOR_CONFIG: ColorConfig = ColorConfig {
    text: Color::BLACK,
    bckgrd: Color::WHITE,
    alt: Color::RED,
};

static mut EXEMPLE: bool = false;

fn vis_addon() {
    push_rect_uniform(
        Rect {
            x: 0,
            y: 0,
            width: 10,
            height: 10,
        },
        Color::BLACK,
    );
}
/// Menu, Options and Game start
pub fn start() {
    let mut opt: [&mut MyOption; 1] = [&mut MyOption {
        name: "Option !\0",
        value: 0,
        possible_values: {
            let mut v = Vec::new();
            unsafe { v.push_unchecked((OptionType::Bool(true), "True\0")) };
            unsafe { v.push_unchecked((OptionType::Bool(false), "False\0")) };
            v
        },
    }];
    loop {
        let start = menu(
            "SNAKE 2.0\0",
            &mut opt,
            &COLOR_CONFIG,
            vis_addon,
            include_str!("./model_controls.txt"),
        );
        // The menu does everything itself !
        if start == 0 {
            unsafe {
                EXEMPLE = opt[0].get_param_value(); // You could use mutable statics, but it is not very good
            }
            loop {
                // a loop where the game is played again and again, which means it should be 100% contained after the menu
                let action = game(opt[0].get_param_value()); // calling the game based on the parameters is better
                if action == 2 {
                    // 2 means quitting
                    return;
                } else if action == 1 {
                    // 1 means back to menu
                    break;
                } // if action == 0 : rejouer
            }
        } else {
            return;
        }
    }
}

/// Images work really well with square tiles. You can still draw other images, but it is less good.
static TILESET: Tileset = Tileset {
    tile_size: 55,
    image: include_bytes!("./icon_snake.ppm"),
};
const PIXELS: usize = { 55 * 55 } as usize;

/// The entire game is here.
pub fn game(_exemple: bool) -> u8 {
    {
        fill_screen(Color::WHITE);
        draw_string_cfg(
            "Push <OK> for some magic ! <Back> to quit back to menu.\0",
            Point::new(0, 0),
            false,
            &COLOR_CONFIG,
            true,
        );
        let memory_test: Vec<u16, 64000> = Vec::new(); // yes to 32000, 64000,  no to 320000, 264000, 128000, 96000
                                                       // from this, I can maybe conclude that I have around 128 000 bytes at my disposition, which is around 125 Kb.
                                                       // Good news, it passes the apparent 32Kb stack size !
        let mut text: String<52> = String::from_str("Memory test passed ! ").unwrap();
        text.push(memory_test.len() as u8 as char).unwrap();
        draw_centered_string(&text, 50, true, &COLOR_CONFIG, true);

        loop {
            let keyboard_state = keyboard::scan();
            if keyboard_state.key_down(key::OK) {
                wait_for_no_keydown();
                // fill_screen(Color::WHITE);
                display::wait_for_vblank();
                tiling::<PIXELS>(
                    &TILESET,
                    Point::new(0, 0),
                    (5, 4),
                    Point::new(0, 0),
                    false,
                    1,
                );
                draw_tile::<PIXELS>(&TILESET, Point::new(0, 0), Point::new(0, 0), 2, false);
            } else if keyboard_state.key_down(key::BACK) {
                break;
            }
        }
    }
    fill_screen(Color::GREEN);
    return 1;
}
