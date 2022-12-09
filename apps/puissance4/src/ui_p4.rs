// def print_grid():
//     fill_rect(50, 50, 212, 170, black)
//     for x in range(7):
//         fill_rect(52 + x * 30, 50, 28, 168, white)
//         for y in range(6):
//             if not darkMode:
//                 print_coin(x, y, (240, 240, 240))
//             else:
//                 print_coin(x, y, (30, 30, 30))

use crate::{
    eadk::{display::push_rect_uniform, Color, Rect},
    utils::{fill_screen, CENTER},
};

// fill_rect(53 + 30 * posx, 191 - 28 * posy, 26, 26, color)

const COIN_SIZE: u16 = 26;
const LEFT_POS: u16 = CENTER.x - 3 * (COIN_SIZE + 2) - COIN_SIZE / 2 - 2;
const UP_POS: u16 = 60;

pub fn draw_coin(x: u16, y: u16, color: Color) {
    push_rect_uniform(
        Rect {
            x: LEFT_POS + 2 + (COIN_SIZE + 2) * x,
            y: UP_POS + (COIN_SIZE + 1) * (5 - y),
            width: COIN_SIZE,
            height: COIN_SIZE,
        },
        color,
    )
}

pub fn draw_grid() {
    fill_screen(Color::WHITE);
    push_rect_uniform(
        Rect {
            x: LEFT_POS,
            y: UP_POS,
            width: (COIN_SIZE + 2) * 7 + 2,
            height: COIN_SIZE * 6 + 8,
        },
        Color::BLACK,
    );
    for x in 0..7 {
        push_rect_uniform(
            Rect {
                x: LEFT_POS + 2 + (COIN_SIZE + 2) * x,
                y: UP_POS,
                width: COIN_SIZE,
                height: COIN_SIZE * 6 + 6,
            },
            Color::WHITE,
        );
        for y in 0..6 {
            push_rect_uniform(
                Rect {
                    x: LEFT_POS + 4 + (COIN_SIZE + 2)*x,
                    y: UP_POS + 1 + (COIN_SIZE + 1) * (5 - y),
                    width: COIN_SIZE-4,
                    height: COIN_SIZE-2,
                },
                Color::from_rgb888(240, 240, 240),
            )
        }
    }
}