#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// implemenation block for caluclating area of rectangle
impl Rectangle {
    fn area_of_rect(&self) -> u32 {
        self.width * self.height
    }
}
// implementation block to check which rectangle is greater

impl Rectangle {
    fn is_greater(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.height
    }
}

// implementation block of assocation function w/o self
impl Rectangle {
    fn area_of_sqr(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
} 

fn main(){
    let rect1 = Rectangle {
        width: 50,
        height: 40,
    };

    let rect2 = Rectangle {
        width: 40,
        height: 30,
    };

    let rect3 = Rectangle {
        width: 40,
        height: 55,
    };

    println!("Is rect1 greater than rect2, {}",rect1.is_greater(&rect2));
    println!("Is rect1 greater than rect3, {}",rect1.is_greater(&rect3));

    println!("The area of rect1, rect2, rect3 are {} {} {}", rect1.area_of_rect(), 
                                                             rect2.area_of_rect(),
                                                             rect3.area_of_rect());
    
    println!("The area of the square is {:#?}.", Rectangle::area_of_sqr(4));

}