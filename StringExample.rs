pub struct M {}

impl M {
    pub fn example1(&self) {
        println!("<<example1()>>");
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
        println!();
    }
}
