fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s2 : String = String::from("magisk-");
    change_mutable(&mut s2); // 这里的 mut 也是不可以少的
    println!("{}", s2);

    /*
     * 但可变引用在使用上有一个很大的限制:对于特定作用域中的特定数据来说,一
     * 次只能声明一个可变引用。以下代码尝试违背这一限制,则会导致编译错误:
     */
    let _r1 =  &mut s2;
    //let _r2 =  &mut s2; error
    //println!("{}, {}", _r1, _r2);

    //通过花括号来创建一个新的作用域范围
    {
        let _r2 = &mut s2;
    }
    let _r3 = &mut s2;


    {
        let mut s3 = String::from("xposed");
        let _r4 = &s3;
        let _r5 = &s3;
        //let _r6 = &mut s3; error
        // 可变的引用和不可变的引用不可以在同一个作用域下面进行同时声明，
        /*
         * 我们不能在拥有不可变引用的同时创建可变引用。不可变引用的用户可不会希望他们眼皮底下的值突然发生变化!
         * 不过,同时存在多个不可变引用是合理合法的,对数据的只读操作不会影响到其他读取数据的用户。
         */
        println!("{}, {}", _r4, _r5);
    }
}


fn calculate_length(s : &String) -> usize {
    s.len()
}

// 与变量类似,引用是默认不可变的,Rust不允许我们去修改引用指向的值。
//fn change(some_string : &String) {
//    some_string.push_str("magisk");
//}


// 可变引用
fn change_mutable(some_string : &mut String) {
    some_string.push_str("EDxposed");
}