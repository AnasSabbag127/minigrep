use std::env;

fn main() {
    println!("Hello, world!");

    command_line_without_argument();
    command_line_with_argument();

    

}

fn command_line_without_argument()
{
    let args: Vec<String> = env::args().collect();
    dbg!(args);

}

fn command_line_with_argument()
{
    let args:Vec<String> = env::args().collect();

    let first_arg=&args[1];
    let second_arg=&args[2];
    println!("first arg : {} \nsecond arg : {}",first_arg,second_arg);
    
}