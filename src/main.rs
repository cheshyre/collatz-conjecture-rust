use std::collections::HashMap;
// use std::io;

fn do_step(x: u32) -> u32 {
    if x % 2 == 0 {
        return x / 2;
    }
    3 * x + 1
}

fn main() {
    let bottom: u32 = 10000;
    let top: u32 = 10000000;
    // let bottom: u32 = 4;
    // let top: u32 = 10;

    let mut max: u32 = 0;
    let mut max_steps: u32 = 0;

    let mut cache = HashMap::new();
    cache.insert(1, 0);

    for val in bottom..top {
        let mut cur_val = val;
        let mut added_dist = 0;
        while !cache.contains_key(&cur_val) {
            cur_val = do_step(cur_val);
            added_dist += 1;
        }

        let mut dist = *cache.get(&cur_val).unwrap();
        if added_dist > 0 {
            cache.insert(val, dist + added_dist);
        }

        dist += added_dist;

        if dist > max_steps {
            max = val;
            max_steps = dist;
        }

    }

    println!("{} with {} steps", max, max_steps);
    println!("cache_size: {}", cache.len());

}
