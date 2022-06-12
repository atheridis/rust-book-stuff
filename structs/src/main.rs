struct Rectangle {
    width: f32,
    height: f32,
}

impl Rectangle {
    fn square(size: f32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> f32 {
        self.height * self.width
    }

    fn fits(&self, other: &Rectangle) -> bool {
        self.width <= other.width && self.height <= other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 3.5,
        height: 5.0,
    };

    let rect2 = Rectangle {
        width: 3.7,
        height: 5.0,
    };

    println!(
        "The rectangle with height {} and width {} {} fit into the rectangle \
        with height {} and width {}",
        rect1.height,
        rect1.width,
        if rect1.fits(&rect2) { "can" } else { "cannot" },
        rect2.height,
        rect2.width
    );

    println!("{}", Rectangle::square(3.0).area());
}
