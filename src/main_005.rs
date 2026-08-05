use std::{fmt::Debug, ops::Bound, ops::RangeBounds};

fn main() {
    println!("\n");

    inspect_range(200..=400);
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    inspect_range(200..400);

    println!("\n -> The End ...\n");
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

fn inspect_range<R, T>(range: R)
where
    R: RangeBounds<T>,
    T: Debug,
{
    match range.start_bound() {
        Bound::Included(start) => println!(" -> start (inclusive) at {:?}", start),
        Bound::Excluded(start) => println!(" -> start (exclusive) at {:?}", start),
        Bound::Unbounded => println!(" -> unbounded start ..."),
    }

    match range.end_bound() {
        Bound::Included(end) => println!(" -> end (inclusive) at {:?}", end),
        Bound::Excluded(end) => println!(" -> end (exclusive) at {:?}", end),
        Bound::Unbounded => println!(" -> unbounded end ..."),
    }
}
