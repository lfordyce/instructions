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
