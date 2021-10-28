fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = x + 1;
    println!("The value of x is: {}", x);

    // let mut u8: u8 = 1;
    // u8 = 10000000;
    //    literal out of range for `u8`
    //    `#[deny(overflowing_literals)]` on by default
    //    the literal `10000000` does not fit into the type `u8`
    //    whose range is `0..=255`rustc(overflowing_literals)

    let guess: u32 = "42".parse().expect("Not a number");
    println!("{}", guess);

    let mut minus = 100;
    println!("{}", minus);
    minus = -50;
    println!("{}", minus);

    let diff = 95.5 - 4.3;
    println!("95.5 - 4.3 = {}", diff);

    let quotient = 56.7 / 32.2;
    println!("56.7 / 32.2 = {}", quotient);

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻'; //ハート目の猫
    println!("{} {} {}", c, z, heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{{:?}}: {:?}", tup);
    println!("{{}} {{}} {{}}: {} {} {}", tup.0, tup.1, tup.2);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);

    let a = [1, 2, 3, 4, 5];
    println!("{:?}", a);
    println!("{:#?}", a);
    println!("{}", a[2]);

    // let index = 10;
    // println!("{}", a[index]);
    //
    //    |
    // 45 |     println!("{}", a[index]);
    //    |                    ^^^^^^^^ index out of bounds: the length is 5 but the index is 10
}
