use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    //println!("The secret number is : {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // mut 表示的是 mutable ,即可变的，表示这个变量是可变的
        // 默认的情况下，所有的变量都是不可变的
        io::stdin().read_line(&mut guess).expect("Failed to read line");
    
        //let guess : u32 = guess.trim().parse().expect("please type a number!");

        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("you should input a number!");
                continue;
            },
        };
    
        println!("Your guessed : {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

// 使用 cargo doc 可以在本地为使用的包建立一个离线文档，这些文档从网络上下载

// 使用 cargo doc --open 可以在浏览器中查看这些离线文档，你可以通过文档了解
// 你使用的包的详细信息