mod bubble;
use bubble::bubble;

mod quick;
use quick::*;

fn main() {
    let mut arr = vec![
        1, 6586, 145, 463, 14, 574, 169, 461, 69875, 4563, 4, 58, 46358, 43, 5,
    ];
    let mut arrb = arr.clone();
    let mut arrq = arr.clone();
    let arrr = arr.clone();
    println!("Original:- \n{:?}\n", arrr);
    println!("Rust:-");
    arr.sort();
    println!("{:?}\n", arr);
    println!("Bubble:-");
    bubble(&mut arrb);
    println!("{:?}\n", arrb);
    println!("Quick:-");
    quick(&mut arrq);
    println!("{:?}\n", arrq);
}
