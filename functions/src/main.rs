fn main() {
    another_function(5, 6);
    expression();
    let x = five(); // 这是一个 statement，但是 five() 是一个语句
    println!("x:{}", x);
    println!("{} + 1 = {}", x, self_plus(x));
}


fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// rust 中的两个概念，statement 和 expression

fn expression() {
    let _x = 5; // statement
    let y = {   // expression
        let _x = 10;
        _x+1
    };
    println!("{}", y);  // statement
}

fn five() -> i32 {
    5
}

fn self_plus(x : i32) -> i32 {
    //x + 1;
    x + 1   // 不可以加分号，加了分号就成了 statement
}

// 注释的方式和 C 一致