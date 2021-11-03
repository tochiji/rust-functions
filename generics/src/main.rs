fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    // 最長の文字列は、{}です
    println!("The longest string is {}", result);
}

fn longest<'a>(l: &'a str, r: &'a str) -> &'a str {
    if l.len() > r.len() {
        l
    } else {
        r
    }
}
