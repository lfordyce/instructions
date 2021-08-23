use derive_more::From;
use num_traits::{identities::Zero, PrimInt};
use std::io;
use std::ops::{Add, Div, Mul, Rem};
use thiserror::Error;

fn syracuse<T>(n: T) -> T
where
    T: Copy
        + Eq
        + Add<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + From<u32>,
{
    let zero: T = 0.into();
    match n % 2.into() == zero {
        true => n / (2.into()),
        false => n * (3.into()) + 1.into(),
    }
}

pub trait Fizzy {
    fn fizzy(&self) -> String;
}

impl<T> Fizzy for T
where
    T: PrimInt + std::fmt::Display,
{
    fn fizzy(&self) -> String {
        let zero = T::zero();
        let three = T::from(3).unwrap();
        let five = T::from(5).unwrap();
        match (*self % three, *self % five) {
            (x, y) if x == zero && y == zero => String::from("FizzBuzz"),
            (x, _) if x == zero => String::from("Fizz"),
            (_, y) if y == zero => String::from("Buzz"),
            _ => format!("{}", self),
        }
    }
}

trait Shared {
    fn f(&self) -> usize;
}

struct A;
impl Shared for A {
    fn f(&self) -> usize {
        1
    }
}

struct B;
impl Shared for B {
    fn f(&self) -> usize {
        2
    }
}

enum E {
    VarA(A),
    VarB(B),
}

impl Shared for E {
    fn f(&self) -> usize {
        match self {
            E::VarA(x) => x.f(),
            E::VarB(x) => x.f(),
        }
    }
}

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("An I/O error: {0}")]
    Io(#[source] io::Error),
}

/// A type returned from a FSM (sub)transition function.
pub type TransitionOut<D, E = crate::RequestError> = Result<DialogueStage<D>, E>;

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum DialogueStage<D> {
    Next(D),
    Exit,
}

pub fn next<Dialogue, State, E>(new_state: State) -> TransitionOut<Dialogue, E>
where
    Dialogue: From<State>,
{
    Ok(DialogueStage::Next(Dialogue::from(new_state)))
}

#[derive(Clone, Debug)]
pub struct StartState;

#[derive(Clone, Debug)]
pub struct HaveNumberState {
    pub number: i32,
}

impl From<HaveNumberState> for Dialogue {
    fn from(a: HaveNumberState) -> Self {
        Self::HaveNumber(a)
    }
}

impl From<StartState> for Dialogue {
    fn from(s: StartState) -> Self {
        Self::Start(s)
    }
}

// #[derive(Clone, From, Debug)]
#[derive(Clone, Debug)]
pub enum Dialogue {
    Start(StartState),
    HaveNumber(HaveNumberState),
}

impl Default for Dialogue {
    fn default() -> Self {
        Self::Start(StartState)
    }
}

// impl From<Dialogue> for Dialogue {
//     fn from(d: Dialogue) -> Self {
//         match d {
//             Dialogue::Start(s) => Self::Start(s),
//             Dialogue::ReceiveFullName(r) => Self::ReceiveFullName(r),
//         }
//     }
// }

#[test]
fn syracuse_test() {
    let input: Vec<(u128, u128)> = vec![(4, 2), (71, 214)];
    input
        .into_iter()
        .for_each(|(input, output)| assert_eq!(syracuse(input), output));
}

#[test]
fn fizzy_test() {
    for x in 1..=100 {
        println!("{}", x.fizzy())
    }
}

#[test]
fn test_trait_enum() {
    let val = E::VarA(A);

    println!("{}", val.f());
}

#[test]
fn test_transition() {
    // let dialoge = Dialogue;

    let a = HaveNumberState { number: 35 };
    let dialogue = Dialogue::from(a);
    println!("{:?}", dialogue);

    // let result: TransitionOut<Dialogue> = next(HaveNumberState { number: 35 });
    let result: TransitionOut<Dialogue> = next(Dialogue::default());
    println!("{:?}", result);
}
