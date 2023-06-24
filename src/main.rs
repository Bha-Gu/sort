mod bubble;
use bubble::bubble;

mod quick;
use quick::quick;

fn main() {
    let mut arr = vec![
        1, 6586, 145, 463, 14, 574, 169, 461, 69875, 4563, 4, 58, 46358, 43, 5,
    ];
    let mut arr_binary = arr.clone();
    let mut arr_quick = arr.clone();
    let arrr = arr.clone();
    println!("Original:- \n{arrr:?}\n");
    println!("Rust:-");
    arr.sort_unstable();
    println!("{arr:?}\n");
    println!("Bubble:-");
    bubble(&mut arr_binary);
    println!("{arr_binary:?}\n");
    println!("Quick:-");
    quick(&mut arr_quick);
    println!("{arr_quick:?}\n");
}
