// Less-than finite domain constraint
use crate::engine::Engine;
use crate::goal::Goal;
use crate::lterm::LTerm;
use crate::relation::diseqfd::diseqfd;
use crate::relation::ltefd::ltefd;
use crate::user::User;

pub fn ltfd<U, E>(u: LTerm<U>, v: LTerm<U>) -> Goal<U, E>
where
    U: User,
    E: Engine<U>,
{
    proto_vulcan!([diseqfd(u, v), ltefd(u, v)])
}

#[cfg(test)]
mod tests {
    use super::ltfd;
    use crate::prelude::*;
    use crate::relation::diseqfd::diseqfd;
    use crate::relation::infd::infd;

    #[test]
    fn test_ltfd_1() {
        let query = proto_vulcan_query!(|q| {
            |x, y| {
                q == [x, y],
                infd(x, #&[1, 2, 3]),
                infd(y, #&[0, 1, 2, 3, 4]),
                ltfd(x, y),
            }
        });
        let mut iter = query.run();
        assert_eq!(iter.next().unwrap().q, lterm!([1, 2]));
        assert_eq!(iter.next().unwrap().q, lterm!([1, 3]));
        assert_eq!(iter.next().unwrap().q, lterm!([1, 4]));
        assert_eq!(iter.next().unwrap().q, lterm!([3, 4]));
        assert_eq!(iter.next().unwrap().q, lterm!([2, 3]));
        assert_eq!(iter.next().unwrap().q, lterm!([2, 4]));
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_ltfd_2() {
        let query = proto_vulcan_query!(|q| {
            |x, y| {
                q == [x, y],
                infd(x, #&[1, 2, 3]),
                infd(y, #&[0, 1, 2, 3, 4]),
                ltfd(x, y),
                diseqfd(x, 1),
                y == 3,
            }
        });
        let mut iter = query.run();
        assert_eq!(iter.next().unwrap().q, lterm!([2, 3]));
        assert!(iter.next().is_none());
    }
}
