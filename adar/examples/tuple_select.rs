use adar::prelude::*;

fn main() {
    let helloworld = (String::from("Hello"), "world");
    println!("String: {}", helloworld.select::<String>());
    println!("&str: {}", helloworld.select::<&'static str>());

    let mut numbers = (2.2_f32, -55_i8, 652_u32);
    *numbers.select_mut::<i8>() += 10;
    println!("i8: {}", numbers.select::<i8>());
    println!("f32: {}", numbers.select::<f32>());
    println!("u32: {}", numbers.select::<u32>());
}
