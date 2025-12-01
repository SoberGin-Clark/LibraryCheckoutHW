#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

mod v1;
use v1::items::{ShowInfo, Book, Dvd};
use v1::catalog::{Catalog};
use v1::member::Member;

fn main() {

    let mut cat = Catalog::new(); // Create new catalog

    cat.add(Box::new(Book::new("B1", "Rust for Humans"))); // Create four entries in the catalog
    cat.add(Box::new(Book::new("B2", "Pythonic Patterns")));
    cat.add(Box::new(Dvd::new("D1", "Taking Flight")));
    cat.add(Box::new(Dvd::new("D2", "Patterns in Motion")));

    println!("{}", cat.get("B1").showTitle()); // Check the title of the first to make sure

    let mut m = Member::new("Clark"); // Make new member

    m.borrow("B1"); // Borrow some items
    m.borrow("D1");

    println!("{} has borrowed: {:?}", m.name(), m.borrowed_ids()); // Display the borrowed IDs
    for (idv, title, days) in cat.details_for(m.borrowed_ids().clone()) { // Display details of the borrowed items
        println!(" • {} — {} ({} days)", idv, title, days);
    }

    m.return_item("B1"); // Return an item
    println!("{} has now borrowed: {:?}", m.name(), m.borrowed_ids()); // Display IDs to confirm return
}

// TESTS - Only run when testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Id already in use in list!")] // Panics when duplicate items are added to one catalog
    fn no_duplicate_entries() {
        let mut cat = Catalog::new();
        cat.add(Box::new(Book::new("B1", "Rust for Humans")));
        cat.add(Box::new(Book::new("B1", "Rust for Humans")));
    }

    #[test]
    #[should_panic(expected = "Couldn't get value for ID- not in catalog!")]
    fn cant_test_whats_not_there() {
        let mut cat = Catalog::new();
        cat.add(Box::new(Book::new("B1", "Rust for Humans")));

        let mut b = Member::new("Betty");
        b.borrow("B1");
        b.borrow("B2");
        for (idv, title, days) in cat.details_for(b.borrowed_ids().clone()) {
        println!(" • {} — {} ({} days)", idv, title, days);
        }
    }

    #[test]
    fn return_everything() {
        let mut s = Member::new("Steve");
        s.borrow("B1");
        s.borrow("B2");
        s.return_item("B1");
        s.return_item("B2");
        let temp : Vec<String> = Vec::new();
        assert_eq!(*s.borrowed_ids(), temp); // Since all were removed, should be an empty vector
    }
}