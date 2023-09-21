mod selection;
use selection::selection;

mod double_selection;
use double_selection::double_selection;

mod insertion;
use insertion::insertion;

mod bubble;
use bubble::bubble;

mod quick;
use quick::quick;

mod merge;
use merge::merge;

fn main() {
    // let mut arr = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    // let mut arr = vec![
    // 1, 6586, 145, 463, 14, 574, 169, 461, 69875, 4563, 4, 58, 46358, 43, 5,
    // ];
    // let mut arr = vec![1, 9, 2, 8, 3, 7, 4, 6, 5];
    let mut arr = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];

    let mut arrr = arr.clone();
    println!("Original:- \n{arrr:?}\n");
    println!("Rust:-");
    arr.sort_unstable();
    println!("{arr:?}\n");

    run_selection(&mut arrr);

    run_double_selection(&mut arrr);

    run_insertion(&mut arr);

    run_bubble(&mut arrr);

    run_quick(&mut arrr);

    run_merge(&mut arr);
}

fn run_selection<T>(arr: &mut [T])
where
    T: PartialOrd + Clone + std::fmt::Debug,
{
    let mut arr = (*arr).to_vec();
    println!("Selection:-");
    selection(&mut arr);
    println!("{arr:?}\n");
}

fn run_double_selection<T>(arr: &mut [T])
where
    T: PartialOrd + Clone + std::fmt::Debug,
{
    let mut arr = (*arr).to_vec();
    println!("Double Selection:-");
    double_selection(&mut arr);
    println!("{arr:?}\n");
}

fn run_insertion<T>(arr: &mut [T])
where
    T: PartialOrd + Clone + std::fmt::Debug,
{
    let mut arr = (*arr).to_vec();
    println!("Insertion:-");
    insertion(&mut arr);
    println!("{arr:?}\n");
}

fn run_bubble<T>(arr: &mut [T])
where
    T: PartialOrd + Clone + std::fmt::Debug,
{
    let mut arr = (*arr).to_vec();
    println!("Bubble:-");
    bubble(&mut arr);
    println!("{arr:?}\n");
}

fn run_quick<T>(arr: &mut [T])
where
    T: PartialOrd + Clone + std::fmt::Debug,
{
    let mut arr = (*arr).to_vec();
    println!("Bubble:-");
    quick(&mut arr);
    println!("{arr:?}\n");
}

fn run_merge<T>(arr: &mut [T])
where
    T: PartialOrd + Clone + std::fmt::Debug,
{
    let mut arr = (*arr).to_vec();
    println!("Bubble:-");
    merge(&mut arr);
    println!("{arr:?}\n");
}
