fn main() {
    println!("Part1: {}", part1());
    println!("Part2: {}", part2());
}

fn part1() -> u32 {
    let input = get_input();

    let mut total_joltage = 0;

    for line in input.lines() {
        let bank = line.chars().map(|x| x.to_digit(10).unwrap());
        let mut battery1 = 0;
        let mut battery2 = 0;
        let mut count = 0;
        
        for battery in bank {
            count += 1;

            if battery > battery1 && !(count == line.len()) {
                battery1 = battery;
                battery2 = 0;
            } else if battery > battery2 {
                battery2 = battery; 
            }

            println!("Battery {}, ({}, {})", battery, battery1, battery2);
        }

        println!("{}: {} + {} = {}", line, battery1, battery2, (battery1*10)+battery2);
        total_joltage += (battery1*10) + battery2;
    }

    total_joltage
}

fn part2() -> u32 {
    let input = get_input();

    let mut total_joltage = 0;

    for line in input.lines() {
        let bank = line.chars().map(|x| x.to_digit(10).unwrap());
        let mut batteries : [u32; 12] = [0;12];
        let mut count = 0;
        
        // for next in bank {
        //     for 
        // }

        // println!("{}: {} + {} = {}", line, battery1, battery2, (battery1*10)+battery2);
        // total_joltage += (battery1*10) + battery2;
    }

    total_joltage
}

fn get_input() -> &'static str {
    if cfg!(feature="examples") {
        include_str!("example.txt")
    } else {
        include_str!("input.txt")
    }
}
