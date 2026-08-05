fn main() {
    println!("\n");

    iterate_over_something(0..=10);
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    iterate_over_something(20..30);

    println!("\n -> The End ...\n");
}

fn iterate_over_something<I>(range: I)
where
    I: IntoIterator<Item = i32>,
{
    for item in range.into_iter() {
        println!(" -> {}", item);
    }
}
