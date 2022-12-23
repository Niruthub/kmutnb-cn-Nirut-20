fn main() {
    // ค่าอินพุต
    let num: u32 = 10;
    // x วนลูปแถว counter วนลูปหลัก
    let mut x = 0;
    let mut counter;
    
    loop{
        counter = 0;
        x+=1;
        while counter <  (num*2-x){
            counter+=1;
           if(counter<x) || (counter>num*2-x) {
                print!("  ");
            }
            else{
                print!("* ");
            }      
        }
        println!("");
        
        if x == num {
            break;
        }
    }
}
