use std::io::stdin;
use crate::connect4::*;
use crate::board::{BoardSize, Difficulty};
// use crate::otto::*;
// use crate::ottobot::*;



fn get_menu_choice(menu: &str, n: u32, ret: &mut u32) {
    // obtaining an integer selection from CLI menu
    loop {
        println!("{}\n", menu);
        let mut s: String = "".to_string();

        match stdin().read_line(&mut s) {
            Err(_) => println!("Something went wrong reading input, please try again."),
            Ok(_) => {}
        };
    
        match s.trim().parse::<u32>() {
            Err(_) => println!("Please enter a valid integer.\n"),
            Ok(p) => {
                if 1 <= p && p <= n {
                    *ret = p;
                    println!("");
                    break;
                } else {
                    println!("Please select a valid option.\n");
                }
            }
        }    
    }
}


fn get_continue() {
    // force user to press enter so menu isn't printed immediately after output
    loop {
        println!("Please press 'enter' to continue...");
        let mut s: String = "".to_string();

        match stdin().read_line(&mut s) {
            Err(_) => println!("Something went wrong reading input, please try again."),
            Ok(_) => {
                println!("");
                break;
            }
        };
    }
}

fn play_connect4(size: u32, mode: u32) -> String {
    println!("Starting Connect4....");
    let size = match size {
        1 => BoardSize::Standard,
        2 => BoardSize::Large,
        _ => BoardSize::Standard,
    };

    let mut game = Connect4Board::new(size);

    let result = match mode {
        1 => game.host_game(),
        2 => game.host_game_AI(Difficulty::Easy),
        3 => game.host_game_AI(Difficulty::Hard),
        _ => "".to_string(),
    };

    result
}

// fn play_otto(size: u32, mode: u32) -> String {
//     println!("Starting Totto & Otto....");
//     let size = match size {
//         1 => BoardSize::Standard,
//         2 => BoardSize::Large,
//         _ => BoardSize::Standard,
//     };

//     let mut game = TootOttoBoard::new(size);

//     let result = match mode {
//         1 => game.host_game(),
//         // 2 => game.host_game_AI(Difficulty::Easy),
//         // 3 => game.host_game_AI(Difficulty::Hard),
//         2 => "(unimplemented!)".to_string(),
//         3 => "(unimplemented!)".to_string(),
//         _ => "".to_string(),
//     };

//     result
// }

#[allow(dead_code)]
pub fn run_cli() {
    // choose a type of tree

    loop {
        let mut g = 0;
        get_menu_choice("Please select a game:
1. Connect4
2. TOOT and OTTO
3. (Exit Program)", 3, &mut g);

        if g == 3u32 {
            break;
        }

        let mut s = 0;
        get_menu_choice("Please select board size:
1. Standard Board
2. Larger Board", 2, &mut s);

        let mut gm = 0;
        get_menu_choice("Please select a game mode:
1. vs Player
2. vs Computer (Easy)
3. vs Computer (Hard)", 3, &mut gm);

        // enter loop with a tree of user's choice
        let result = match g {
            1u32 => {
                play_connect4(s, gm)
            },
            2u32 => {
                // play_otto(s, gm)
                "unimplemented".to_string()
            },
            _ => "Something went wrong, please try again".to_string(),
        };
        get_continue();
    }
}