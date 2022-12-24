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

    let num = input;                // input keybord
    let mut prime = vec![];         // ว่างๆไว้เก็บจำนวนเฉพาะ 1 ถึง ตัวมันเอง
    let mut x;                      // ไว้คำนวนตอนท้ายว่ามีเลขไรหารได้บ้าง
    let mut counter;                // ไว้นับ loop      start = 0
    let mut checkprime:bool = true; // ไว้เช็คเลขเฉพาะ

    // วนหาเลขเฉพาะตั้งแต่ 1 ถึง input
    for i in 1..num+1 {
        if i == 0 || i ==1 {
            checkprime = false;
        }
        for j in 2..i{
            if i % j == 0 {
                checkprime = false;
                break;
            }
        }
        if checkprime {
            prime.push(i);              // เป็นเลขเฉพะ เพิ่มเข้าไปใน ตัวแปร prime
        }
        checkprime = true;
    }
    // เลขเฉพาะทั้งหมด
    //println!("prime = {:?}", prime);

    let mut s = String::new();      // กล่องข้อความ
    let _s_2 = String::new();       // กล่องข้อความ ไว้รับตัวแปร prime


    // ตัวแปรที่ใช้ คำนวน
    counter = 0;
    x = num;

    // วนหาตัวคูณที่ใช้จริงๆ ในเลขเฉพาะทั้งหมด
    while counter < prime.len(){
        if prime[counter] < x  || x != 0{
            while x % prime[counter] == 0{
                x = x / prime[counter];
                let _s_2 = prime[counter].to_string();
                s.push_str(&_s_2.to_string());
                s.push_str("*");     
            } 
        } 
        else {
            break;
        }              
        counter += 1;
    }
    
    s.pop();
    println!("{} = {}",num,s);
}
