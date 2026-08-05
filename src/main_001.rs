use crate::friedrich_nietzsche::show_inclusive_range;

fn main() {
    println!("\n");

    show_inclusive_range(0..=23);

    println!("\n -> The End ...\n");
}

mod friedrich_nietzsche {
    use std::ops::RangeInclusive;

    pub fn show_inclusive_range(range: RangeInclusive<i32>) {
        println!(" -> start from: {}", range.start());
        println!(" -> end to : {}", range.end());

        for item in range {
            println!(" {}", item);
        }
    }
}
