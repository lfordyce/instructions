use std::borrow::Cow;
use crate::BoolTrait;
use serde::Deserialize;
use std::collections::btree_set::{BTreeSet, IntoIter};
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::FromIterator;
use std::sync::Arc;

pub struct Context<S, D>
    where
        S: Strategy<D>,
{
    pub s: S,
    pub data: D,
}

pub trait Strategy<D> {
    // fn run(&self, item: &impl HasData<Data = D>);
    fn run(&self, e: &dyn HasData<Data=D>);
}

pub trait HasData {
    type Data;
    fn data(&self) -> &Self::Data;
}

impl<D, S: Strategy<D>> Context<S, D> {
    // the complex code in this impl is the actual meat of the library:
    pub fn do_it(&self) {
        self.s.run(self); // using the Strategy trait
    }
}

impl<D, S: Strategy<D>> HasData for Context<S, D> {
    type Data = D;

    fn data(&self) -> &Self::Data {
        &self.data
    }
}

pub struct ContextStrategy;

#[derive(Debug)]
pub struct ContextData {
    pub field: String,
    pub token: u64,
}

impl Default for ContextData {
    fn default() -> Self {
        Self {
            field: "context".to_string(),
            token: u64::MAX,
        }
    }
}

impl Strategy<ContextData> for ContextStrategy {
    fn run(&self, item: &dyn HasData<Data=ContextData>) {
        let d = item.data();
        // use ContextData
        println!("{:?}", d);
    }
}

// ========
pub trait TraitTake {
    fn take<'slf>(self: &'_ mut Self) -> Box<dyn 'slf + SomeTrait>
        where
            Self: 'slf;
}

impl<T: Default> TraitTake for T
    where
        T: SomeTrait, // if we impl the subtrait and Default, then we have `.take()`
{
    fn take<'slf>(self: &'_ mut Self) -> Box<dyn 'slf + SomeTrait>
        where
            Self: 'slf,
    {
        Box::new(::core::mem::take(self))
    }
}

pub trait SomeTrait: TraitTake {
    fn some_method(self: &'_ Self) -> String;
}

/// Still works.
pub fn my_take<'lt>(obj: &'_ mut (dyn 'lt + SomeTrait)) -> Box<dyn 'lt + SomeTrait> {
    obj.take()
}

impl SomeTrait for i32 {
    fn some_method(self: &'_ Self) -> String {
        self.to_string()
    }
}

impl SomeTrait for () {
    fn some_method(self: &'_ Self) -> String {
        "I'm the One!".into()
    }
}

pub trait DecodedPacket<'a> {}

pub struct ReferencePacket<'a> {
    data: &'a [u8],
}

impl<'a: 'b, 'b> DecodedPacket<'b> for ReferencePacket<'a> {}

pub struct Decoder {}

impl Decoder {
    pub fn receiver<'a>(&self, on_packet: Arc<dyn Fn(&Box<dyn DecodedPacket<'a>>)>) {
        let slice: &[u8] = &[0, 1, 2];
        let reference_packet: Box<dyn DecodedPacket<'a>> =
            Box::new(ReferencePacket { data: slice });
        on_packet(&reference_packet);
    }
}

pub trait Fnord {
    fn do_it(&self) -> i32;
}

impl Fnord for i32 {
    fn do_it(&self) -> i32 {
        *self
    }
}

impl<'a> From<&'a i32> for &'a dyn Fnord {
    fn from(i: &'a i32) -> Self {
        i as _
    }
}

pub struct YourObject {
    v1: Vec<i32>,
    v2: Vec<i32>,
}

impl YourObject {
    pub fn iter_as<'a, T>(&'a self) -> impl Iterator<Item=T> + 'a
        where
            T: From<&'a i32>,
    {
        self.v1
            .iter()
            .map(|o| o.into())
            .chain(self.v2.iter().map(|o| o.into()))
    }
}

struct FontLoader(String);

struct Font(*const String);

impl FontLoader {
    fn load(&self) -> Font {
        Font(&self.0)
    }
}

struct Window;

struct Phi<'window> {
    window: &'window Window,
    loader: FontLoader,
    font: Option<Font>,
}

impl<'window> Phi<'window> {
    fn do_the_thing(&mut self) {
        let font = self.loader.load();
        self.font = Some(font);
    }
}

#[derive(Debug)]
pub struct TestThingy<'a> {
    label: &'a str,
}

const TEST_VALUES: [TestThingy; 3] = [
    TestThingy { label: "one" },
    TestThingy { label: "two" },
    TestThingy { label: "three" },
];

pub fn value_for_num(num: &str) -> Option<&'static TestThingy<'static>> {
    TEST_VALUES.iter().find(|value| value.label == num)
}

