mod selection;
use selection::selection;

mod double_selection;
use double_selection::double_selection;

mod insertion;
use insertion::insertion;

mod binary_insertion;
use binary_insertion::binary_insertion;

mod bubble;
use bubble::bubble;

mod shaker;
use shaker::shaker;

mod quick;
use quick::quick;

mod merge;
use merge::merge;

mod heap;
use heap::heap;

fn main() {
    // let mut arr = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    // let mut arr = vec![
    //     1, 6586, 145, 463, 14, 574, 169, 461, 69875, 4563, 4, 58, 46358, 43, 5,
    // ];
    // let mut arr = vec![1, 9, 2, 8, 3, 7, 4, 6, 5];
    // let mut arr = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];

    let mut arr: Vec<u32> = (0..50).rev().collect();

    let arrr = arr.clone();
    println!("Original:- \n{arrr:?}\n");
    println!("Rust:-");
    arr.sort_unstable();
    println!("{arr:?}\n");

    run_selection(&arrr);

    run_double_selection(&arrr);

    run_insertion(&arrr);

    run_binary_insertion(&arrr);

    run_bubble(&arrr);

    run_shaker(&arrr);

    run_quick(&arrr);

    run_merge(&arrr);

    run_heap(&arrr);
}

fn run_selection<T>(arr: &[T]) -> Vec<T>
where
    T: PartialOrd + Clone + std::fmt::Debug,
{
    let mut arr = (*arr).to_vec();
    println!("Selection:-");
    selection(&mut arr);
    println!("{arr:?}\n");
    arr
}

fn run_double_selection<T>(arr: &[T]) -> Vec<T>
where
    T: PartialOrd + Clone + std::fmt::Debug,
{
    let mut arr = (*arr).to_vec();
    println!("Double Selection:-");
    double_selection(&mut arr);
    println!("{arr:?}\n");
    arr
}

fn run_insertion<T>(arr: &[T]) -> Vec<T>
where
    T: PartialOrd + Clone + std::fmt::Debug,
{
    let mut arr = (*arr).to_vec();
    println!("Insertion:-");
    insertion(&mut arr);
    println!("{arr:?}\n");
    arr
}

fn run_binary_insertion<T>(arr: &[T]) -> Vec<T>
where
    T: PartialOrd + Clone + std::fmt::Debug,
{
    let mut arr = (*arr).to_vec();
    println!("Binary Insertion:-");
    binary_insertion(&mut arr);
    println!("{arr:?}\n");
    arr
}

fn run_bubble<T>(arr: &[T]) -> Vec<T>
where
    T: PartialOrd + Clone + std::fmt::Debug,
{
    let mut arr = (*arr).to_vec();
    println!("Bubble:-");
    bubble(&mut arr);
    println!("{arr:?}\n");
    arr
}

fn run_shaker<T>(arr: &[T]) -> Vec<T>
where
    T: PartialOrd + Clone + std::fmt::Debug,
{
    let mut arr = (*arr).to_vec();
    println!("Shaker:-");
    shaker(&mut arr);
    println!("{arr:?}\n");
    arr
}

fn run_quick<T>(arr: &[T]) -> Vec<T>
where
    T: PartialOrd + Clone + std::fmt::Debug,
{
    let mut arr = (*arr).to_vec();
    println!("Quick:-");
    quick(&mut arr);
    println!("{arr:?}\n");
    arr
}

fn run_merge<T>(arr: &[T]) -> Vec<T>
where
    T: PartialOrd + Clone + std::fmt::Debug,
{
    let mut arr = (*arr).to_vec();
    println!("Merge:-");
    merge(&mut arr);
    println!("{arr:?}\n");
    arr
}

fn run_heap<T>(arr: &[T]) -> Vec<T>
where
    T: PartialOrd + Clone + std::fmt::Debug,
{
    let mut arr = (*arr).to_vec();
    println!("Heap:-");
    heap(&mut arr);
    println!("{arr:?}\n");
    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    fn arrays() -> Vec<Vec<i32>> {
        let sorted = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let reverse = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        let interlaced = vec![0, 9, 1, 8, 2, 7, 3, 6, 4, 5];
        let mountain = vec![0, 2, 4, 6, 8, 9, 7, 5, 3, 1];
        Vec::from([sorted, reverse, interlaced, mountain])
    }

    #[test]
    fn test_selection() {
        let arrays = arrays();
        for i in arrays {
            let mut arr = i.clone();
            println!("{arr:?}");
            arr.sort_unstable();
            assert_eq!(run_selection(&i), arr);
        }
    }

    #[test]
    fn test_double_selection() {
        let arrays = arrays();
        for i in arrays {
            let mut arr = i.clone();
            println!("{arr:?}");
            arr.sort_unstable();
            assert_eq!(run_double_selection(&i), arr);
        }
    }

    #[test]
    fn test_insertion() {
        let arrays = arrays();
        for i in arrays {
            let mut arr = i.clone();
            println!("{arr:?}");
            arr.sort_unstable();
            assert_eq!(run_insertion(&i), arr);
        }
    }

    #[test]
    fn test_binary_insertion() {
        let arrays = arrays();
        for i in arrays {
            let mut arr = i.clone();
            println!("{arr:?}");
            arr.sort_unstable();
            assert_eq!(run_binary_insertion(&i), arr);
        }
    }

    #[test]
    fn test_bubble() {
        let arrays = arrays();
        for i in arrays {
            let mut arr = i.clone();
            println!("{arr:?}");
            arr.sort_unstable();
            assert_eq!(run_bubble(&i), arr);
        }
    }

    #[test]
    fn test_shaker() {
        let arrays = arrays();
        for i in arrays {
            let mut arr = i.clone();
            println!("{arr:?}");
            arr.sort_unstable();
            assert_eq!(run_shaker(&i), arr);
        }
    }

    #[test]
    fn test_quick() {
        let arrays = arrays();
        for i in arrays {
            let mut arr = i.clone();
            println!("{arr:?}");
            arr.sort_unstable();
            assert_eq!(run_quick(&i), arr);
        }
    }

    #[test]
    fn test_merge() {
        let arrays = arrays();
        for i in arrays {
            let mut arr = i.clone();
            println!("{arr:?}");
            arr.sort_unstable();
            assert_eq!(run_merge(&i), arr);
        }
    }

    #[test]
    fn test_heap() {
        let arrays = arrays();
        for i in arrays {
            let mut arr = i.clone();
            println!("{arr:?}");
            arr.sort_unstable();
            assert_eq!(run_heap(&i), arr);
        }
    }
}
