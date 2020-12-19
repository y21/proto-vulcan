use crate::goal::Goal;
use crate::user::User;

/// A relation that succeeds.
///
/// Proto-vulcan provides a built-in syntax `true` to avoid the use-clause.
///
/// ```rust
/// extern crate proto_vulcan;
/// use proto_vulcan::*;
/// fn main() {
///     let query = proto_vulcan_query!(|q| {
///         conde {
///             [true, q == 1],
///             [false, q == 2],
///         }
///     });
///     let mut iter = query.run();
///     assert!(iter.next().unwrap().q == 1);
///     assert!(iter.next().is_none());
/// }
/// ```
pub fn succeed<U: User>() -> Goal<U> {
    Goal::succeed()
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_succeed() {
        let query = proto_vulcan_query!(|q| {true});
        let mut iter = query.run();
        assert!(iter.next().unwrap().q.is_any());
        assert!(iter.next().is_none());
    }
}
