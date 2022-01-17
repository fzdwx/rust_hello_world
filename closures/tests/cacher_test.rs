use std::thread;
use std::time::Duration;
use closures::cacher::Cacher;

fn generate_workout(intensity: u32, random_number: u32) {
    let mut test = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            test.get(intensity)
        );
        println!(
            "Next, do {} situps!",
            test.get(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                test.get(intensity)
            );
        }
    }
}

#[test]
fn test2() {
    generate_workout(2, 4);
}

#[test]
fn test1() {
    let mut cacher = Cacher::new(|num| {
        num
    });
    let v1 = cacher.get(1);
    let v2 = cacher.get(2);

    assert_eq!(v2, 2);
    assert_eq!(v1, 1);
}