pub fn test_out_thingy() {
    let tmp_val = String::from("two");
    if let Some(test_thingy) = value_for_num(&tmp_val) {
        println!("test_thingy: {:?}", test_thingy);
    }
}

#[derive(Deserialize, Debug)]
struct Analysis<A, S> {
    algorithm: A,
    search_space: S,
}

impl<A, S> Analysis<A, S>
    where
        A: Algorithm,
        S: SearchSpace,
{
    fn solve(&self) -> f64 {
        self.algorithm.evaluate(self.search_space.point())
    }
}

trait Algorithm {
    fn evaluate(&self, point: f64) -> f64;
}

trait SearchSpace {
    fn point(&self) -> f64;
}

#[derive(Deserialize, Debug)]
struct Efficient {
    param: f64,
}

impl Algorithm for Efficient {
    fn evaluate(&self, point: f64) -> f64 {
        self.param * point
    }
}

#[derive(Deserialize, Debug)]
struct SlightlySlower {
    param: f64,
}

impl Algorithm for SlightlySlower {
    fn evaluate(&self, point: f64) -> f64 {
        self.param * 0.5 * point
    }
}

#[derive(Deserialize, Debug)]
struct SquareGrid {
    param: f64,
}

impl SearchSpace for SquareGrid {
    fn point(&self) -> f64 {
        self.param
    }
}

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

impl Default for Person {
    fn default() -> Self {
        Self {
            name: "John".to_string(),
            age: 30,
        }
    }
}

impl From<&str> for Person {
    fn from(s: &str) -> Self {
        if s.is_empty() {
            Self::default()
        } else {
            let i = s.find(':');
            // i is a byte index, not a character index.
            // But we know that the '+1' will work here because the UTF-8
            // representation of ':' is a single byte.
            let r = i.map(|i| (&s[0..i], &s[i + 1..]));
            let mut text = s.splitn(2, ':');
            let (a, b) = (text.next(), text.next());

            let fields: Vec<&str> = s.split(",").collect();
            match (fields.get(0), fields.get(1)) {
                (Some(name), Some(age)) if !name.is_empty() => match age.parse::<usize>() {
                    Err(_) => Person::default(),
                    Ok(age) => Person {
                        name: String::from(*name),
                        age,
                    },
                },
                _ => Self::default(),
            }
        }
    }
}

enum UniquePermutations {
    Leaf {
        elements: Option<Vec<String>>,
    },
    Stem {
        elements: Vec<String>,
        unique_elements: IntoIter<String>,
        first_element: String,
        inner: Box<Self>,
    },
}

impl UniquePermutations {
    fn new(elements: Vec<String>) -> Self {
        if elements.len() == 1 {
            let elements = Some(elements);
            Self::Leaf { elements }
        } else {
            let mut unique_elements = elements
                .clone()
                .into_iter()
                .collect::<BTreeSet<_>>()
                .into_iter();

            let (first_element, inner) = Self::next_level(&mut unique_elements, elements.clone())
                .expect("Must have at least one item");

            Self::Stem {
                elements,
                unique_elements,
                first_element,
                inner,
            }
        }
    }

    fn next_level(
        mut unique_elements: impl Iterator<Item=String>,
        elements: Vec<String>,
    ) -> Option<(String, Box<Self>)> {
        let first_element = unique_elements.next()?;

        let mut remaining_elements = elements;

        if let Some(idx) = remaining_elements
            .iter()
            .position(|i| i.clone() == first_element)
        {
            remaining_elements.remove(idx);
        }

        let inner = Box::new(Self::new(remaining_elements));

        Some((first_element, inner))
    }
}

impl Iterator for UniquePermutations {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Leaf { elements } => elements.take(),
            Self::Stem {
                elements,
                unique_elements,
                first_element,
                inner,
            } => loop {
                match inner.next() {
                    Some(mut v) => {
                        // v.insert(0, *first_element);
                        v.insert(0, first_element.clone());
                        return Some(v);
                    }
                    None => {
                        let (next_fe, next_i) =
                            Self::next_level(&mut *unique_elements, elements.clone())?;
                        *first_element = next_fe;
                        *inner = next_i;
                    }
                }
            },
        }
    }
}

fn transpose<const R: usize, const C: usize, T: Copy + Default>(m: [[T; C]; R]) -> [[T; R]; C] {
    let mut result: [[T; R]; C] = [[Default::default(); R]; C];
    for i in 0..R {
        for j in 0..C {
            result[j][i] = m[i][j];
        }
    }
    result
}

fn f<'a>(s: impl Into<Cow<'a, str>>) -> Cow<'a, str> {
    let mut s: Cow<str> = s.into();

    if s.is_empty() {
        s.to_mut().push_str("empty")
    }
    s
}

