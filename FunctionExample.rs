pub struct M {}



impl M {
    pub fn example1(&self) {
        println!("<<example1()>>");
        say_hello("edutilos");
        println!("5 + 4 = {}", get_sum(5, 4));
        let sum = get_sum ;
        println!("6 +4 = {}", sum(6,4));
        println!();
    }

    pub fn example2(&self) {
        //closures
        let sum_nums = |x : i32 , y: i32| x + y;
        println!("7 + 8 = {}", sum_nums(7,8));
        let num_ten = 10 ;
        let add_10 = |x: i32| x + num_ten;
        println!("5 + 10 = {}", add_10(5));
        println!();
    }

    pub fn example3(&self) {
        println!("<<example3()>>");
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

        println!();
    }
}


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
