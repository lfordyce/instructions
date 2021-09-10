use crate::BoolTrait;
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
    fn run(&self, e: &dyn HasData<Data = D>);
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
    fn run(&self, item: &dyn HasData<Data = ContextData>) {
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
    pub fn iter_as<'a, T>(&'a self) -> impl Iterator<Item = T> + 'a
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
