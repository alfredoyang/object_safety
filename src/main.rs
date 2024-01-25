#[macro_use]
extern crate static_assertions;

use std::fmt::Debug;

/*
    Object-Safety
    1. https://stackoverflow.com/a/30941589
    2. https://rust-lang.github.io/rfcs/0255-object-safety.html
    3. https://doc.rust-lang.org/book/ch17-02-trait-objects.html#trait-objects-perform-dynamic-dispatch
    4. Trait object is DST, so Self wont be Sized by default.
*/
trait Animal {
    fn new() -> Box<dyn Animal> where Self: Sized;
    fn new2() -> Self where Self: Sized;
    fn do_nothing() where Self: Sized {}
    fn die(self);
    fn run(&self);
    fn walk(&self);
}

struct Cat {}

impl Animal for Cat {
    fn new() -> Box<dyn Animal> where Self: Sized {
        Box::new(Cat {})
    }

    fn new2() -> Self where Self: Sized {
        Cat {}
    }

    fn die(self) {
        // move happens
    }

    fn run(&self) {
        println!("Cat is running.");
    }
    fn walk(&self) {
        println!("Cat is walking.");
    }
}

fn animal_walk(animal: &dyn Animal) {
    animal.walk();
}

fn animal_run(animal: &dyn Animal) {
    animal.run();
}

fn main() {
    assert_obj_safe!(Animal);
    assert_obj_safe!(Debug);
    let cat = Cat::new();

    animal_walk(&*cat);
    animal_run(&*cat);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn animal_object_safe() {
        assert_obj_safe!(Animal);
    }

    #[test]
    fn create_cat() {
        let _cat = Cat::new();
    }

    #[test]
    fn create_cat2() {
        let cat = Cat::new2();
        cat.die();
    }
}