use std::time::Duration;

fn main() {
    println!("Hello, world!");

    let s = create_str_1mb();

    // print the size of the string in mb
    println!("Size of the string: {:.2}mb", str_get_size_mb(&s));

    // create vector of strings
    let mut str_buffers = Vec::new();

    const GB_SIZE: i32 = 1024;

    // ramp up the vector to 2gb
    ramp_up(&mut str_buffers, GB_SIZE * 6);

    // print the size of the vector in mb
    println!("ramp up: {:.2}mb", get_size_of_buffer(&mut str_buffers));

    // sleep idle for 10 seconds
    println!("sleeping for 10 seconds");
    std::thread::sleep(Duration::from_secs(10));

    // ramp down the vector
    ramp_down(&mut str_buffers);

    // print the size of the vector in mb
    println!("ramp down: {:.2}mb", get_size_of_buffer(&mut str_buffers));

    // sleep idle for 60 seconds
    println!("idle for 60 seconds");
    std::thread::sleep(Duration::from_secs(60));
}

fn get_size_of_buffer(v: &mut Vec<String>) -> f64 {
    // create variable to store the total size of the vector
    let mut total_size = 0.0;

    // iterate over the vector and calculate the total size
    for s in v {
        total_size += str_get_size_mb(s);
    }

    total_size
}

fn ramp_up(v: &mut Vec<String>, desire_size_mb: i32) {
    for _ in 0..desire_size_mb {
        let str = create_str_1mb();
        v.push(str);
        println!("add -> {:.2}mb", get_size_of_buffer(v));
    }
}

fn ramp_down(v: &mut Vec<String>) {
    v.clear();
}

fn create_str_1mb() -> String {
    let mut s = String::new();
    for _ in 0..(1024 * 1024) {
        s.push('a');
    }
    s
}

fn str_get_size_mb(s: &str) -> f64 {
    let size = std::mem::size_of_val(s) as f64;
    size / 1024.0 / 1024.0
}
