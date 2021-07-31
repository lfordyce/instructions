/// A result which may outlive its creator.
trait ResultTrait {}

impl ResultTrait for () {}

trait FactoryTrait {
    type Result: ResultTrait;
    fn get(self) -> Self::Result;
}

// only ever used with _Constraint == &'a Self, gives
// rise to an implicit Self: 'a constraint which
// restricts the `for<'a> FactoryCreatorHasFactory<'a, Self::Result>`
// supertrait of `FactoryCreator` to only range about `'a` that fulfill `Self: 'a`.
trait FactoryCreatorHasFactory<'a, Result, _Constraint = &'a Self> {
    type Factory: FactoryTrait<Result = Result>;
}
type FactoryCreatorFactory<'a, Self_> =
    <Self_ as FactoryCreatorHasFactory<'a, <Self_ as FactoryCreator>::Result>>::Factory;

trait FactoryCreator: Default + for<'a> FactoryCreatorHasFactory<'a, Self::Result> {
    type Result: ResultTrait;
    fn create(&self) -> FactoryCreatorFactory<'_, Self>;
}

#[derive(Default)]
struct StructA {}

impl<'a> FactoryCreatorHasFactory<'a, ()> for StructA {
    type Factory = FactoryA<'a>;
}
impl FactoryCreator for StructA {
    type Result = ();
    fn create(&self) -> FactoryA<'_> {
        FactoryA(self)
    }
}

struct FactoryA<'a>(&'a StructA);

impl<'a> FactoryTrait for FactoryA<'a> {
    type Result = ();
    fn get(self) -> Self::Result {
        ()
    }
}

fn spawn<T: FactoryCreator>() -> T::Result {
    let factory_creator = T::default();
    let factory = factory_creator.create();
    factory.get()
}

trait Phone<'a> {
    fn call(&self);
}

struct IPhone<'a> {
    my_str: &'a str,
}

impl<'a> Phone<'a> for IPhone<'a> {
    fn call(&self) {
        println!("IPhone id: {}", self.my_str);
    }
}

trait Factory<'a> {
    type Output: Phone<'a>;
    fn new_phone(&self, ms: &'a str) -> Self::Output;
}

struct IPhoneFactory;

impl<'a> Factory<'a> for IPhoneFactory {
    type Output = IPhone<'a>;
    fn new_phone(&self, ms: &'a str) -> IPhone<'a> {
        IPhone { my_str: ms }
    }
}

fn call_phone<F: for<'a> Factory<'a>>(f: F) {
    for i in 0..10 {
        let s = i.to_string();
        let p = f.new_phone(&s);
        p.call();
    }
}

// fn call_phone<'a, P: Phone<'a>, F: Factory<'a>>(f: F, s: &'a str) {
//     for _ in 0..10 {
//         let p = f.new_phone(s);
//         p.call();
//     }
// }

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_factory() {
        call_phone(IPhoneFactory);
    }
}
