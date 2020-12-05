extern crate proto_vulcan;
use proto_vulcan::relation::distinctfd;
use proto_vulcan::relation::infdrange;
use proto_vulcan::*;

fn main() {
    const BOARD_SIZE: usize = 9;
    const SQUARE_SIZE: usize = 3;

    #[rustfmt::skip]
    let board = lterm!(
        [_, _, _, 2, 6, _, 7, _, 1,
         6, 8, _, _, 7, _, _, 9, _,
         1, 9, _, _, _, 4, 5, _, _,
         8, 2, _, 1, _, _, _, 4, _,
         _, _, 4, 6, _, 2, 9, _, _,
         _, 5, _, _, _, 3, _, 2, 8,
         _, _, 9, 3, _, _, _, 7, 4,
         _, 4, _, _, 5, _, _, 3, 6,
         7, _, 3, _, 1, 8, _, _, _]
    );

    let board_vec: Vec<LTerm> = board.iter().cloned().collect();

    let mut rows = vec![];
    for row_index in 0..BOARD_SIZE {
        let mut row = vec![];
        for col_index in 0..BOARD_SIZE {
            row.push(board_vec[row_index * BOARD_SIZE + col_index].clone());
        }
        rows.push(LTerm::from_vec(row));
    }

    let mut cols = vec![];
    for col_index in 0..BOARD_SIZE {
        let mut col = vec![];
        for row_index in 0..BOARD_SIZE {
            col.push(board_vec[row_index * BOARD_SIZE + col_index].clone());
        }
        cols.push(LTerm::from_vec(col));
    }

    let mut squares = vec![vec![]; 9];
    for row_index in 0..BOARD_SIZE {
        for col_index in 0..BOARD_SIZE {
            let x = board_vec[row_index * BOARD_SIZE + col_index].clone();
            let square_index =
                (row_index / SQUARE_SIZE) * (BOARD_SIZE / SQUARE_SIZE) + (col_index / SQUARE_SIZE);
            squares[square_index].push(x);
        }
    }

    let squares: Vec<LTerm> = squares.into_iter().map(|v| LTerm::from_vec(v)).collect();

    let query = proto_vulcan_query!(|q| {
        q == board,
        for x in &board {
            infdrange(x, #&(1..=9))
        },
        for row in &rows {
            distinctfd(row)
        },
        for col in &cols {
            distinctfd(col)
        },
        for square in &squares {
            distinctfd(square)
        }
    });

    println!("Sudoku query:");
    let mut iter = board.iter();
    for _ in 0..BOARD_SIZE {
        for _ in 0..BOARD_SIZE {
            let t = iter.next().unwrap();
            if t.is_any() {
                print!("_ ");
            } else {
                print!("{} ", t);
            }
        }
        println!("");
    }

    println!("\nSudoku solution:");
    for result in query.run() {
        let mut iter = result.q.iter();
        for _ in 0..BOARD_SIZE {
            for _ in 0..BOARD_SIZE {
                print!("{} ", iter.next().unwrap());
            }
            println!("");
        }
    }
}
