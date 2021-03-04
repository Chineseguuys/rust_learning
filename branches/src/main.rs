fn main() {
    let number = 3;
    // rust 不会将整数值自动转化为 bool 值
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    expression();
    println!("loop_t: {}", loop_t());
    while_t();
    for_t();
}


fn expression() {
    let condition = true;
    let _v = 200;

    let number = if condition {
        _v * 10
    }
        else {
            _v * 100
    };
    println!("Number is {}", number);

/*
    let number = if condition {
        _v * 100
    } else {
        "six"   // error 数据结构要匹配
    };
*/
    let number = if condition {
        "_v * 100;"
    }else {
        "_v * 10"
    };
    println!("Number is {}", number);
}

fn loop_t() -> i32 {
    let mut counter = 0;
    let res = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
            // break counter * 2  这里加分号和不加分号都可以
        }
    };
    res
}

fn while_t(){
    let array = [10; 50];
    let mut index = 0;

    while index < 50 {
        println!("array[{}] = {}", index, array[index]);
        index = index + 1;
    }
}

fn for_t() {
    let array = [10, 20, 30, 40, 50];

    for element in array.iter() {
        println! ("{}", element);
    }

    for number in (1..4).rev() {
        println! ("{}!", number);
    }
}