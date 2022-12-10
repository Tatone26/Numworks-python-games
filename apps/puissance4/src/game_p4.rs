use heapless::Vec;

use crate::{
    eadk::{
        display::{self, SCREEN_HEIGHT, SCREEN_WIDTH},
        key, keyboard, timing, Color,
    },
    menu::{menu, selection_menu, MenuConfig, MyOption},
    ui_p4::{clear_selection_coin, draw_coin, draw_grid, draw_selection_coin, victory},
    utils::{wait_for_no_keydown, ColorConfig, LARGE_CHAR_HEIGHT, draw_centered_string},
};

/// The number of Boolean Options used. Public so menu() can use it.
pub const BOOL_OPTIONS_NUMBER: usize = 2;

// This dictates the principal colors that will be used
const COLOR_CONFIG: ColorConfig = ColorConfig {
    text: Color::BLACK,
    bckgrd: Color::WHITE,
    alt: Color::from_rgb888(90, 90, 255),
};

static mut THREE_PLAYERS: bool = false;

fn vis_addon() {
    draw_coin(2, 5, FIRST_COLOR);
    draw_coin(3, 5, SECOND_COLOR);
    draw_coin(4, 5, FIRST_COLOR)
}
/// Menu, Options and Game start
pub fn start() {
    let mut opt: [&mut MyOption<bool, 2>; BOOL_OPTIONS_NUMBER] = [
        &mut MyOption {
            name: "3 Joueurs\0",
            value: 1,
            possible_values: [(true, "Oui\0"), (false, "Non\0")],
        },
        &mut MyOption {
            name: "Mode sombre\0",
            value: 1,
            possible_values: [(true, "Oui\0"), (false, "Non\0")],
        },
    ];
    loop {
        let start = menu("PUISSANCE 4\0", &mut opt, &COLOR_CONFIG, vis_addon); // The menu does everything itself !
        if start == 1 {
            unsafe {
                THREE_PLAYERS = opt[0].get_value().0; // You could use mutable statics, but it is not very good
            }
            loop {
                let color_config: ColorConfig;
                if opt[1].get_value().0 {
                    color_config = ColorConfig { text: COLOR_CONFIG.bckgrd, bckgrd: COLOR_CONFIG.text, alt: COLOR_CONFIG.alt }
                } else {
                    color_config = COLOR_CONFIG
                }
                // a loop where the game is played again and again, which means it should be 100% contained after the menu
                let action = game(opt[0].get_value().0, &color_config); // calling the game based on the parameters is better
                if action == 0 {
                    // 0 means quitting
                    return;
                } else if action == 2 {
                    // 2 means back to menu
                    break;
                } // if action == 1 : rejouer
            }
        } else {
            return;
        }
    }
}

pub const MAX_WIDTH_SIZE: usize = 8;
pub const MAX_HEIGHT_SIZE: usize = 6;

pub const FIRST_COLOR: Color = Color::RED;
pub const SECOND_COLOR: Color = Color::BLUE;
pub const THIRD_COLOR: Color = Color::from_rgb888(250, 200, 0);

/// The entire game is here.
pub fn game(three_players: bool, c: &ColorConfig) -> u8 {
    let mut table: Vec<Vec<u8, MAX_HEIGHT_SIZE>, MAX_WIDTH_SIZE> = Vec::new();
    for _ in 0..MAX_WIDTH_SIZE {
        let mut new_vec = Vec::<u8, MAX_HEIGHT_SIZE>::new();
        for _ in 0..MAX_HEIGHT_SIZE {
            new_vec.push(0).unwrap();
        }
        table.push(new_vec).unwrap();
    }
    let mut first_player_pos: u16 = 3;
    let mut second_player_pos: u16 = 3;
    let mut third_player_pos: u16 = 3;
    draw_grid(three_players, c);
    loop {
        first_player_pos = selection(first_player_pos, FIRST_COLOR, three_players, c);
        while !table
            .get(first_player_pos as usize)
            .unwrap()
            .last()
            .eq(&Some(&0))
        {
            first_player_pos = selection(first_player_pos, FIRST_COLOR, three_players, c);
        }
        place_coin(first_player_pos, 1, &mut table);
        let check_1 = check(&table);
        if check_1.is_some() {
            victory(check_1, c);
            break;
        }
        if table_is_full(&table, three_players) {
            draw_centered_string("Egalite !\0", 10, true, c, false);
            break;
        }
        second_player_pos = selection(second_player_pos, SECOND_COLOR, three_players, c);
        while !table
            .get(second_player_pos as usize)
            .unwrap()
            .last()
            .eq(&Some(&0))
        {
            second_player_pos = selection(second_player_pos, SECOND_COLOR, three_players, c);
        }
        place_coin(second_player_pos, 2, &mut table);
        let check_2 = check(&table);
        if check_2.is_some() {
            victory(check_2, c);
            break;
        }
        if table_is_full(&table, three_players) {
            draw_centered_string("Egalité !\0", 10, true, c, false);
            break;
        }
        if three_players {
            third_player_pos = selection(third_player_pos, THIRD_COLOR, three_players, c);
            while !table
                .get(third_player_pos as usize)
                .unwrap()
                .last()
                .eq(&Some(&0))
            {
                third_player_pos = selection(third_player_pos, THIRD_COLOR, three_players, c);
            }
            place_coin(third_player_pos, 3, &mut table);
            let check_3 = check(&table);
            if check_3.is_some() {
                victory(check_3, c);
                break;
            }
            if table_is_full(&table, three_players) {
                draw_centered_string("Egalité !\0", 10, true, c, false);
                break;
            }
        }
    }
    let menu_config = MenuConfig {
        first_choice: "Replay\0",
        second_choice: "Menu\0",
        null_choice: "Exit\0",
        rect_margins: (0, 0),
        dimensions: (SCREEN_WIDTH, LARGE_CHAR_HEIGHT),
        offset: (0, SCREEN_HEIGHT / 2 - LARGE_CHAR_HEIGHT),
        back_key_return: 2,
    };
    return selection_menu(c, &menu_config, true);
}

