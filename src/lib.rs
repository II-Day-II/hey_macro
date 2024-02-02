//! # hey! macro
//!
//! This crate contains a macro that lets your objects know who's the boss.
//! Make your method calls with authority!

/// Commands an object to call a method with authority
/// as though it were a dog being commanded to perform a trick
///
/// # Examples
///
/// ```
/// use hey_macro::hey;
/// let mut nums = [0, 1, 2, 3, 5, 9, 8, 4, 7, 6];
/// hey!(nums, sort);
/// assert_eq!(nums, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
/// ```
/// ```
/// use hey_macro::hey;
/// let nums = [0, 1, 2, 3, 5];
/// let len = hey!(nums, len);
/// assert_eq!(len, 5);
/// ```
/// ```
/// use hey_macro::hey;
/// let mut nums = vec![0, 1, 2, 3, 5];
/// hey!(nums, push, 4);
/// assert_eq!(nums, vec![0, 1, 2, 3, 5, 4]);
/// ```
#[macro_export]
macro_rules! hey {
    ($object:expr, $method:ident) => {
        $object.$method()
    };
    ($object:expr, $method:ident, $($arg:expr),*) => {
        $object.$method($($arg,)*)
    };

}

// TODO: find a way to make this the same macro as hey!
// #[macro_export]
// macro_rules! hey_a {
//     ($type:ty, $func:ident) => {
//         <$type>::$func()
//     };
//     ($type:ty, $func:ident, $($arg:expr),*) => {
//         <$type>::$func($($arg,)*)
//     };
// }

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn assoc() {
    //     let s = hey_a!(String, new);
    //     assert_eq!(s, "");
    // }
    //
    // #[test]
    // fn assoc_arg() {
    //     let v = hey_a!(Vec<u8>, with_capacity, 10);
    //     assert_eq!(v.capacity(), 10);
    // }

    #[test]
    fn builtin(){
        let mut nums = [0, 1, 2, 3, 5, 9, 8, 4, 7, 6];
        hey!(nums, sort);
        assert_eq!(nums, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn state() {
        struct Concater(u8);
        impl Concater {
            pub fn concat(&mut self, a: impl Into<String>, b: impl Into<String>) -> String {
                self.0 += 1;
                a.into() + &b.into()
            }
            pub fn how_much_work_have_you_done(&self) -> Option<u8> {
                Some(self.0)
            }
        }
        let mut conny = Concater{0: 0};
        let work = hey!(conny, concat, "Hello,", " World!");
        assert_eq!(work, "Hello, World!");
        if let Some(x) = (|| {Some(hey!(conny, how_much_work_have_you_done)?)})() {
            assert_eq!(x, 1);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn adding() {
        struct Adder<T>(std::marker::PhantomData<T>);
        impl<T: std::ops::Add<Output = T>> Adder<T> {
            pub fn add(&self, a: T, b: T)  -> T
            {
                a + b
            }
        }
        let adder : Adder<i32> = Adder(std::marker::PhantomData);
        let result = hey!(adder, add, 3, 2);
        assert_eq!(result, 5);
    }

    #[test]
    fn hello() {
        struct Greeter;
        impl Greeter {
            pub fn say_hello(&self, to: &mut String) {
                *to = String::from("Hello!");
            }
        }
        let greeter = Greeter;
        let mut buf = String::new();
        hey!(greeter, say_hello, &mut buf);
        assert_eq!(&buf, "Hello!");
    }
}
