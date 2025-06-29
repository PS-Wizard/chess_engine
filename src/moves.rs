#![allow(dead_code)]
pub fn get_white_pawn_move(from: usize) -> (Option<usize>, Option<usize>) {
    if from / 8 == 7 {
        // pawn on last rank, no forward moves
        return (None, None);
    }

    let one_step = from + 8;

    let two_steps = if from / 8 == 1 { Some(from + 16) } else { None };

    (Some(one_step), two_steps)
}

pub fn get_black_pawn_move(from: usize) -> (usize, Option<usize>) {
    println!("Black pawn at square {}", from);

    let one_step = from + 8;
    let two_steps = if from / 8 == 1 { Some(from + 16) } else { None };

    (one_step, two_steps)
}
