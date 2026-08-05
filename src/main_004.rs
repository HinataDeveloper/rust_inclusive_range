use std::{fmt::Debug, ops::Bound, ops::RangeBounds};

fn main() {
    println!("\n");

    inspect_range(0..20);
    inspect_range(100..=200);

    println!("\n -> The End ...\n");
}

fn inspect_range<R, T>(range: R)
where
    R: RangeBounds<T>,
    T: Debug,
{
    match range.start_bound() {
        Bound::Included(start) => println!(" -> starts (inclusive) at: {:?}", start),
        Bound::Excluded(start) => println!(" -> starts (exclusive) at: {:?}", start),
        Bound::Unbounded => println!(" -> unboounded start"),
    }

    match range.end_bound() {
        Bound::Included(end) => println!(" -> end (inclusive) at: {:?}", end),
        Bound::Excluded(end) => println!(" -> end (exclusive) at: {:?}", end),
        Bound::Unbounded => println!(" -> unbounded end"),
    }
}
