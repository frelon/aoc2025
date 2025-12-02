fn main() {
    println!("Part1: {}", part1());
    println!("Part2: {}", part2());
}

fn part1() -> u64 {
    let product_ranges = get_input().trim().split(",");

    let mut total = 0;

    for range in product_ranges {
        let s : Vec<&str> = range.split("-").collect();
        let start = s[0].parse::<u64>().unwrap();
        let end = s[1].parse::<u64>().unwrap();

        for id in start..=end {
            if !is_valid(&id.to_string()) {
                total += id;
            }
        }
    }
    
    total
}

fn part2() -> u64 {
    let product_ranges = get_input().trim().split(",");

    let mut total = 0;

    for range in product_ranges {
        let s : Vec<&str> = range.split("-").collect();
        let start = s[0].parse::<u64>().unwrap();
        let end = s[1].parse::<u64>().unwrap();

        for id in start..=end {
            if !is_valid2(&id.to_string()) {
                total += id;
            }
        }
    }
    
    total
}

fn is_valid(id :&str) -> bool {
    let split = id.split_at(id.len()/2);
    split.0 != split.1
}

fn is_valid2(id :&str) -> bool {
    for n in 1..id.len() {
        let mut s = true;
        let v = id.chars().collect::<Vec<char>>();
        let mut chunks = v.chunks(n);
        let first = chunks.next().unwrap();

        for chunk in chunks.clone() {
            if !same(first, chunk) {
                s = false
            }
        }

        if s {
            return false;
        }
    }

    true
}

fn same(a: &[char], b : &[char]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    for (n, m) in a.into_iter().zip(b) {
        if n != m {
            return false;
        }
    }

    true
}

fn get_input() -> &'static str {
    if cfg!(feature="examples") {
        include_str!("example.txt")
    } else {
        include_str!("input.txt")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        assert_eq!(is_valid("12"), true);
        assert_eq!(is_valid("1213"), true);
    }

    #[test]
    fn test_invalid() {
        assert_eq!(is_valid("55"), false);
        assert_eq!(is_valid("6464"), false);
        assert_eq!(is_valid("1010"), false);
    }

    #[test]
    fn test_same() {
        assert_eq!(same(&['a'], &['a']), true);
        assert_eq!(same(&['a'], &['b']), false);
        assert_eq!(same(&['2', '1'], &['2', '1']), true);
        assert_eq!(same(&['2', '1', '1'], &['2', '1']), false);
    }

    #[test]
    fn test_invalid2() {
        assert_eq!(is_valid2("55"), false);
        assert_eq!(is_valid2("6464"), false);
        assert_eq!(is_valid2("1010"), false);

        assert_eq!(is_valid2("12341234"), false);
        assert_eq!(is_valid2("123123123"), false);
        assert_eq!(is_valid2("1212121212"), false);
        assert_eq!(is_valid2("1111111"), false);
    }
}
