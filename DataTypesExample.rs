use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
pub struct M {}

impl M {
    pub fn example1(&self) {
        println!("<<example1()>>");
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
        println!();
    }
}
