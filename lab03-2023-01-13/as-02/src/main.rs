fn main() {
    // A
    let s = 1;

    // step 1 s(2)
    let s = sum(s+s); // s = 3

    // step 2 s(3)
    let s = sum(s); // s = 6
  
    // step 3 s(4)
    let s = sum(4); // s = 10

    //B s(100)
    let s = sum(100);
    println!("{}",s);
}

fn sum(s:u32) -> u32{
    if s==1 {
        return s;
    }
    return sum(s-1)+s;
}
/*
fn sum(mut s:u32) -> u32{
    let mut i:u32 = 0;
    let stop = s;
    s = 0;
    while i<stop {
        s = s + i + 1;
        i += 1;
    }
    s
}*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_unique_work() {
        let s = 5;
        let result =sum(5); // result = 1+2+3+4+5 = 15
        assert_eq!(result, 15);
    }
}