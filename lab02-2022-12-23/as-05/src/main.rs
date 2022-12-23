use std::io;

fn main() {
    // Read a line of input from the user
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Trim the input and parse it as an integer
    let input: i32 = input.trim().parse().expect("Input y = ");

    // Use the input value
    //println!("You entered: {}", input);

    // ค่าอินพุต
    let num = input;

    // i วนลูปแถว j วนลูปหลัก
    let mut i = 0;
    let mut j;

    loop{
        j = 0;
        i += 1;
        while j < num{
            j += 1;
            if(j==1) || (j==num) || (j==i) {
                print!("X ");
            }
            else{
                print!("O ");
            }
        }
        println!("");
        if i >= num {
            break;
        }
    }
}