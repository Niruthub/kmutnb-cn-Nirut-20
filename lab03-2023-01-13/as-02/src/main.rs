fn main() {
    // A
    let smash = 1;

    // step 1 s(2)
    let smash = sum(smash+smash); // s = 3

    // step 2 s(3)
    let smash = sum(smash); // s = 6
  
    // step 3 s(4)
    let smash = sum(4); // s = 10

    //B s(100)
    let smash = sum(100);
    println!("{}",smash);
}

fn sum(_s:u32) -> u32{
    if _s==1 {
        return _s;
    }
    return sum(_s-1)+_s;
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
    fn sum_of_number() {
        let _s = 5;
        let result =sum(_s); // result = 1+2+3+4+5 = 15
        assert_eq!(result, 15);
    }
}