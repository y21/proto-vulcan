use crate::engine::Engine;
use crate::goal::{AnyGoal, DFSGoal, Goal, GoalCast, InferredGoal};
use crate::lterm::LTerm;
use crate::solver::{Solve, Solver};
use crate::state::State;
use crate::stream::Stream;
use crate::user::User;
use std::any::Any;

#[derive(Derivative)]
#[derivative(Debug(bound = "U: User"))]
pub struct Fresh<U, E, G>
where
    U: User,
    E: Engine<U>,
    G: AnyGoal<U, E>,
{
    variables: Vec<LTerm<U, E>>,
    body: InferredGoal<U, E, G>,
}

impl<U, E, G> Fresh<U, E, G>
where
    U: User,
    E: Engine<U>,
    G: AnyGoal<U, E>,
{
    pub fn new(variables: Vec<LTerm<U, E>>, body: InferredGoal<U, E, G>) -> InferredGoal<U, E, G> {
        InferredGoal::dynamic(Fresh { variables, body })
    }

    pub fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<U, E, G> Solve<U, E> for Fresh<U, E, G>
where
    U: User,
    E: Engine<U>,
    G: AnyGoal<U, E>,
{
    fn solve(&self, _solver: &Solver<U, E>, state: State<U, E>) -> Stream<U, E> {
        if let Some(bfs) = self.as_any().downcast_ref::<Fresh<U, E, Goal<U, E>>>() {
            Stream::pause(Box::new(state), bfs.body.clone().cast_into())
        } else if let Some(dfs) = self.as_any().downcast_ref::<Fresh<U, E, DFSGoal<U, E>>>() {
            Stream::pause_dfs(Box::new(state), dfs.body.clone().cast_into())
        } else {
            unreachable!()
        }
    }
}
