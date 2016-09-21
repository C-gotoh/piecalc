use std::env;

fn main() {
    let mut p = 3.0;
    let mut count = 2.0;
    let mut sign = true;
    let args: Vec<String> = env::args().collect();
    let it = args[1].parse::<i32>().unwrap();
    println!("Making Pie");
    for _ in 0..it {
        if sign {
            p += 4.0 / (count * (count + 1.0) * (count + 2.0));
        } else {
            p -= 4.0 / (count * (count + 1.0) * (count + 2.0));
        }
        sign = !sign;
        count += 2.0;
    }
    println!("Pie is {}", p);
}
