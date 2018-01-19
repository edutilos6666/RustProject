pub struct M {}

impl M  {
    pub fn example1(&self) {
        println!("<<example1()>>");
        let hulk = Hero::Strong(100);
        let quicksilver = Hero::Fast;
        let spiderman = Hero::Info {name: "Spipderman".to_owned(),
          secret: "Peter Parker".to_owned() };

        get_info(hulk);
        get_info(quicksilver);
        get_info(spiderman);
        println!();
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