pub fn abstract_iterator<I, O>(input: I) -> O
    where
        I: IntoIterator,
        I::Item: Hash + Eq,
        O: FromIterator<usize>,
{
    let mut dict = HashMap::new();
    input.into_iter().map(move |elem| {
        let id = dict.len();
        *dict.entry(elem).or_insert(id)
    }).collect()
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::str::Chars;
    use rand::{Rng, thread_rng};
    use super::*;

    #[test]
    fn test_permutations() {
        let items = vec![
            "BTC".to_string(),
            "ETH".to_string(),
            "DOT".to_string(),
            "USD".to_string(),
            "USDC".to_string(),
        ];
        for perm in UniquePermutations::new(items) {
            println!("{:?}", perm);
        }
    }

    #[test]
    fn test_function_parameterized_trait() {
        let obj = YourObject {
            v1: vec![1, 2],
            v2: vec![3],
        };

        for s in obj.iter_as::<&dyn Fnord>() {
            println!("{}", s.do_it()); // 1 2 3
        }
    }

    #[test]
    fn test_window_load() {
        let window = Window;
        let mut p1 = Phi {
            window: &window,
            loader: FontLoader(String::from("example")),
            font: None,
        };
        p1.do_the_thing();

        eprintln!("{:p} =?= {:p}", &p1.loader.0, p1.font.as_ref().unwrap().0);

        let p2 = p1;
        eprintln!("{:p} =?= {:p}", &p2.loader.0, p2.font.as_ref().unwrap().0);
    }

    #[test]
    fn test_lifetime_thingy() {
        test_out_thingy();
    }

    #[test]
    fn test_polymorphic_generics() {
        // This is how I kinda want my configurator to be parsed (or something similar)
        // let input = r#"
        // {
        //     "algorithm": {"Efficient": { "param": 0.1}},
        //     "search_space": {"SquareGrid": {"param": 0.1}}
        // }
        // "#;

        // This works, but I'd like to specify the variants of algorithm and search space
        let input = r#"
        {
            "algorithm": {"param": 0.5},
            "search_space": {"param": 0.5}
        }
        "#;

        // This lets me parse a monomorphisized analysis struct..
        let result: Analysis<SlightlySlower, SquareGrid> = serde_json::from_str(input).unwrap();

        // But how to gracefully dispatch different variants?
        println!("Result of complex analysis was: {}", result.solve());
    }

    #[test]
    fn test_more_matching_patterns() {
        let idx = 6;
        let num = 15;
        let res = match idx {
            n if (1..=num - 5).contains(&n) => Some("match!"),
            _ => None,
        };
        println!("{:?}", res);
    }

    #[test]
    fn test_start_end_iteration() {
        let v = vec![1, 2, 3, 4, 5];

        // With iterators
        // let (f, b) = v.split_at(v.len() / 2);
        // for (x, y) in f.iter().zip(b.iter().rev()) {
        //     println!("{}, {}", x, y)
        // }

        // with pattern matching
        let mut s = &v[..];
        loop {
            match s {
                [a, rest @ .., b] => {
                    println!("{}, {}", a, b);
                    s = rest;
                }
                [a] => {
                    println!("{}", a);
                    break;
                }
                [] => break,
            }
        }
    }

    #[test]
    fn test_from_str() {
        let p = Person::from("mark,");
        println!("{:?}", p);
        // assert_eq!(p.name, "mark");
        // assert_eq!(p.age, 30)
    }

    #[test]
    fn test_transpose() {
        let mut rng = thread_rng();
        let mut a = [[0.0f32; 3]; 2];
        for i in 0..2 {
            for j in 0..3 {
                a[i][j] = rng.gen();
            }
        }
        println!("BEFORE: {:?}", a);
        let b = transpose(a);
        println!("AFTER: {:?}", b)
    }

    #[test]
    fn test_abstract_iterator() {
        // assert!(abstract_iterator::<[i32; 3], O>([1,2,1]) == abstract_iterator("aba".chars()))
        assert_eq!(abstract_iterator::<[i32; 3], Vec<usize>>([1, 2, 1]), abstract_iterator::<Chars<'_>, Vec<usize>>("aba".chars()))
    }

    #[test]
    fn test_cow_stuff() {
        println!("{}", f(""));
    }

    #[test]
    fn test_maintain_interior_mutability() {
        let mut numbers: HashMap<usize, Cell<i32>> = vec![0; 100].into_iter().enumerate().map(
            |(i, _)| (i, Cell::new(i as i32))
        ).collect();
        let even_references = numbers.iter().filter(|(_, n)| n.get() & 1 == 0);
        let odd_references = numbers.iter().filter(|(_, n)| n.get() & 1 != 0);

        even_references.for_each(|item| item.1.set(item.1.get() * 2));
        odd_references.for_each(|item| item.1.set(item.1.get() * 2));

        println!("{numbers:?}")
    }
}
