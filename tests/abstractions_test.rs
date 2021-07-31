use instructions::abstractions::{
    managers_only, office_workers_only, open_office_workers_only, CarFactory, LoggingProducerTrait,
    Manager, ManagerImpl, OfficeWorker, OpenOfficeWorker, ProducerTrait, ProductTrait,
};

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
