const L: i32 = 10;

fn deduct(a: &mut i32) -> () {
    *a = *a - 1;
    println!("{}", a);
}

fn main() {
    let mut n: i32 = L;

    deduct(&mut n);

   println!("{}", n);
    
    println!("Hello, world!");
}
