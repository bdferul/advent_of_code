#![feature(result_option_inspect)]

const INPUT: &str = include_str!("123.txt");

fn main() {
    let p1 = INPUT.lines().map(part1).sum::<u64>();
    let p2 = INPUT.lines().map(part2).map(|x| part1(&x)).sum::<u64>();

    println!("Part 1: {p1}\nPart 2: {p2}");
}

fn part1(s: &str) -> u64 {
    let mut iter = s.chars().filter(|x| x.is_digit(10));
    let a = iter.next().unwrap_or('0');
    let b = iter.last().unwrap_or(a);

    let mut r = String::new();
    r.push(a);
    r.push(b);
    println!("{r}");
    r.parse().unwrap()
}

fn part2(s: &str) -> String {
    let map = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    let mut r = String::new();

    for i in 0..s.len() {
        let c = s.chars().nth(i).unwrap();
        if c.is_digit(10) {
            r.push(c);
            continue;
        }
        for (k, v) in map {
            if (&s[i..]).starts_with(k) {
                r.push(v);
            }
        }
    }

    r
}
