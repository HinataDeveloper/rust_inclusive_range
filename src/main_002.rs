use crate::friedrich_nietzsche::get_random_number;

fn main() {
    println!("\n");

    let rnd_num = get_random_number(1000..=10000);
    println!(" -> random number is: {}", rnd_num);

    println!("\n -> The End ...\n");
}

mod friedrich_nietzsche {
    use rand::RngExt;
    use std::ops::RangeInclusive;

    pub fn get_random_number(range: RangeInclusive<i32>) -> i32 {
        rand::rng().random_range(range)
    }
}
