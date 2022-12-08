fn main() {
    // conversions();
    // concatenate_strings();
    concatenate_strings_in_a_loop();
}

fn conversions() {
    let num_str = "5";

    // str to i32
    let parsed: i32 = num_str.parse().unwrap();

    //  str to i32 turbofish
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}

fn concatenate_strings() -> String {
    let s = "hello";
    let mut cs = String::from(s);
    cs.push_str(" world");
    cs.push('!');
    println!("{}", cs);
    return cs;
}

fn concatenate_strings_in_a_loop() -> String {
    let mut s = String::from("Hello");
    for el in [", ", "world", "!"] {
        s.push_str(el);
    }
    println!("{s}");
    return s;
}

fn concatenate_strings_in_a_loop2() -> String {
    let mut acc = String::new();
    let others = [", ", "world", "!"];

    for el in others {
        acc.push_str(el);
    }
    println!("{acc}");
    return acc;
}
