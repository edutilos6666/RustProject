pub struct M {}

impl M {
    pub fn example1(&self) {
        println!("<<example1()>>");
        let mut circle1 = Circle {
            x: 10.0 , y:210.0, radius: 10.0
        };
        println!("x = {}, y = {}, r = {}", circle1.x, circle1.y , circle1.radius);
        circle1.x = 20.0 ;
        println!("x = {}, y = {}, r = {}", circle1.x, circle1.y , circle1.radius);

        println!("Circle Radius : {}", get_radius(&circle1));
        println!("Circle x = {}", circle1.get_x());
        println!();
    }

    pub fn example2(&self) {
        println!("<<example2()>>");
        let mut person = Person {
            id: 1, name: String::from("foo"), age: 10, wage: 100.0, active: true
        };

        println!("person = {}", person.toString());
        person.setId(10);
        person.setName(String::from("bar"));
        person.setAge(20);
        person.setWage(200.0);
        person.setActive(false);
        println!("person = {}", person.toString());

        println!("<<person details>>");
        println!("id = {}", person.getId());
        println!("name = {}", person.getName());
        println!("age = {}", person.getAge());
        println!("wage = {}", person.getWage());
        println!("active = {}", person.isActive());

        person.id = 3 ;
        person.name = String::from("bim");
        person.age = 30;
        person.wage = 300.0;
        person.active = true;
        println!("<<person details>>");
        println!("id = {}", person.id);
        println!("name = {}", person.name);
        println!("age = {}", person.age);
        println!("wage = {}", person.wage);
        println!("active = {}", person.active);
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


struct Person {
    id: i64 ,
    name: String ,
    age: i32 ,
    wage: f64 ,
    active: bool ,
}



impl Person {
    pub fn getId(&self) -> i64 {
        return self.id ;
    }

    pub fn getName(&self) -> &String {
        return &self.name;
    }

    pub fn getAge(&self) -> i32 {
        return self.age ;
    }

    pub fn getWage(&self)-> f64 {
        return self.wage ;
    }

    pub fn isActive(&self) -> bool {
        return self.active ;
    }

    pub fn setId(&mut self, id:i64) {
        self.id = id ;
    }

    pub fn setName(&mut self, name:String) {
        self.name = name;
    }

    pub fn setAge(&mut self, age:i32) {
        self.age = age ;
    }
    pub fn setWage(&mut self, wage:f64) {
        self.wage = wage;
    }
    pub fn setActive(&mut self, active:bool) {
        self.active = active ;
    }

    pub fn toString(&mut self) -> String {
        return format!("Person({},{},{},{},{})", self.id ,
    self.name, self.age, self.wage, self.active);
    }
}
