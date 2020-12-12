use crate::goal::{Goal, Solve};
use crate::operator::FnOperatorParam;
use crate::state::State;
use crate::stream::Stream;
use crate::user::User;
use std::fmt;

pub struct FnGoal<U: User> {
    f: Box<dyn Fn(State<U>) -> Stream<U>>,
}

impl<U: User> FnGoal<U> {
    pub fn new(f: Box<dyn Fn(State<U>) -> Stream<U>>) -> Goal<U> {
        Goal::new(FnGoal { f })
    }
}

impl<U: User> Solve<U> for FnGoal<U> {
    fn solve(&self, state: State<U>) -> Stream<U> {
        (*self.f)(state)
    }
}

impl<U: User> fmt::Debug for FnGoal<U> {
    fn fmt(&self, fm: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fm, "FnGoal()")
    }
}

pub fn fngoal<U: User>(param: FnOperatorParam<U>) -> Goal<U> {
    FnGoal::new(param.f)
}
