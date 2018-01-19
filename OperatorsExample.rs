pub struct M {}


impl M {
    pub fn test_ArithmeticOps(&self) {
        let mut x = 10i64;
        let mut y = 3i64 ;
        let add = x + y;
        let sub = x - y ;
        let mult = x * y ;
        let div = x / y;
        let modulo = x % y ;
        println!("<<Arithmetic Operators>>");
        println!("x = {} and y = {}", x, y);
        println!("x + y = {}", add);
        println!("x - y = {}", sub);
        println!("x * y = {}", mult);
        println!("x / y = {}", div);
        println!("x % y= {}", modulo);
        println!();
    }

    pub fn test_RelationalOps(&self) {
        let mut x = 10i32;
        let mut y = 3i32;
        let eq = x == y ;
        let ne = x != y ;
        let gt = x > y ;
        let gte = x >= y ;
        let lt = x < y ;
        let lte = x <= y ;
        println!("<<Relational Operators>>");
        println!("x = {} and y = {}", x, y);
        println!("x == y = {}", eq);
        println!("x != y = {}", ne);
        println!("x > y = {}", gt);
        println!("x >= y = {}", gte);
        println!("x < y = {}", lt);
        println!("y <= y = {}", lte);
        println!();
    }

    pub fn test_LogicalOps(&self) {
        let  mut x = true ;
        let mut y = false ;
        let and = x && y ;
        let or = x || y ;
        let not_x = !x ;
        let not_y = !y ;
        println!("<<Logical Operators>>");
        println!("x = {} and y = {}", x, y);
        println!("x && y = {}", and);
        println!("x || y = {}", or);
        println!("!x = {}", not_x);
        println!("!y = {}", not_y);
        println!();
    }

    pub fn test_BitwiseOps(&self) {
        let mut x = 10;
        let mut y = 23;
        let band = x & y ;
        let bor = x | y ;
        let bxor = x ^ y;
        let x_ls_2 = x << 2;
        let x_rs_2 = x >> 2;
        let not_x = !x ;
        let not_y = !y ;
        println!("<<Bitwise Operators>>");
        println!("x = {} and y = {}", x, y);
        println!("x & y = {}", band);
        println!("x | y = {}", bor);
        println!("x ^ y = {}", bxor);
        println!("x << 2 = {}", x_ls_2);
        println!("x >> 2 = {}", x_rs_2);
        println!("!x = {}", not_x);
        println!("!y = {}", not_y);
        println!();
    }


    pub fn test_MathFunctions(&self) {
        println!("<<Math Functions>>");
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
        println!(); 
    }

}
