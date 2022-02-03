
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let width1 = 30;
    let heigth1 = 50;
    println!("The are of rectangle is {} square pixels",
        area1(width1, heigth1)
    );

    let rect1 = (30, 50); // Struct with tuplesx`
    println!("The are of rectangle is {} square pixels",
        area2(rect1)
    );
    let scale = 2;
    let rect = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    println!("The are of rectangle is {} square pixels from .area() method",
        rect.area()
    );

    println!("The are of rectangle is {} square pixels",
        area3(&rect)
    );

    println!("rect is {:?}", rect);

    println!("rect is {:#?}", rect);

    dbg!(&rect);

    let rect2 = Rectangle {
        width: 10,
        height: 25,
    };
    println!("Can rect1 hold rect2? {}", 
            rect.can_hold(&rect2)
    );
    let square = Rectangle::square(10);
    println!("square is {:#?}", square);
}

fn area1(width: u32, heigth: u32) -> u32 {
    width * heigth
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}