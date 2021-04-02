use crate::engine::{DefaultEngine, Engine};
use crate::goal::Goal;
use crate::lresult::LResult;
use crate::lterm::LTerm;
use crate::state::State;
use crate::user::{EmptyUser, User};
use std::iter::FusedIterator;
use std::marker::PhantomData;
use std::rc::Rc;

pub trait QueryResult<U = EmptyUser>
where
    U: User,
{
    fn from_vec(v: Vec<LResult<U>>) -> Self;
}

pub struct ResultIterator<R, U = EmptyUser, E = DefaultEngine<U>>
where
    R: QueryResult<U>,
    U: User,
    E: Engine<U>,
{
    engine: E,
    variables: Vec<LTerm<U>>,
    stream: E::Stream,
    _phantom: PhantomData<R>,
}

#[doc(hidden)]
impl<R, U, E> ResultIterator<R, U, E>
where
    R: QueryResult<U>,
    U: User,
    E: Engine<U>,
{
    pub fn new(
        engine: E,
        variables: Vec<LTerm<U>>,
        goal: Goal<U, E>,
        initial_state: State<U>,
    ) -> ResultIterator<R, U, E> {
        let stream = goal.solve(&engine, initial_state);
        ResultIterator {
            engine,
            variables,
            stream,
            _phantom: PhantomData,
        }
    }
}

#[doc(hidden)]
impl<R, U, E> Iterator for ResultIterator<R, U, E>
where
    R: QueryResult<U>,
    U: User,
    E: Engine<U>,
{
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        match self.engine.next(&mut self.stream) {
            Some(state) => {
                // At this point the state has already gone through initial reification
                // process
                let smap = state.smap_ref();
                let purified_cstore = state.cstore_ref().clone().purify(smap).normalize();
                let reified_cstore = Rc::new(purified_cstore.walk_star(smap));
                let results = self
                    .variables
                    .iter()
                    .map(|v| LResult(state.smap_ref().walk_star(v), Rc::clone(&reified_cstore)))
                    .collect();
                Some(R::from_vec(results))
            }
            None => None,
        }
    }
}

/* ResultIterator is fused because uncons() will always keep returning None on empty stream */
#[doc(hidden)]
impl<R, U, E> FusedIterator for ResultIterator<R, U, E>
where
    R: QueryResult<U>,
    U: User,
    E: Engine<U>,
{
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Query<R, U = EmptyUser, E = DefaultEngine<U>>
where
    R: QueryResult<U>,
    U: User,
    E: Engine<U>,
{
    variables: Vec<LTerm<U>>,
    goal: Goal<U, E>,
    _phantom: std::marker::PhantomData<R>,
}

impl<R, E> Query<R, EmptyUser, E>
where
    R: QueryResult<EmptyUser>,
    E: Engine<EmptyUser>,
{
    pub fn run(&self) -> ResultIterator<R, EmptyUser, E> {
        let user_state = EmptyUser::new();
        self.run_with_user(user_state)
    }
}

impl<R, U, E> Query<R, U, E>
where
    R: QueryResult<U>,
    U: User,
    E: Engine<U>,
{
    pub fn new(variables: Vec<LTerm<U>>, goal: Goal<U, E>) -> Query<R, U, E> {
        Query {
            variables,
            goal,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn run_with_user(&self, user_state: U) -> ResultIterator<R, U, E> {
        let initial_state = State::new(user_state);
        let engine = E::new();
        ResultIterator::new(
            engine,
            self.variables.clone(),
            self.goal.clone(),
            initial_state,
        )
    }
}

#[macro_export]
macro_rules! proto_vulcan_query {
    (| $($query:ident),+ | { $( $body:tt )* } ) => {{
        // Declare the query variables visible in the query namespace.
        $(let $query = LTerm::var(stringify!($query));)+

        // Collect query variables to a vector as well (to be used later).
        let __vars__ = vec![ $( $query.clone() ),+ ];

        use $crate::state::reify;
        let goal = proto_vulcan!(|__query__| {
            __query__ == [$($query),+],
            [ $( $body )* ],
            reify(__query__)
        });

        use $crate::user::User;
        use std::fmt;
        use $crate::lresult::LResult;
        use $crate::lterm::LTerm;
        use $crate::query::QueryResult;

        // Each query has a custom result struct type with fields named
        // according to the query variable as in the operator |a, b, c| {}
        #[derive(Clone, Debug)]
        struct QResult<U: User> {
            $( $query: LResult<U>, )+
        }

        impl<U: User> QueryResult<U> for QResult<U> {
            fn from_vec(v: Vec<LResult<U>>) -> QResult<U> {
                let mut vi = v.into_iter();
                QResult {
                    $( $query: vi.next().unwrap(), )+
                }
            }
        }

        impl<U: User> fmt::Display for QResult<U> {
            #[allow(unused_variables, unused_assignments)]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let mut count = 0;
                $( if count > 0 { writeln!(f, "")?; }  write!(f, "{}: {}", stringify!($query), self.$query)?; count += 1; )+
                write!(f, "")
            }
        }

        $crate::query::Query::<QResult<_>>::new(__vars__, goal)
    }};
}
