use std::io;

fn main() {
    // Read a line of input from the user
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Trim the input and parse it as an integer
    let input: i32 = input.trim().parse().expect("Enable number : ");

    // Use the input value
    println!("You entered: {}", input);
    println!("-----------------------------------------------------------------------");

    let num = input;
    let mut vec = vec![0, 1, 1];
    let mut x = 2;
    let mut counter = 2;             // start = 0

    //x = arr[arr.len()-1] + arr[arr.len()-2];
    //println!("x = {}",x);
    loop{
        x = vec[vec.len()-1] + vec[vec.len()-2];
        
        if x > num{
            break;
        }
        vec.push(x);
        counter += 1 ;
    }
    
    //println!("counter = {}",counter);
    //println!("vec = {:?}", vec);
    
    let mut vec2 = vec![1];
    let mut s = String::new();      // กล่องข้อความ
    for i in vec.iter() {
        if i < &2{
            continue;
        }
        
        //s.push_str();
        //vec2.push(*i);
        print!("{}", i);
        print!(" ");
    }
    println!("{}",s);

}
