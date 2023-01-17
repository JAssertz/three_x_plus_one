fn main() {
    println!("The solution was {}", find_solution())
}

fn find_solution() -> i128 {
    let mut last_known_million: i128 = 0;
    let mut active_starting_num: i128 = 2;
    #[warn(while_true)]
    loop {
        
        let mut active_number: i128 = active_starting_num;

        while active_number != 1 {
            
            if active_number == 0 {
                println!("Success found the real number { }", active_number);
                return active_number;
            }

            if active_number % 2 == 0 {
                active_number = active_number / 2;
            } else {
                active_number = (active_number * 3) + 1;
            }
            
        }
        active_starting_num += 1;
        if active_starting_num == last_known_million + 1000000 {
            println!("Passed {}", active_starting_num);
            last_known_million = active_starting_num;
        }
    }
}



