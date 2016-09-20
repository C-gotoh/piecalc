use std::env;

fn main() {
    let mut p = 4.0;
    let mut count = 3.0;
    let mut sign = false;
    let args: Vec<String> = env::args().collect();
    let it = args[1].parse::<i32>().unwrap();
    println!("Making Pie");
    for x in 0..it {
        if sign {
            p += 4.0/count; 
        }
        else {
            p -= 4.0/count;
        }
        sign = !sign;
        count += 2.0;
    }
    println!("Pie is {}", p);
}