fn table_is_full(
    table: &Vec<Vec<u8, MAX_HEIGHT_SIZE>, MAX_WIDTH_SIZE>,
    three_players: bool,
) -> bool {
    if three_players {
        for i in table {
            for j in i {
                if *j == 0 {
                    return false;
                }
            }
        }
        return true;
    } else {
        for i in 0..table.len() - 1 {
            for j in table.get(i).unwrap() {
                if *j == 0 {
                    return false;
                }
            }
        }
        return true;
    }
}

fn place_coin(x: u16, number: u8, table: &mut Vec<Vec<u8, MAX_HEIGHT_SIZE>, MAX_WIDTH_SIZE>) {
    let vec_x = table.get_mut(x as usize).unwrap();
    let mut y = 0;
    for i in vec_x {
        if *i == 0 {
            *i = i.checked_add(number).unwrap();
            break;
        } else {
            y += 1
        }
    }
    draw_coin(
        x,
        y,
        if number == 2 {
            SECOND_COLOR
        } else if number == 1 {
            FIRST_COLOR
        } else if number == 3 {
            THIRD_COLOR
        } else {
            Color::WHITE
        },
    );
}

const REPETITION_SPEED: u64 = 250;

fn selection(initial_pos: u16, color: Color, three_players: bool, c: &ColorConfig) -> u16 {
    let mut pos = initial_pos;
    wait_for_no_keydown();
    let mut last_action: u64 = timing::millis();
    let mut last_action_key: u32 = key::ALPHA;
    draw_selection_coin(initial_pos, color);
    loop {
        let keyboard_state = keyboard::scan();
        if (keyboard_state.key_down(key::LEFT) | keyboard_state.key_down(key::RIGHT))
            & (timing::millis() >= last_action + REPETITION_SPEED)
        {
            let old_pos = pos;
            if keyboard_state.key_down(key::LEFT) {
                last_action_key = key::LEFT;
                if pos > 0 {
                    pos -= 1;
                }
            } else if keyboard_state.key_down(key::RIGHT) {
                last_action_key = key::RIGHT;
                if pos < {
                    if !three_players {
                        6
                    } else {
                        7
                    }
                } {
                    pos += 1;
                }
            }
            if old_pos != pos {
                clear_selection_coin(old_pos, c);
                draw_selection_coin(pos, color);
                display::wait_for_vblank();
            }
            last_action = timing::millis();
        } else if keyboard_state.key_down(key::OK) | keyboard_state.key_down(key::DOWN) {
            wait_for_no_keydown();
            clear_selection_coin(pos, c);
            break;
        } else if !keyboard_state.key_down(last_action_key) {
            last_action = timing::millis() - REPETITION_SPEED;
        }
    }
    return pos;
}

fn check(
    table: &Vec<Vec<u8, MAX_HEIGHT_SIZE>, MAX_WIDTH_SIZE>,
) -> Option<(u8, (u16, u16), (u16, u16))> {
    for x in 0..MAX_WIDTH_SIZE - 3 {
        let x_vec = table.get(x).unwrap();
        if x_vec.len() == 0 {
            panic!()
        }
        for y in 0..MAX_HEIGHT_SIZE {
            let t = table[x][y];
            if t != 0 && t == table[x + 1][y] && t == table[x + 2][y] && t == table[x + 3][y] {
                return Some((t, (x as u16, y as u16), (x as u16 + 3, y as u16)));
            }
        }
        for y in 0..MAX_HEIGHT_SIZE - 3 {
            let t = table[x][y];
            if t != 0
                && t == table[x + 1][y + 1]
                && t == table[x + 2][y + 2]
                && t == table[x + 3][y + 3]
            {
                return Some((t, (x as u16, y as u16), (x as u16 + 3, y as u16 + 3)));
            }
        }
        for y in MAX_HEIGHT_SIZE - 3..MAX_HEIGHT_SIZE {
            let t = table[x][y];
            if t != 0
                && t == table[x + 1][y - 1]
                && t == table[x + 2][y - 2]
                && t == table[x + 3][y - 3]
            {
                return Some((t, (x as u16, y as u16), (x as u16 + 3, y as u16 - 3)));
            }
        }
    }
    for x in 0..MAX_WIDTH_SIZE {
        for y in 0..MAX_HEIGHT_SIZE - 3 {
            let t = table[x][y];
            if t != 0 && t == table[x][y + 1] && t == table[x][y + 2] && t == table[x][y + 3] {
                return Some((t, (x as u16, y as u16), (x as u16, y as u16 + 3)));
            }
        }
    }

    return None;
}
