use futures::future::BoxFuture;
use std::fmt::{Debug, Display, Error, Formatter};
use std::marker::PhantomData;
use std::{collections::HashMap, sync::Arc};
use thiserror::Error;
use tokio::sync::Mutex;

// Interface
pub trait OfficeWorker {
    fn id(&self) -> usize;
    fn work_hard(&self);
}

// Abstract class
pub struct OpenOfficeWorker<T: OfficeWorker> {
    inner: T,
}

impl<T: OfficeWorker> OpenOfficeWorker<T> {
    pub fn work_even_harder(&self) {
        self.inner.work_hard();
        println!("I'm unreplaceable. Sorta. ");
    }
}

impl<T: OfficeWorker> AsRef<T> for OpenOfficeWorker<T> {
    fn as_ref(&self) -> &T {
        &self.inner
    }
}

impl<T: OfficeWorker> OfficeWorker for OpenOfficeWorker<T> {
    fn id(&self) -> usize {
        self.inner.id()
    }
    fn work_hard(&self) {
        self.work_even_harder();
    }
}

// End implementation
pub struct ManagerImpl {
    id_: usize,
}

impl ManagerImpl {
    // We create only Managers (decorated OpenOfficeWorkers), not ManagerImpls
    pub fn new(id: usize) -> Manager {
        OpenOfficeWorker {
            inner: ManagerImpl { id_: id },
        }
    }
    pub fn blame(&self) {
        println!(
            "U know, I'm a Bad Luck. Number {}, all that, u know...",
            self.id_
        );
    }
}

impl OfficeWorker for ManagerImpl {
    fn id(&self) -> usize {
        self.id_
    }
    fn work_hard(&self) {
        print!("I work hard, but no one cares. ");
    }
}

// Publicly manageable alias
pub type Manager = OpenOfficeWorker<ManagerImpl>;

pub fn office_workers_only(w: &dyn OfficeWorker) {
    println!(
        "Here enters #{}, but no one has noticed. #{} leaves.\n",
        w.id(),
        w.id()
    );
}

pub fn open_office_workers_only<T: OfficeWorker>(w: &OpenOfficeWorker<T>) {
    print!("- Welcome Mr {}! Say us something!\n- ", w.id());
    w.work_even_harder();
    println!(
        "- Yikes, Mr {}! You make us sad. Go away please.\n ",
        w.id()
    );
}

pub fn managers_only(w: &Manager) {
    print!("- Hey pal {}, how R U!\n- ", w.id());
    w.as_ref().blame();
    println!("- Cool story bro. Grab a beer.\n ");
}

// ===============================
// Preamble: we need some types...
// ===============================

pub trait Number {
    const VALUE: usize;
}

pub struct One;

pub struct PlusOne<T> {
    _marker: PhantomData<T>,
}

impl Number for One {
    const VALUE: usize = 1;
}

impl<T: Number> Number for PlusOne<T> {
    const VALUE: usize = <T as Number>::VALUE + 1;
}

// ======================
// Now for the real thing
// ======================

pub trait AccessorTrait {
    type Output;
    fn get(&self, index: usize) -> Self::Output;
}

pub struct MultiDimensionalTensorAccessor<'a, N: Number, Index> {
    _number: PhantomData<N>,
    strides: &'a [Index],
}

pub struct SingleDimensional {}

impl<'a, N: Number, Index> MultiDimensionalTensorAccessor<'a, N, Index> {
    pub fn new(strides: &'a [Index]) -> Self {
        Self {
            _number: PhantomData,
            strides,
        }
    }
}

impl<'a, N: Number, Index> AccessorTrait for MultiDimensionalTensorAccessor<'a, PlusOne<N>, Index> {
    type Output = MultiDimensionalTensorAccessor<'a, N, Index>;

    fn get(&self, index: usize) -> Self::Output {
        Self::Output::new(&self.strides[1..])
    }
}

impl<'a, Index> AccessorTrait for MultiDimensionalTensorAccessor<'a, One, Index> {
    type Output = SingleDimensional;

