use instructions::abstractions::{
    managers_only, office_workers_only, open_office_workers_only, Manager, ManagerImpl,
    OfficeWorker, OpenOfficeWorker,
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
