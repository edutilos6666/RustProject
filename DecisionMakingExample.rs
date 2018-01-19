pub struct M {}

impl M  {
    pub fn example1(&self) {
        println!("<<example1()>>");
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
        println!();
    }

    pub fn example2(&self) {
        println!("<<example2()>>");
        let age = 10 ;
        if (age > 0 ) &&  (age < 10) {
            println!("You are a kid.");
        } else if (age >= 10 ) && (age < 20) {
            println!("You are a teenager.");
        } else if (age >= 20) && (age < 50) {
            println!("You are an adult.");
        } else {
            println!("You are an elderly."); 
        }
        println!();
    }
}
