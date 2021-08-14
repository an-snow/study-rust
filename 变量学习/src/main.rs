fn main() {
    let mut x = 5;//给x赋值5 添加mut使得此变量可变
    println!("The value of x is {}", x);//输出x的值，大括号代表占位符，输出时替换为后面的x的值

    x =6;
    println!("The value of x is {}", x);
}