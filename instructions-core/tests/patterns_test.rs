use instructions_core::patterns;
use instructions_core::patterns::{
    my_take, Context, ContextData, ContextStrategy, SomeTrait, Strategy,
};
use std::fmt;

struct Radix {
    x: i32,
    radix: u32,
}

impl Radix {
    fn new(x: i32, radix: u32) -> Result<Self, &'static str> {
        if radix < 2 || radix > 36 {
            Err("Unnsupported radix")
        } else {
            Ok(Self { x, radix })
        }
    }
}

impl fmt::Display for Radix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut x = self.x;
        // Good for binary formatting of `u128`s
        let mut result = ['\0'; 128];
        let mut used = 0;
        let negative = x < 0;
        if negative {
            x *= -1;
        }
        let mut x = x as u32;
        loop {
            let m = x % self.radix;
            x /= self.radix;

            result[used] = std::char::from_digit(m, self.radix).unwrap();
            used += 1;

            if x == 0 {
                break;
            }
        }

        if negative {
            write!(f, "-")?;
        }

        for c in result[..used].iter().rev() {
            write!(f, "{}", c)?;
        }

        Ok(())
    }
}

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

#[test]
pub fn test_number_base_to_string() {
    assert_eq!(Radix::new(1234, 10).unwrap().to_string(), "1234");
    assert_eq!(Radix::new(1000, 10).unwrap().to_string(), "1000");
    assert_eq!(Radix::new(0, 10).unwrap().to_string(), "0");
}
