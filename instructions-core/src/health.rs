use std::cell::Ref;
use std::cell::RefCell;

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

trait RetrieveComponent<T> {
    fn retrieve(&self) -> Ref<'_, T>;
}

impl RetrieveComponent<Health> for Entity {
    fn retrieve(&self) -> Ref<Health> {
        self.health.borrow()
    }
}

trait SystemArgument {
    type Component;

    fn retrieve_component(entity: &Entity) -> Ref<'_, Self::Component>
    where
        Entity: RetrieveComponent<Self::Component>;

    fn into_arg_type<'r>(borrowed_ref: &'r Ref<'_, Self::Component>) -> &'r Self;
}

impl<T> SystemArgument for T {
    type Component = T;

    fn retrieve_component(entity: &Entity) -> Ref<'_, Self::Component>
    where
        Entity: RetrieveComponent<Self::Component>,
    {
        entity.retrieve()
    }

    fn into_arg_type<'r>(borrowed_ref: &'r Ref<'_, Self::Component>) -> &'r Self {
        &*borrowed_ref
    }
}

trait System {
    fn run(&self, entity: &Entity);
}

struct Callback<T> {
    callback: Box<dyn Fn(&T)>,
}

impl<T> System for Callback<T>
where
    T: SystemArgument,
    Entity: RetrieveComponent<T::Component>,
{
    fn run(&self, entity: &Entity) {
        let component_ref = T::retrieve_component(entity);
        let component_as_arg = T::into_arg_type(&component_ref);
        (self.callback)(component_as_arg);
    }
}

fn print_health(health: &Health) {
    println!("print_health, health: {}", health.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_health() {
        let entity = Entity::default();

        let mut systems: Vec<Box<dyn System>> = Vec::new();

        systems.push(Box::new(Callback {
            callback: Box::new(print_health),
        }));

        for sys in &systems {
            sys.run(&entity);
        }
    }
}
