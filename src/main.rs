pub mod a_o_c_2023;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let res = a_o_c_2023::day_9::solve_part_1();
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    print!("Your result: {}", res.unwrap());
}
