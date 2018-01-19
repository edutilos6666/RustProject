use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
use std::io::stdin;

mod SimpleMath;

fn main() {
    println!("Hello World");
    let num = 10;
    let mut age : i32 = 40;
    println!("Max i8 {}", i8::MAX);
    println!("Min i8{}", i8::MIN);
    println!("Max i16 {}", i16::MAX);
    println!("Min i16 {}", i16::MIN);
    println!("Max i32 {}", i32::MAX);
    println!("Min i32 {}", i32::MIN);
    println!("Max i64 {}", i64::MAX);
    println!("Min i64 {}", i64::MIN);
    println!("Max isize {}", isize::MAX);
    println!("Min isize {}", isize::MIN);
    println!("Max usize {}", usize::MAX);
    println!("Min usize {}", usize::MIN);
    println!("Max f32 {}", f32::MAX);
    println!("Min f32 {}", f32::MIN);
    println!("Max f64 {}", f64::MAX);
    println!("Min f64 {}", f64::MIN);

    let is_it_true: bool =true ;
    let let_x:char = 'x';
    println!("I am {} years old", age);
    println!("is it true = {}", is_it_true);
    println!("let_x = {}", let_x);
    let (fname, lname) = ("Leo", "Messi");
    println!("fname = {} and lname = {}", fname, lname);
    //OUTPUT
    println!("It is {0} that {1} is {0}", is_it_true, let_x);
    //formatted output
    println!("{:.2}", 1.234);
    println!("B : {:b} H: {:x} O: {:o}", 10 , 10, 10);
    //zse named arguments
    println!("{ten:>ws$}", ten = 10 , ws = 5);
    println!("{ten:>0ws$}", ten= 10, ws= 5);

    //MATH
    println!("5 + 4 = {}", 5 + 4);
    println!("5 -4 = {}", 5 - 4);
    println!("5 * 4 = {}", 5 * 4);
    println!("5 / 4 = {}", 5 / 4);
    println!("5 % 4 = {}", 5 % 4);
    let mut neg_4 = -4i32;
    println!("abs(-4) = {}", neg_4.abs());
    println!("4 ^ 6 = {}", 4i32.pow(6));
    println!("sqrt 9 = {}", 9f64.sqrt());
    println!("cbrt 9 = {}", 27f64.cbrt());
    println!("Round 1.45 = {}", 1.45f64.round());
    println!("Floor 1.45 = {}", 1.34f64.floor());
    println!("Ceiling 1.45 = {}", 1.45f64.ceil());
    println!("e ^ 2 = {}", 2f64.exp());
    println!("log(2) = {}", 2f64.ln());
    println!("log10(2) = {}", 2f64.log10());
    println!("90 to Radians = {}", 90f64.to_radians());
    println!("PI to Degrees =  {}", 3.14f64.to_degrees());
    println!("Max 4, 5 = {}", 4f64.max(5f64));
    println!("Min 4, 5 = {}", 4f64.min(5f64));
    println!("Sin 3.14 = {}", 3.14f64.sin());
    let sin  = 1f64.asin().sin();
    let cos = 1f64.acos().cos();
    let tan = 1f64.atan().tan();
    let sinh = 1f64.asinh().sinh();
    let cosh = 1f64.acosh().cosh();
    let tanh = 0.99f64.atanh().tanh();
    println!("sin asin 1 = {}", sin);
    println!("cos acos 1 = {}", cos);
    println!("tan atan 1 = {}", tan);
    println!("sinh asinh 1 = {}", sinh);
    println!("cosh acosh 1 = {}", cosh);
    println!("tanh atanh 0.99 = {}", tanh);
    //conditionals
    let age_old = 6;
    if (age_old == 5) {
        println!("Go to kindergarten");
    } else if(age_old > 5) && (age_old <= 18) {
        println!("Go to grade {}", (age_old -5));
    } else if(age_old <= 25) && (age_old > 18) {
        println!("Go to college.");
    } else {
        println!("Do what you want.");
    }

    println!("!true = {}", !true);
    println!("true || false = {}", true|| false);
    println!("true != false = {}", (true != false));
    //ternary op
    let can_vote = if(age_old >= 18) {true} else {false};
    println!("Can Vote = {}", can_vote);
    // looping
    let mut x = 1 ;
    loop {
        if((x% 2) == 0) {
            println!("{}", x);
            x += 1;
            continue;
        }
        if(x > 10) {
            break;
        }
        x += 1 ;
        continue;
    }

    let mut y = 1 ;
    while y <= 10 {
        println!("WHILE : {}", y);
        y += 1 ;
    }
    for z in 1..10 {
        println!("FOR : {}", z);
    }

    // Strings
    let rand_string = "I am a random string";
    println!("Length = {}", rand_string.len());
    let (first, second) = rand_string.split_at(6);
    println!("first = {} , and second = {}", first , second);

    let count = rand_string.chars().count();
    let mut chars = rand_string.chars();
    let mut indiv_char = chars.next();
    loop {
        match indiv_char {
            Some (x) => println!("{} ",  x),
            None => break,
        }
        indiv_char = chars.next();
    }

    //split on whitespaces
    let mut iter = rand_string.split_whitespace();
    let mut indiv_word = iter.next();
    loop {
        match  indiv_word {
            Some(x) => println!("{}", x),
            None => break,
        }
        indiv_word = iter.next();
    }

    let rand_string2 =  "I am a random string\nThere are other strings like it\nThis string is the best";
    let mut lines = rand_string2.lines();
    let mut indiv_line = lines.next();
    loop {
        match indiv_line {
            Some(x) => println!("{} ", x),
            None => break ,
        }
        indiv_line = lines.next();
    }

    println!("Find Best : {}", rand_string2.contains("best"));

    //input
    'outer: loop {
        let number:i32 = 10 ;
        println!("Pick a Number");
        loop {
            let mut line = String::new();
            let input = stdin().read_line(&mut line);
            let guess:Option<i32> = input.ok().map_or(None, |_| line.trim().parse().ok());
            match guess  {
                None => println!("enter a Number"),
                Some(n) if n == number => {
                    println!("You Guessed It");
                    break 'outer;
                }
                Some(n) if n < number => println!("Too Low"),
                Some(n) if n > number => println!("Too High"),
                Some(_) => println!("Error")
            }
        }
    }  // end of 'outer loop

    let mut id = String::new() ;
    let mut name = String::new();
    let mut age = String::new() ;
    let mut wage = String::new();
    let mut active = String::new();
    println!("Enter your id: ");
    stdin().read_line(&mut id);
    println!("Enter your name: ");
    stdin().read_line(&mut name);
    println!("Enter your age: ");
    stdin().read_line(&mut age);
    println!("Enter your wage: ");
    stdin().read_line(&mut wage);
    println!("Enter your active: ");
    stdin().read_line(&mut active);

    println!("<<You entered>>");
    println!("id = {}", id.trim());
    println!("name = {}", name.trim());
    println!("age = {}", age.trim());
    println!("wage = {}", wage.trim());
    println!("active = {}", active.trim());


    // Arrays
    let rand_array = [1,2,3];
    println!("{}", rand_array[0]);
    println!("{}", rand_array.len());
    //:? for printing array
    println!("Second 2 : {:?}", &rand_array[1..3]);

    // Vectors
    let mut vect1 = vec![1,2,3,4,5];
    println!("Item 2 : {}", vect1[1]);
    for i in &vect1 {
        println!("vect: {}", i);
    }
    println!("");
    // without &vect1 , vect1 is moved , and can not be used further
    // for i in vect1 {
    //     println!("vect = {}", i);
    // }

    vect1.push(6);
    vect1.pop();


    // tuples
    let rand_tuple = ("foobar", 40);
    let rand_tuple_2: (&str, i8) = ("foobar", 40);
    println!("Name = {}", rand_tuple_2.0);
    println!("rand_tuple = {:?}", rand_tuple);
    println!("rand_tuple_2 = {:?}", rand_tuple_2);

    // functions
    say_hello("edutilos");
    println!("5 + 4 = {}", get_sum(5, 4));
    let sum = get_sum ;
    println!("6 +4 = {}", sum(6,4));

    //closures
    let sum_nums = |x : i32 , y: i32| x + y;
    println!("7 + 8 = {}", sum_nums(7,8));
    let num_ten = 10 ;
    let add_10 = |x: i32| x + num_ten;
    println!("5 + 10 = {}", add_10(5));

    //ownership /pointers
    let vect1 = vec![1,2,3];
    //we can not use vect1 anymore , because data was
    // moved to vect2
    // let vect2 = vect1 ;
    let vect2 = &vect1;
    println!("vect1 = {:?}", vect1);
    println!("vect2 = {:?}", vect2);

    //for primitive data types copy by assignment is applied
    let prim_val = 1 ;
    let prim_val2 = prim_val ;
    println!("prim_val = {}, and prim_val2 = {}", prim_val, prim_val2);


    let vect2 = vec![1,2,3,4];
    // println!("Sum of Vect =  {}", sum_vects(vect2));
    // println!("Vect2 = {:?}", vect2);
    println!("Sum of Vect = {}", sum_vects(&vect2));
    println!("Vect2 = {:?}", vect2);

    println!("");
    println!("Sum of Vect =  {}", sum_vects2(vect2));
    //error vect2 is moved
    // println!("Vect2 = {:?}", vect2);
    // following 2 lines spit errors as well
    // println!("Sum of Vect = {}", sum_vects2(&vect2));
    // println!("Vect2 = {:?}", vect2);


    //Structs
    let mut circle1 = Circle {
        x: 10.0 , y:210.0, radius: 10.0
    };
    println!("x = {}, y = {}, r = {}", circle1.x, circle1.y , circle1.radius);
    circle1.x = 20.0 ;
    println!("x = {}, y = {}, r = {}", circle1.x, circle1.y , circle1.radius);

    println!("Circle Radius : {}", get_radius(&circle1));
    println!("Circle x = {}", circle1.get_x());

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

    //traits
    println!("Circle Area = {}", circle1.area());
    let mut rect1 = Rectangle {
         height: 10.0, width: 10.0
    };
    println!("Rect Area = {}", rect1.area());

    //enums
    let hulk = Hero::Strong(100);
    let quicksilver = Hero::Fast;
    let spiderman = Hero::Info {name: "Spipderman".to_owned(),
      secret: "Peter Parker".to_owned() };

    get_info(hulk);
    get_info(quicksilver);
    get_info(spiderman);


    //module example
    let x:f64 = 10f64 ;
    let y: f64 = 3.0;

    let add_res = SimpleMath::add(x, y);
    let sub_res = SimpleMath::sub(x,y);
    let mult_res = SimpleMath::mult(x, y);
    let div_res = SimpleMath::div(x,y);
    println!("<<SimpleMath>>");
    println!("x = {} and y = {}", x, y);
    println!("x + y = {}", add_res);
    println!("x - y = {}", sub_res);
    println!("x * y = {}", mult_res);
    println!("x / y = {}", div_res);

}


// structs
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




// traits
struct Rectangle {
    height: f64 ,
    width: f64 ,
}

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


// enums
enum Hero {
    Fast ,
    Strong(i32),
    Info {name: String , secret: String}
}

fn get_info(h:Hero) {
    match h {
        Hero::Fast => println!("Fast"),
        Hero::Strong(i) => println!("Lifts {} tons", i),
        Hero::Info {name, secret} => {
            println!("{} is {}", name, secret);
        },
    }
}

// functions
fn say_hello(name: &str) {
    println!("Hello {}", name);
}

fn get_sum(num1:i32 , num2: i32) -> i32 {
    num1 + num2
}

fn sum_vects(v1: &Vec<i32>) -> i32 {
    let sum = v1.iter().fold(0, |mut sum, &x| {sum += x ; sum});
    return sum;
}


fn sum_vects2(v1: Vec<i32>) -> i32 {
    let sum = v1.iter().fold(0, |mut sum, &x| {sum += x ; sum});
    return sum;
}
