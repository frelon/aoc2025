fn day1() -> u32 {
    let input = get_input();
    let mut zeroes = 0;
    let mut dial = 50;

    for line in input.lines() {
        let lr = line.chars().nth(0).unwrap();
        let steps = line.replace("R", "").replace("L", "").parse::<u32>().unwrap();

        for n in 0..steps {
            match lr {
                'L' => { 
                    dial-=1;
                    if dial < 0 {
                        dial+=100;
                    }
                },
                'R' => {
                    dial+=1;
                    if dial >= 100 {
                        dial-=100;
                    }
                },
                _ => unreachable!(),
            } 

        }
        
        if dial == 0 {
            zeroes+=1;
        }

        println!("line: {}, dial: {}, zeroes: {}", line, dial, zeroes)
    }

    zeroes
}
fn day2() -> u32 {
    let input = get_input();
    let mut zeroes = 0;
    let mut dial = 50;

    for line in input.lines() {
        let lr = line.chars().nth(0).unwrap();
        let steps = line.replace("R", "").replace("L", "").parse::<u32>().unwrap();

        for n in 0..steps {
            match lr {
                'L' => { 
                    dial-=1;
                    if dial < 0 {
                        dial+=100;
                    }
                },
                'R' => {
                    dial+=1;
                    if dial >= 100 {
                        dial-=100;
                    }
                },
                _ => unreachable!(),
            } 

            if dial == 0 {
                zeroes+=1;
            }
        }

        println!("line: {}, dial: {}, zeroes: {}", line, dial, zeroes)
    }

    zeroes
}

fn main() {
    println!("Day1: {}", day1());
    println!("Day2: {}", day2());
}

fn get_input() -> &'static str {
    if cfg!(feature="examples") {
        include_str!("example.txt")
    } else {
        include_str!("input.txt")
    }
}
