use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(t: T) -> Self {
        Self(t)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("An instance of MyBox<T> was dropped.")
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);
    let z = *y;
    println!("{z}");
}
