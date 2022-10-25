use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("猜数游戏！");
    let secret_number = rand::thread_rng().gen_range(1..101);    
    println!("数字是{}",secret_number);
    loop{
        println!("猜测一个数");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("无法读取行");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜测的数是:{}",guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("猜小了"),
            Ordering::Greater => println!("猜大了"),
            Ordering::Equal => {
                println!("猜对了！");
                break;
            }
        }

    }
}
