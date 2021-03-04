fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
    //string_assign();
    //string_clone();

    let s = String::from("magisk");
    //takes_ownership(s); // 所有权发生转移
    // println!("{}", s); error
    
    takes_ownership(s.clone());
    println!("{}", s);

    let s : String = give_ownership();
    let s : String = take_and_give_back(s);
    println!("{}", s);

    let (s, l) = calculate_length(s);
    println!("String {}'s length is {}", s, l);
}

/*
 * 简单的数据结构没有必要进行浅拷贝，因为它们足够的简单，最浅的拷贝也是完全拷贝，速度也很快，
 * 所以没有必要去区分浅拷贝和深拷贝
 * 对于一些复杂的数据结构，浅拷贝可以加快赋值的速度
 * 某种类型拥有了Copy这种trait,那么它的变量就可以在赋值给其他变量之后保持可用性。
 * 如果一种类型本身或这种类型的任意成员实现了Drop这种trait,那么Rust就不允许其实现Copy这种trait
 * Drop 你可以理解为析构函数，Copy 你可以理解为拷贝构造函数（C++ 中的拷贝构造函数你可以自己定义拷贝的方式，灵活性很高，
 * 因此也很容易出错）
*/

fn string_assign() {
    let s1 : String = String::from("Hello");
    let mut s2 = s1;
    s2.push_str(", World!");
    // 首先 s1 和 s2 是栈上面的内存，但是其中存储的字符串在堆上面。
    // rust 为了防止在 s1 和 s2 出作用域的时候，对堆上的空间进行两次
    // 重复的释放，会认为第一个 s1 废弃，即，它不再是一个有效的变量。
    // 虽然通过 lldb 调试的时候还可以看到这个变量依然指向堆空间，但是从语法层面上，
    // 他已经被废弃了
    // 上面的这种所有权的转移的过程，也可以类似于 C++ 中的转移语义
    // println!("{}", s1); error
    println!("{}", s2);
}

fn string_clone() {
    let mut s1 = String::from("Hello");
    let s2 = s1.clone();
    s1.push_str(", World!");

    println! ("s1 = {}, s2 = {}", s1, s2);
}

fn takes_ownership(some_string:String) {
    let s = some_string;
    println!("{}", s);
}

fn give_ownership() -> String {
    let some_string = String::from("LSPosed");
    some_string
}


fn take_and_give_back(a_string : String) -> String {
    a_string
} 


// 这种写法未免太过笨拙了
fn calculate_length(s : String) -> (String, usize) {
    let length : usize = s.len();

    (s, length)
}