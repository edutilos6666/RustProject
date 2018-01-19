pub struct M {}

impl M {
    pub fn example1(&self) {
        println!("<<example1()>>");
        let mut circle1 = Circle {
            x: 10.0 , y:210.0, radius: 10.0
        };

        println!("Circle Area = {}", circle1.area());
        let mut rect1 = Rectangle {
             height: 10.0, width: 10.0
        };
        println!("Rect Area = {}", rect1.area());
        
        println!();
    }
}



struct Circle {
    x: f64 ,
    y: f64 ,
    radius : f64 ,
}

fn get_radius(circle: &Circle) -> f64 {
    circle.radius
}

// it is recommended to define struct methods with
// impl keyword
impl Circle {
    pub fn get_x(&self) -> f64 {
        self.x
    }
}



struct Rectangle {
    height: f64 ,
    width: f64 ,
}


//trait
trait HasArea {
    fn area(&self) -> f64 ;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        3.14159* (self.radius * self.radius)
    }
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.height* self.width
    }
}
