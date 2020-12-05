extern crate proto_vulcan;
use proto_vulcan::proto_vulcan;
use proto_vulcan::relation::diseqfd;
use proto_vulcan::relation::distinctfd;
use proto_vulcan::relation::infdrange;
use proto_vulcan::relation::plusfd;
use proto_vulcan::*;
use std::ops::RangeInclusive;

fn diago<U: UserState>(qi: LTerm, qj: LTerm, d: LTerm, range: &RangeInclusive<isize>) -> Goal<U> {
    proto_vulcan!(
        |qi_plus_d, qj_plus_d| {
            infdrange([qi_plus_d, qj_plus_d], #range),
            plusfd(qi, d, qi_plus_d),
            diseqfd(qi_plus_d, qj),
            plusfd(qj, d, qj_plus_d),
            diseqfd(qj_plus_d, qi)
        }
    )
}

fn diagonalso<U: UserState>(n: isize, i: isize, j: isize, s: LTerm, r: LTerm) -> Goal<U> {
    proto_vulcan_closure!(
        match r {
            [] | [_] => ,
            [_, second | rest] => {
                s == [],
                diagonalso(#n, #i + 1, #i + 2, rest, [second | rest]),
            },
            [qi | _] => {
                |qj, tail| {
                    s == [qj | tail],
                    diago(qi, qj, (j - i), #&(0..=2 * n)),
                    diagonalso(#n, #i, #j + 1, tail, r),
                }
            }
        }
    )
}

fn nqueenso<U: UserState>(queens: LTerm, n: isize, i: isize, l: LTerm) -> Goal<U> {
    if i == 0 {
        proto_vulcan!(|ltail| {
            l == [_ | ltail],
            [distinctfd(l), diagonalso(#n, #0, #1, ltail, l), queens == l]
        })
    } else {
        proto_vulcan_closure!(|x| {
            infdrange(x, #&(1..=n)),
            nqueenso(queens, #n, #i - 1, [x | l])
        })
    }
}

fn main() {
    let n = 8;
    let query = proto_vulcan_query!(|queens| {
        nqueenso(queens, #n, #n, [])
    });

    for (i, result) in query.run().enumerate() {
        println!("{}: {}", i, result.queens);
    }
}
