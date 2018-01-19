pub struct M {}

impl M {
    pub fn arrayExample(&self) {
        println!("<<Array Example>>");
        let rand_array = [1,2,3];
        println!("{}", rand_array[0]);
        println!("{}", rand_array.len());
        //:? for printing array
        println!("Second 2 : {:?}", &rand_array[1..3]);
        println!();
    }

    pub fn vectorExample(&self) {
        println!("<<Vector Example>>");
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
        println!();
    }

    pub fn tupleExample(&self) {
        println!("<<Tuple Example>>");
        let rand_tuple = ("foobar", 40);
        let rand_tuple_2: (&str, i8) = ("foobar", 40);
        println!("Name = {}", rand_tuple_2.0);
        println!("rand_tuple = {:?}", rand_tuple);
        println!("rand_tuple_2 = {:?}", rand_tuple_2);
        println!();
    }
}