    fn get(&self, index: usize) -> Self::Output {
        SingleDimensional {}
    }
}

pub trait ProductTrait {
    fn name(&self) -> &str;
}

pub trait ProducerTrait<'p>: Copy {
    type Product: ProductTrait;
    fn produce(self, name: &str) -> Self::Product;
}

pub trait LoggingProducerTrait<'p>: ProducerTrait<'p>
where
    <Self as ProducerTrait<'p>>::Product: Display,
{
    fn produce_and_log(self, name: &str) -> Self::Product {
        let product = self.produce(name);
        println!("Produced '{}'", product);
        product
    }
}

pub struct CarFactory {}

pub struct Car<'p> {
    producer: &'p CarFactory,
    name: String,
}

impl Display for Car<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Car with name: {}", self.name)
    }
}

impl<'p> ProductTrait for Car<'p> {
    fn name(&self) -> &str {
        self.name.as_ref()
    }
}

impl<'p> ProducerTrait<'p> for &'p CarFactory {
    type Product = Car<'p>;

    fn produce(self, name: &str) -> Self::Product {
        Car {
            producer: self,
            name: String::from(name),
        }
    }
}

impl<'p> LoggingProducerTrait<'p> for &'p CarFactory {}

trait Endpoint: for<'a> DeserializeBody<'a> {}
trait DeserializeBody<'a> {
    type Out: 'a;
    fn deserialize(raw_body: &'a [u8]) -> Self::Out;
}

fn store_ep<'a, EP, F>(func: F)
where
    EP: Endpoint,
    F: 'static + Fn(&'a [u8]) -> <EP as DeserializeBody<'a>>::Out,
{
    let _ = Box::new(func);
    unimplemented!();
}

struct MyEndpoint;
struct MyEndpointBody<'a> {
    pub string: &'a str,
}
impl Endpoint for MyEndpoint {}
impl<'a> DeserializeBody<'a> for MyEndpoint {
    type Out = MyEndpointBody<'a>;
    fn deserialize(raw_body: &'a [u8]) -> Self::Out {
        unimplemented!();
    }
}

struct ColorDisplay<Ifc> {
    ifc: Ifc,
}

struct ColorDisplayDataIter<'a, Ifc> {
    ifc: &'a mut Ifc,
}
impl<'a, Ifc> Iterator for ColorDisplayDataIter<'a, Ifc> {
    type Item = u16;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

trait HasReadDataIter<'a, MutRef = &'a mut Self> {
    type Iter: Iterator<Item = u16>;
}
type ReadDataIter<'a, Self_> = <Self_ as HasReadDataIter<'a>>::Iter;

trait ReadData: for<'a> HasReadDataIter<'a> {
    fn read(&mut self) -> ReadDataIter<'_, Self>;
}

impl<'a, Ifc> HasReadDataIter<'a> for ColorDisplay<Ifc> {
    type Iter = ColorDisplayDataIter<'a, Ifc>;
}

impl<Ifc> ReadData for ColorDisplay<Ifc> {
    fn read(&mut self) -> ReadDataIter<'_, Self> {
        ColorDisplayDataIter { ifc: &mut self.ifc }
    }
}

// =====
pub trait Storage<D> {
    type Error;

    fn record_data(
        self: Arc<Self>,
        id: i64,
        data: D,
    ) -> BoxFuture<'static, Result<(), Self::Error>>
    where
        D: Send + Sync + 'static;

    fn fetch_update(self: Arc<Self>, id: i64)
        -> BoxFuture<'static, Result<Option<D>, Self::Error>>;
}

/// An error returned from [`InMemStorage`].
#[derive(Debug, Error)]
pub enum InMemStorageError {
    /// Returned from [`InMemStorage::remove_dialogue`].
    #[error("row not found")]
    DialogueNotFound,
}

#[derive(Debug)]
pub struct InMemStorage<D> {
    map: Mutex<HashMap<i64, D>>,
}

