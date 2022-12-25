use std::io;

fn main() {
    // Read a line of input from the user
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Trim the input and parse it as an integer
    let input: i32 = input.trim().parse().expect("Enable number : ");

    // Use the input valueclear
    println!("You entered: {}", input);
    println!("-----------------------------------------------------------------------");

    let num = input;
    let mut prime = vec![0, 1, 1];
    let mut x = 2;
    let mut counter = 2;             // start = 0

    if num < 2 {
        for i in 0..2-num{
            prime.pop();
        }
    }
    else{
        loop{
            x = prime[prime.len()-1] + prime[prime.len()-2];
            
            if counter >= num{
                break;
            }
            prime.push(x);
            counter += 1 ;
        }
    }

    println!("Fibonacci = {:?}", prime);
  

}
