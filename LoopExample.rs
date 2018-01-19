pub struct M {}

impl M {
    pub fn example1(&self) {
        println!("<<example1()>>");
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
        println!();
    }

    pub fn example2(&self) {
        println!("<<example2()>>");
        let mut y = 1 ;
        while y <= 10 {
            println!("WHILE : {}", y);
            y += 1 ;
        }
        println!();
    }

    pub fn example3(&self) {
        println!("<<example3()>>");
        for z in 1..10 {
            println!("FOR : {}", z);
        }
        println!();
    }
}
