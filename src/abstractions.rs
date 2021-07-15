use std::marker::PhantomData;

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
