use mouse_rs::Mouse;
use rand::Rng;
use std::{env, thread, time::Duration};

struct RngPosition {
    x: i32,
    y: i32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        usage(args.get(0).expect("[ERROR] Program name should always exist."));
        return;
    }

    let width: i32 = args.get(1).unwrap().parse().unwrap_or(100);
    let height: i32 = args.get(2).unwrap().parse().unwrap_or(100);
    
    let mouse = Mouse::new();
    
    loop {
        let rng_pos = get_random_value(width, height);
        println!("{} - {}", rng_pos.x, rng_pos.y);
        mouse.move_to(rng_pos.x, rng_pos.y).expect("[ERROR] Unable to move mouse");
        thread::sleep(Duration::from_secs(5));
    }
}

fn usage(program_name: &String) {
    println!("USAGE: {program_name} <MAX_WIDTH> <MAX_HEIGHT>");
}

fn get_random_value(width: i32, height: i32) -> RngPosition {
    let mut rng = rand::thread_rng();
    let rng_x = rng.gen_range(0..width);
    let rng_y = rng.gen_range(0..height);
    return RngPosition {
        x: rng_x,
        y: rng_y
    };
}
