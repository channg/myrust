fn main() {
    let mut some = true;
    let mut count = 100;
    while some {

        if(count % 40==1 ){
            some = false
        }
        count = count -1;
        //println!("{}",count)
    }


    for x in 0..10 {
        println!("{}", x); // x: i32
    }

}
