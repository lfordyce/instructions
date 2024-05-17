use core::cell::RefCell;

#[derive(Default)]
struct Health(f32);

struct Entity {
    health: RefCell<Health>,
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            health: RefCell::new(Health::default()),
        }
    }
}

trait RetrieveComponent<T: Sized> {
    fn retrieve(&self) -> &RefCell<T>;
}

impl RetrieveComponent<Health> for Entity {
    fn retrieve(&self) -> &RefCell<Health> {
        &self.health
    }
}

trait SystemArgument: Sized {
    fn retrieve_component(entity: &Entity) -> &RefCell<Self>
    where
        Entity: RetrieveComponent<Self>,
    {
        entity.retrieve()
    }
}

impl<T> SystemArgument for T {}

trait System {
    fn run(&self, entity: &Entity);
}

struct Callback<T> {
    callback: T,
}

impl<T> System for Callback<Box<dyn for<'a> Fn(&'a mut T)>>
where
    T: SystemArgument,
    Entity: RetrieveComponent<T>,
{
    fn run(&self, entity: &Entity) {
        let cell = <T as SystemArgument>::retrieve_component(entity);
        let mut value = cell.borrow_mut();
        (self.callback)(&mut *value);
    }
}

impl<T> System for Callback<Box<dyn for<'a> Fn(&'a T)>>
where
    T: SystemArgument,
    Entity: RetrieveComponent<T>,
{
    fn run(&self, entity: &Entity) {
        let cell = <T as SystemArgument>::retrieve_component(entity);
        let value = cell.borrow();
        (self.callback)(&*value);
    }
}

fn increase_health(health: &mut Health) {
    health.0 += 1.0;
}

fn print_health(health: &Health) {
    println!("print_health, health: {}", health.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alt_system_health() {
        let entity = Entity::default();

        let mut systems: Vec<Box<dyn System>> = Vec::new();

        let cb: Box<dyn for<'a> Fn(&'a mut Health)> = Box::new(increase_health);
        systems.push(Box::new(Callback { callback: cb }));

        let cb: Box<dyn for<'a> Fn(&'a Health)> = Box::new(print_health);
        systems.push(Box::new(Callback { callback: cb }));

        for sys in &systems {
            sys.run(&entity);
        }
    }
}
