use std::io;//预导入
use std::cmp::Ordering;
use rand::Rng;//引入rand随机数，rng相当于接口

fn main() {
    println!("猜数字游戏");
    let  secret_number = rand::thread_rng().gen_range(1,101);//1代表最小值，生成的数中包含它，101为最大值，生成的数中不包含它，生成的数是1-100的任意一个
    
    loop{//loop,对以下代码无限循环
        println!("猜测一个数字");
        let mut guess = String::new();      
        io::stdin().read_line(&mut guess).expect("无法读取行");//随用户输入而修改guess的值
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("你猜测的数字是：{}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("你猜的太小了！"),
            Ordering::Greater => println!("你猜的太大了！"),
            Ordering::Equal => {
                println!("恭喜，你猜对了！！");
                break;
            }//break,当用户输入的数字与生成的随机数相匹配，跳出循环，程序终止
        }//比较用户输入的数字与生成的随机数，并进行反馈
    }


}