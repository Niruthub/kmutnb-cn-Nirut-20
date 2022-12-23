fn main(){
    //รับข้อความจาก คียบอร์ด ใส่ตัวแปร
    let num: u32 = 5;
    // ตัวนับ
    let mut counter = 0;
    let mut kai;
    //ตัวข้อความ
    let mut s = String::new();

    println!("num = {}",num);
    let x = loop {                          // วนลูปในแต่ละแถว (ไล่ลงไป)
        kai = 0;                        // รีเซ็ต ลูปใน
        s = "".to_string();             // รีเซ็ตกล่องข้อความ
        counter+=1;                         // 
        while counter != kai{           // วนลูปในแต่ละหลัก (ไปทางขวา)
            kai+=1;
            s.push_str("* ");           // ต่อข้อความ
            
        }
        //println!("counter = {}",counter);
        if num < counter{
            break counter;
        }
        println!("{}",s);
    };
    
    //println!("outside loop x value is {}",x);
    //println!("num = {}",num);
 }