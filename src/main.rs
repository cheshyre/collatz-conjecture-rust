use std::collections::HashMap;
use std::io;

fn do_step(x: u32) -> u32 {
    if x % 2 == 0 {
        return x / 2;
    }
    3 * x + 1
}

fn main() {
    let bottom: u32 = 10000;
    let top: u32 = 100000;
    // let bottom: u32 = 4;
    // let top: u32 = 10;

    let mut max: u32 = 0;
    let mut max_steps: u32 = 0;

    let mut cache = HashMap::new();
    cache.insert(1, 0);

    for val in bottom..top {
        let mut stack = vec![];
        let mut cur_val = val;
        while !cache.contains_key(&cur_val) {
            stack.push(cur_val);
            cur_val = do_step(cur_val);
        }

        let mut dist = *cache.get(&cur_val).unwrap();
        while stack.len() != 0 {
            dist += 1;
            cur_val = stack.pop().unwrap();
            cache.insert(cur_val, dist);
        }

        // println!("{} {}", val, dist);
        if dist > max_steps {
            max = val;
            max_steps = dist;
        }

    }

    println!("{} with {} steps", max, max_steps);
    println!("cache_size: {}", cache.len());

}
