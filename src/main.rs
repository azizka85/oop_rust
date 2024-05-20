pub mod oop;

use oop::{MonotonicStack, CarFabric};

fn main() {
    let mut monotonic_stack = MonotonicStack::new(
        &vec![73, 74, 75, 71, 69, 72, 76, 73]
    );

    println!("{:?}", monotonic_stack);
    println!("{:?}", monotonic_stack.data());    

    monotonic_stack.add(57);
    println!("{:?}", monotonic_stack.data());

    monotonic_stack.add_list(&vec![34, 23, 11]);
    println!("{:?}", monotonic_stack.data());

    let car_fabric = CarFabric::new(String::from("China"));

    println!("{:?}", car_fabric.create_car(false, String::from("Honda"), 250, None));
    println!("{:?}", car_fabric.create_car(true, String::from("Mercedes"), 235, Some(String::from("Germany"))));
    println!("{:?}", car_fabric.create_car(true, String::from("Zeekr"), 235, None));
}
