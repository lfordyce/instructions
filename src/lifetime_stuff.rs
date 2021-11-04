trait Getter<'a> {
    fn get<'b>(&'b self) -> &'a u8
    where
        'a: 'b;
}

impl<'a> Getter<'a> for &'a u8 {
    fn get<'b>(&'b self) -> &'a u8
    where
        'a: 'b,
    {
        *self
    }
}

trait Data {
    fn dump(&mut self); // this needs to be mutable by intention, as the code is highly reduced
}

struct MyData<'a> {
    val1: &'a mut u8,
    val2: u8,
}

impl<'a> Data for MyData<'a> {
    fn dump(&mut self) {
        println!("val = {}", self.val2);
    }
}

fn create_data<'a>(num: &'a mut u8) -> Box<dyn Data + 'a> {
    Box::new(MyData { val1: num, val2: 8 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifetime_trait_fun() {
        let x = 10u8;
        let x_ref = &x;

        {
            let r = x_ref.get();
            println!("{:?}", r);
        }
        {
            let r = x_ref.get();
            println!("{:?}", r);
        }
    }

    #[test]
    fn test_lifetime() {
        let mut a: u8 = 4;
        let mut x = create_data(&mut a);
        x.dump();
    }
}
