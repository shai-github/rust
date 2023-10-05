/*
Closures are like functions except they are anonymous - can be stored as variables or passed as 
arguments to other functions.
*/

use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intesity: u32) -> u32 {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));

    intesity
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

struct Cacher<T>
// Fn provided by std library
// There are also FnMut and FnOnce
where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v, // return value if Some
            None => {
                let v = (self.calculation)(arg); // call closure
                self.value = Some(v); // store value, this is where caching happens
                v // return value
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // input parameter inside vertical pipes | |
    // variable expensive_closure stores the closure itself
    let mut cache_result = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));

        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            cache_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            cache_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                cache_result.value(intensity)
            );
        }
    }
}