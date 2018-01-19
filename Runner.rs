//use std::{i8,i16,i32,i64,u8,u16,u32,u64, isize, usize, f32, f64};
use std::io::stdin;

mod OperatorsExample;
mod DataTypesExample;
mod DecisionMakingExample;
mod LoopExample;
mod StringExample;
mod IOExample;
mod ContainerDataTypesExample;
mod FunctionExample;
mod StructExample;
mod TraitExample;
mod EnumExample;
mod SimpleMath;
pub fn main() {
    run_OperatorsExample();
    run_DataTypesExample();
    run_DecisionMakingExample();
    run_LoopExample();
    run_StringExample();
    run_IOExample();
    run_ContainerDataTypesExample();
    run_FunctionExample();
    run_StructExample();
    run_TraitExample();
    run_EnumExample();
    run_ModuleExample();
}

fn run_OperatorsExample() {
    println!("<<OperatorsExample>>");
    let mut ex = OperatorsExample::M {};
    ex.test_ArithmeticOps();
    ex.test_RelationalOps();
    ex.test_LogicalOps();
    ex.test_BitwiseOps();
    ex.test_MathFunctions();
    println!("");
}


fn run_DataTypesExample() {
    println!("<<DataTypesExample>>");
    let ex =  DataTypesExample::M{};
    ex.example1();
    println!();
}


fn run_DecisionMakingExample() {
    println!("<<DecisionMakingExample>>");
    let ex = DecisionMakingExample::M{};
    ex.example1();
    ex.example2();
    println!();
}


fn run_LoopExample() {
    println!("<<LoopExample>>");
    let ex = LoopExample::M{};
    ex.example1();
    ex.example2();
    ex.example3();
    println!();
}


fn run_StringExample() {
    println!("<<StringExample>>");
    let ex = StringExample::M{};
    ex.example1();
    println!();
}

fn run_IOExample() {
    println!("<<IOExample>>");
    let ex = IOExample::M {};
    ex.example1();
    ex.example2();
    ex.example3();
    println!();
}


fn run_ContainerDataTypesExample() {
    println!("<<ContainerDataTypesExample>>");
    let ex = ContainerDataTypesExample::M {};
    ex.arrayExample();
    ex.vectorExample();
    ex.tupleExample();
    println!();
}


fn run_FunctionExample() {
    println!("<<FunctionExample>>");
    let ex = FunctionExample::M  {};
    ex.example1();
    ex.example2();
    ex.example3();
    println!();
}


fn run_StructExample() {
    println!("<<StructExample>>");
    let ex = StructExample::M {};
    ex.example1();
    ex.example2();
    println!();
}


fn run_TraitExample() {
    println!("<<TraitExample>>");
    let ex = TraitExample::M {};
    ex.example1();
    println!();
}


fn run_EnumExample()  {
    println!("<<EnumExample>>");
    let ex = EnumExample::M {};
    ex.example1();
    println!();
}

/*
 I could not use `mod SimpleMath ` in files , which did not contain "main function"
*/
fn run_ModuleExample() {
    println!("<<ModuleExample>>");
    println!("<<example1()>>");
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
    println!();
}
