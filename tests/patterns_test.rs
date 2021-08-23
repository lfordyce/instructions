use instructions::patterns;
use instructions::patterns::{my_take, Context, ContextData, ContextStrategy, SomeTrait, Strategy};

#[test]
pub fn test_strategy() {
    let context = Context {
        s: ContextStrategy,
        data: ContextData::default(),
    };
    context.do_it();
}

#[test]
pub fn some_trait_take_test() {
    let mut obj: Box<dyn SomeTrait> = Box::new(42);
    assert_eq!(obj.some_method(), "42");
    drop(my_take(&mut *obj));
    assert_eq!(obj.some_method(), "0");
    obj = Box::new(());
    assert_eq!(obj.some_method(), "I'm the One!");
}
