struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn double_width(&mut self) {
        self.width = self.width * 2
    }
}

fn main() {
    let mut rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {}",
        rect1.area()
    );

    println!("the width of the rectangle is: {}", rect1.width);

    let ref_to_rect1 = &rect1;
    println!("the reference rectangle width is: {}", ref_to_rect1.width);

    // you can make a mutable ref if the immutable ref(s)
    // aren't going to be used again:

    rect1.double_width();

    // but if you try to use the immutable ref again after the mutable ref
    // was created, you'll get an error:

    // println!("the reference rectangle width is: {}", ref_to_rect1.width);

    println!("the new width of the rectangle is: {}", rect1.width);
}