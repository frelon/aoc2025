fn main() {
    let mut curr = 50;

    let input = get_input();
    let mut attempts = 0;

    for line in input.lines() {
        let steps = line.replace("L", "-").replace("R", "").parse::<i32>().unwrap() % 100;

        curr += steps;
        if curr <= 0 {
            curr += 100;
        }
        if curr >= 100 {
            curr -= 100;
        }

        if curr == 0 {
            attempts+=1;
        }

        println!("line: {}, curr: {}", line, curr);
    }

    println!("attempts: {}", attempts)
}


#[cfg(feature="examples")]
fn get_input() -> &'static str {
    include_str!("example.txt")
}

#[cfg(feature="live")]
fn get_input() -> &'static str {
    include_str!("input.txt")
}
