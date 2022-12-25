fn main() {
    // input
    //let mut array = [1,2,0,4,3,0,5,0];
    //let mut array = [1,2,0,0,0,3,6];
    let mut array = [0,9,0,4,3,0,0,0,5,0];

    // clone
    let mut array2 = array.clone();

    // ไว้จำตำแหน่งย้ายข้อมูล
    let mut mov = 0;

    println!("array = {:?}",array);
    for i in 0..array2.len(){
            if array2[i] != 0{
                array[mov] = array2[i];
            }
            else{
                continue;
            }
        mov += 1;

    }
    for j in mov..array.len(){
            array[j] = 0;
    }
    println!("array new = {:?}",array);

   
}
