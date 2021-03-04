
const MAX_POINTS : u32 = 100_000;

fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
    println!("The value of MAX_POINTS is {}", MAX_POINTS);
    shadow();
    let_mut();
}


fn shadow() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is {}", x);
}

fn let_mut() {
    let spaces = "    ";
    let spaces = spaces.len();
    println!("The len of spaces is {}", spaces);

    //error
    //let mut spaces = "   ";
    //spaces  = spaces.len();
}