impl<S> InMemStorage<S> {
    #[must_use]
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            map: Mutex::new(HashMap::new()),
        })
    }
}

impl<D> Storage<D> for InMemStorage<D>
where
    D: Clone,
    D: Send + 'static,
{
    type Error = InMemStorageError;

    fn record_data(self: Arc<Self>, id: i64, data: D) -> BoxFuture<'static, Result<(), Self::Error>>
    where
        D: Send + Sync + 'static,
    {
        Box::pin(async move {
            self.map.lock().await.insert(id, data);
            Ok(())
        })
    }

    fn fetch_update(
        self: Arc<Self>,
        id: i64,
    ) -> BoxFuture<'static, Result<Option<D>, Self::Error>> {
        Box::pin(async move { Ok(self.map.lock().await.get(&id).map(ToOwned::to_owned)) })
    }
}

// ==== Visitor Pattern
pub struct Foo {
    value: i64,
}

pub struct Bar {
    value: i64,
}

pub trait Base<T> {
    fn accept(&self, v: &dyn Visitor<Result = T>) -> T;
}

impl<T> Base<T> for Foo {
    fn accept(&self, v: &dyn Visitor<Result = T>) -> T {
        v.visit_foo(&self)
    }
}

impl<T> Base<T> for Bar {
    fn accept(&self, v: &dyn Visitor<Result = T>) -> T {
        v.visit_bar(&self)
    }
}

pub trait Visitor {
    type Result;
    fn visit_foo(&self, foo: &Foo) -> Self::Result;
    fn visit_bar(&self, bar: &Bar) -> Self::Result;
}

pub struct StringVisitor<S> {
    pub data: String,
    pub storage: Arc<S>,
    // pub _phantom: PhantomData<Mutex<D>>,
}

impl<S> Visitor for StringVisitor<S>
where
    // D: Default + Send + 'static,
    S: Storage<String> + Send + Sync + 'static,
    S::Error: Debug + Send + 'static,
{
    type Result = String;
    fn visit_foo(&self, foo: &Foo) -> String {
        let storage = Arc::clone(&self.storage);
        Box::pin(async move {
            if let Err(err) = storage
                .record_data(foo.value, format!("it was Foo: {:}!", foo.value))
                .await
            {
                println!("failed to write data to storage! {:?}", err);
            }
        });
        format!("it was Foo: {:}!", foo.value)
    }
    fn visit_bar(&self, bar: &Bar) -> String {
        let storage = Arc::clone(&self.storage);
        Box::pin(async move {
            if let Err(err) = storage
                .record_data(bar.value, format!("it was Bar: {:}!", bar.value))
                .await
            {
                println!("failed to write data to storage! {:?}", err);
            }
        });
        format!("it was Bar: {:}!", bar.value)
    }
}

pub fn test_visitor<T>(v: bool) -> Box<dyn Base<T>> {
    if v {
        Box::new(Foo { value: 5 })
    } else {
        Box::new(Bar { value: 10 })
    }
}

pub trait FooBase {}

pub trait AsFoo {
    fn as_foo(&self) -> &dyn FooBase;
}

impl AsFoo for dyn FooBase {
    fn as_foo(&self) -> &dyn FooBase {
        self
    }
}

impl<T: AsFoo + ?Sized> AsFoo for &'_ T {
    fn as_foo(&self) -> &dyn FooBase {
        T::as_foo(*self)
    }
}

impl<T: AsFoo + ?Sized> AsFoo for Box<T> {
    fn as_foo(&self) -> &dyn FooBase {
        T::as_foo(&**self)
    }
}

pub struct MyFooStruct;

impl FooBase for MyFooStruct {}

pub fn accept_foos<T: AsFoo>(foos: Vec<T>) {}

pub fn create_foos() -> Box<dyn FooBase> {
    Box::new(MyFooStruct)
}

// let foos1: Vec<Box<dyn Foo>> = vec![f1];
// let foos2: Vec<&Box<dyn Foo>> = vec![&f2];
