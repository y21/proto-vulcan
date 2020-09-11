use crate::goal::Goal;
use crate::lterm::LTerm;
use crate::relation::conso;
use crate::state::UserState;
use std::rc::Rc;

/// A relation such that `rest` is `list` without its first element.
///
/// # Example
/// ```rust
/// # #![recursion_limit = "512"]
/// use proto_vulcan::*;
/// use proto_vulcan::relation::resto;
/// let query = proto_vulcan_query!(|q| {
///     resto([1, 2, 3], q)
/// });
/// assert!(query.run().next().unwrap().q == lterm!([2, 3]));
/// ```
pub fn resto<U: UserState>(list: &Rc<LTerm>, rest: &Rc<LTerm>) -> Rc<dyn Goal<U>> {
    proto_vulcan!(|first| { conso(first, rest, list) })
}

#[cfg(test)]
mod test {
    use super::resto;
    use crate::*;

    #[test]
    fn test_resto_1() {
        let query = proto_vulcan_query!(|q| { resto([1], q) });
        assert!(query.run().next().unwrap().q == lterm!([]));
    }

    #[test]
    fn test_resto_2() {
        let query = proto_vulcan_query!(|q| { resto([1, 2], q) });
        assert!(query.run().next().unwrap().q == lterm!([2]));
    }

    #[test]
    fn test_resto_3() {
        let query = proto_vulcan_query!(|q| { resto([1, 2, 3], q) });
        assert!(query.run().next().unwrap().q == lterm!([2, 3]));
    }

    #[test]
    fn test_resto_4() {
        let query = proto_vulcan_query!(|q| { resto([1, [2, 3]], q) });
        assert!(query.run().next().unwrap().q == lterm!([[2, 3]]));
    }
}
