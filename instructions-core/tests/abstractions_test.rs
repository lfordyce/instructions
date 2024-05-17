use instructions_core::abstractions::{
    managers_only, office_workers_only, open_office_workers_only, test_visitor, CarFactory,
    InMemStorage, LoggingProducerTrait, Manager, ManagerImpl, OfficeWorker, OpenOfficeWorker,
    ProducerTrait, ProductTrait, Storage, StringVisitor,
};
use std::sync::Arc;
use std::{fmt::Debug, marker::PhantomData};

#[test]
fn test_office_work() {
    // Implementation management
    let manager: Manager = ManagerImpl::new(13);

    // Levels of indirection
    office_workers_only(&manager);
    open_office_workers_only(&manager);
    managers_only(&manager);

    // Incapsulation
    let office_worker: Box<dyn OfficeWorker> = Box::new(manager);
    office_workers_only(office_worker.as_ref());
    //open_office_workers_only(office_worker.as_ref()); // sry bro
    //managers_only(office_worker.as_ref()); // sry bro
}

#[test]
fn test_car_factory() {
    let producer = CarFactory {};
    let product = producer.produce("Herby");
    let product = producer.produce_and_log("Herby");
    println!("Product name: {}", product.name());
}

#[tokio::test]
async fn test_visitor_pattern() {
    let f = test_visitor(true);
    let sv = StringVisitor {
        data: "HELLO".to_string(),
        storage: InMemStorage::new(),
        // _phantom: PhantomData,
    };
    // println!("{:?}", f.accept(sv));
    f.accept(&sv);

    // let store = Arc::clone(sv.storage);

    let result = sv.storage.fetch_update(5).await.unwrap();
    println!("{:?}", result);

    // println!(
    //     "{:?}",
    //     f.accept(&StringVisitor {
    //         data: "HELLO".to_string(),
    //         storage: InMemStorage::new(),
    //         // _phantom: PhantomData,
    //     })
    // );
}

pub trait Property<'a> {
    fn property() -> &'a str;
    fn some_behavior_using_prop(&self) {
        let property1 = Self::property();
        println!("{}", property1);
    }
}

pub struct SomeElement {
    pub hostname: String,
    pub message_type: i32,
}

impl<'a> Property<'a> for SomeElement {
    fn property() -> &'a str {
        "my_data_type"
    }
}
