fn main() {
    let _guess : u64  = "12132456".parse().expect("Is Not a Number!");

    //error
    //let guess =  "1123".parse().expect("Is Not a Number!");

    //几种整数值的写法
    //let number = 56u8;
    //let number = 0xffff;
    //let number = 0b1010_0101;
    //let number = b'A';

    //let mut number : u8 = 255;
    // 似乎不会再进行补码环绕了
    //number = 257;
    //println!("The value of number is: {}", number);

    let f1 = 2.0; // f64
    let f2 : f32 = 3.1415; // f32
    println!("{}, {}", f1, f2);

    let heart_eyed_cat : char = '😻';
    println!("Heart eyed cat emoji looks like : {}", heart_eyed_cat);

    tuple_and_array();
}



fn tuple_and_array() {
    let tuple : (i32, f64, u8) = (500, 3.14, 1);
    let (x, y, z) = tuple;
    println!("The value of (x, y z) is ({},{},{})", x, y, z);

    let five_hundred = tuple.0;
    let three_point_one = tuple.1;
    let one = tuple.2;
    println!("Tuple is ({}, {}, {})", five_hundred, three_point_one, one);

    let mut tuple = (500, 3.1, 1);
    tuple.1 = 2.333;
    tuple.2 = 255;
    println!("Tuple is ({}, {}, {})", tuple.0, tuple.1, tuple.2);

    //array
    //let a = [1,2,3,4,5];
    //let a : [i32; 5] = [1,2,3,4,5];
    //let a = [10; 100];

}