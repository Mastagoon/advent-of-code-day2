use std::fs;

fn main() {
    let opponent_moves = ["A", "B", "C"];
    let mut points = 0;
    for line in fs::read_to_string("strat.txt").unwrap().lines() {
        let game = line.split_whitespace().collect::<Vec<&str>>();
        let opponent_move = game.first().expect("my move does not exist!");
        let game_result = game.get(1).expect("enemy move does not exist!").clone();
        let opponent_move_idx = opponent_moves
            .iter()
            .position(|m| m == opponent_move)
            .expect("illegal move");
        match game_result {
            "X" => {
                // we need to lose
                if opponent_move_idx == 0 {
                    // opponent chose rock, we need to get secissors
                    points += 3;
                } else {
                    points += opponent_move_idx - 1 + 1;
                }
            }
            "Y" => {
                // need to draw
                points += 3 + opponent_move_idx + 1;
            }
            "Z" => {
                // need to win
                points += 6;
                if opponent_move_idx == 2 {
                    points += 1;
                } else {
                    points += opponent_move_idx + 1 + 1;
                }
            }
            _ => panic!(),
        }
        //
        // match my_move_idx.cmp(&opponent_move_idx) {
        //     Ordering::Equal => {
        //         // equal moves is a draw
        //         println!("{} vs {}. draw", my_move, opponent_move);
        //         points = points + 3;
        //     }
        //     Ordering::Less => {
        //         // either A Y , B Z or A Z
        //         if my_move_idx == 0 {
        //             match opponent_move_idx {
        //                 1 => {
        //                     println!("{} vs {}. lose", my_move, opponent_move);
        //                 }
        //                 // lose
        //                 2 => {
        //                     points = points + 6;
        //                     println!("{} vs {}. win", my_move, opponent_move);
        //                 }
        //                 // win
        //                 _ => panic!("Soomething's really wrong."),
        //             };
        //         } else {
        //             // else move is 1 and opponent is 2 , we lose
        //             println!("{} vs {}. lose", my_move, opponent_move);
        //         }
        //     }
        //     Ordering::Greater => {
        //         // either B X, C Y or C X
        //         if my_move_idx == 2 {
        //             match opponent_move_idx {
        //                 0 => {
        //                     println!("{} vs {}. lose", my_move, opponent_move);
        //                 }
        //                 1 => {
        //                     points = points + 6;
        //                     println!("{} vs {}. win", my_move, opponent_move);
        //                 }
        //                 _ => panic!("again, something is fishy"),
        //             }
        //         } else {
        //             println!("{} vs {}. win", my_move, opponent_move);
        //             points = points + 6
        //         }
        //     }
        // }
        println!("{points}");
    }
}
