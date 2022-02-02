use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;
mod counter;

struct Cacher<T, U, V>
where
    T: Fn(U) -> V,
    U: Eq + Hash + Copy,
    V: Copy,
{
    calculation: T,
    results: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V>
where
    T: Fn(U) -> V,
    U: Eq + Hash + Copy,
    V: Copy,
{
    fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            results: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> V {
        match self.results.get(&arg) {
            Some(&v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.results.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(1));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            )
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    use_move_keyword();

    use_iterator_constructors();

    use_consuming_adaptor();

    use_iterator_adaptor();

    use_iterator_with_capturing_closure();

    // custom iterator
    let counter = counter::Counter::new();
    println!("counter: {:?}", counter);
}

fn use_move_keyword() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    let y = vec![1, 2, 3];
    let z = vec![1, 5, 3];

    assert!(equal_to_x(y));
    assert!(!equal_to_x(z));
}

fn use_iterator_constructors() {
    println!("{}", "Use iterator constructors");

    println!("{}", "Use #iter");
    // use immutable references from an immutable collection
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    v1_iter.for_each(|n| println!("{}", n));

    println!("{}", v1[2]);

    println!("{}", "Use #iter_mut");
    // Mutate references from an mutable collection
    let mut v2 = vec![1, 2, 3];
    let v2_iter = v2.iter_mut();
    v2_iter.for_each(|n| *n = 1);

    println!("{}", v2[2]);

    println!("{}", "Use #into_iter");
    // Take ownership of a collection and
    let v3 = vec![1, 2, 3];
    let v3_iter = v3.into_iter();
    v3_iter.for_each(|n| drop(n));

    //borrow of moved value;
    //println!("{}", v3[2]);
}

fn use_consuming_adaptor() {
    println!("{}", "Use a consuming adaptor");

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    assert_eq!(v1_iter.sum::<i32>(), 6);

    // borrow of moved value due to #sum taking ownership of v1_iter
    // v1_iter.next();
}

fn use_iterator_adaptor() {
    println!("{}", "Use an iterator adaptor");

    let v1 = vec![1, 2, 3];

    println!("{:?}", v1.iter().map(|x| x + 1).collect::<Vec<_>>());
}

fn use_iterator_with_capturing_closure() {
    println!("{}", "Use an iterator with a capturing closure");

    let v1 = vec![1, 2, 3, 4, 5];

    assert_eq!(
        vec![1, 3, 5],
        v1.into_iter().filter(|n| n % 2 == 1).collect::<Vec<_>>()
    );

    // borrow fo moved value
    // println!("v1: {:?}", v1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cacher_caches_different_ints() {
        let mut cacher = Cacher::new(|x| x);

        assert_eq!(cacher.value(1), 1);
        assert_eq!(cacher.value(2), 2);
    }

    #[test]
    fn cacher_caches_different_string_slices() {
        let mut cacher = Cacher::new(|x| x);

        assert_eq!(cacher.value("foobar"), "foobar");
        assert_eq!(cacher.value("fizbuz"), "fizbuz");
    }

    #[test]
    fn cacher_caches_different_key_value_types() {
        let mut cacher = Cacher::new(|x: &str| x.len());

        assert_eq!(cacher.value("foo"), 3);
        assert_eq!(cacher.value("fizbuz"), 6);
    }

    // #[test]
    // fn cacher_caches_non_copy_types() {
    //     let mut cacher = Cacher::new(|x: String| x);

    //     assert_eq!(cacher.value("foo".to_string()), "foo".to_string());
    // }

    #[test]
    fn zip_different_length_iterators() {
        let v1 = vec![1, 2, 3, 4];
        let v2 = vec![1, 2, 3];

        let pairs = v1.into_iter().zip(v2).collect::<Vec<_>>();

        assert_eq!(pairs, vec![(1, 1), (2, 2), (3, 3)]);
    }
}
