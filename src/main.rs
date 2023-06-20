fn bubble<T>(arr: &mut Vec<T>) 
where T: PartialOrd,
{
    for i in 0..arr.len() - 1 {
        let mut swapped = false;
        for j in 0..(arr.len() - 1 - i) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j+1);
                swapped = true;
            }
        }
        if !swapped{
            break;
        }
        
    }
}

fn main() {
    let mut arr = vec![1,6586,145,463,14, 574,169,461,69875,4563,4,58,46358,43,5];
    let mut arr2 = arr.clone();
    let arrr = arr.clone();
    arr.sort();
    bubble( &mut arr2 );
    println!("{:?}\n{:?}\n{:?}", arrr, arr, arr2);
}
