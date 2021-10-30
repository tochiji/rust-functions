fn main() {
    let s0 = "Hello World!";
    println!("{}", s0);

    let mut s1 = String::from("hello");
    s1.push_str(", world!");
    println!("{}", s1);

    let s2 = s1.clone();
    println!("{}", s1);
    println!("{}", s2);

    let s3 = String::from("hello");
    takes_ownership(s3);
    // println!("{}", s3);

    let x0 = 5;
    makes_copy(x0);

    let s4 = String::from("hello, hello");
    let len = calculate_length(&s4);
    println!("len: {}", len);

    let mut s5 = String::from("mut test");
    change(&mut s5);
    println!("{}", s5);

    let s6 = String::from("this string to be empty");

    let word = first_word(&s6);

    println!("s6: {}", s6);
    println!("s6 first word: {}", word);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("{:?}", slice);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn calculate_length(s: &str) -> usize {
    // s.push_str(", right?"); <- NG!
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", dayo!");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
