struct Rectangle {
    width: u32,
    height: u32,
}

// can use impl to define struct method
impl Rectangle {
    // self gives access to struct fields using dot operator
    // calculate the area of a rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // method to compare rectangles and determine if
    // rectangle can hold the area of another rectangle
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// can have multiple impl blocks
// we will write associated functions here
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;
    let area1 = area(width1, height1);

    println!(
        "The area of the rectangle is {area1} square pixels.",
    );

    let rect = Rectangle {
        width: width1,
        height: height1
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    // let's try comparing rectangles
    let rect1 = Rectangle {
        width: 20,
        height: 40
    };

    let rect2 = Rectangle {
        width: 40,
        height: 50
    };

    println!(
        "Can rect hold rect1? {}",
        rect1.can_hold(&rect2)
    );

    println!(
        "Can rect hold rect2? {}",
        rect.can_hold(&rect2)
    );

    println!(
        "Can rect1 hold rect2? {}",
        rect1.can_hold(&rect2)
    );

    println!(
        "Can rect2 hold rect1? {}",
        rect2.can_hold(&rect1)
    );

    // let's use associated functions, which are not tied to instance of a struct
    let rect3 = Rectangle::square(10);
    
    println!(
        "The area of the rectangle is {} square pixels.",
        rect3.area()
    );
}

// this function syntax does not express that the width and height are related
fn area(width: u32, height: u32) -> u32 {
    width * height
}
