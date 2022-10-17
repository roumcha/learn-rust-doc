use std::io;

fn main() {
    christmas();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to get str!");
    let input: u32 = input.trim().parse().expect("Input must be an integer!");

    println!("{}", fib(input));
}

fn _c_to_f(deg_c: f64) -> f64 {
    (deg_c * 9. / 5.) + 32.
}

fn _f_to_c(def_f: f64) -> f64 {
    (def_f - 32.) * 5. / 9.
}

fn fib(n: u32) -> u32 {
    if n == 0 {
        panic!("Invalid input for fib!");
    } else if n == 1 {
        1
    } else {
        let mut prev = 1;
        let mut cur = 1;
        for _ in 2..n {
            (prev, cur) = (cur, prev + cur);
        }
        cur
    }
}

fn christmas() {
    let ord_num = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let cnt_num = [
        "a", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven",
        "twelve",
    ];
    let item = [
        "partridge in a pear tree",
        "turtle doves",
        "French hens",
        "calling birds",
        "golden rings",
        "geese a-laying",
        "swans a-swimming",
        "maids a-milking",
        "ladies dancing",
        "lords a-leaping",
        "pipers piping",
        "drummers drumming",
    ];

    fn pascal(s: &str) -> String {
        let head = &s[0..1];
        let tail = &s[1..];
        head.to_uppercase() + tail
    }

    for i in 0..12 {
        println!(
            "On the {} day of Christmas, my true love sent to me",
            ord_num[i]
        );

        if i == 0 {
            println!("{} {}\n", pascal(cnt_num[0]), item[0]);
            continue;
        }

        for j in (1..i + 1).rev() {
            println!(
                "{} {}{}",
                pascal(cnt_num[j]),
                item[j],
                if j != i - 1 { "," } else { "" }
            );
        }

        println!("And {} {}.\n", cnt_num[0], item[0]);
    }
}
