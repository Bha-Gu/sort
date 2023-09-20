mod selection;
use selection::selection;

mod double_selection;

mod bubble;
use bubble::bubble;

mod quick;
use quick::quick;

use crate::double_selection::double_selection;

fn main() {
    let mut arr = vec![
        1, 6586, 145, 463, 14, 574, 169, 461, 69875, 4563, 4, 58, 46358, 43, 5,
    ];
    // let mut arr = vec![1, 9, 2, 8, 3, 7, 4, 6, 5];
    let mut arr_selection = arr.clone();
    let mut arr_double_selection = arr.clone();
    let mut arr_bubble = arr.clone();
    let mut arr_quick = arr.clone();
    let arrr = arr.clone();
    println!("Original:- \n{arrr:?}\n");
    println!("Rust:-");
    arr.sort_unstable();
    println!("{arr:?}\n");

    println!("Selection:-");
    selection(&mut arr_selection);
    println!("{arr_selection:?}\n");

    println!("Double Selection:-");
    double_selection(&mut arr_double_selection);
    println!("{arr_double_selection:?}\n");

    println!("Bubble:-");
    bubble(&mut arr_bubble);
    println!("{arr_bubble:?}\n");

    println!("Quick:-");
    quick(&mut arr_quick);
    println!("{arr_quick:?}\n");
}
