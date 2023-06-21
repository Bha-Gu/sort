pub fn bubble<T>(arr: &mut [T])
where
    T: PartialOrd,
{
    let mut numop: usize = 0;
    for i in 0..arr.len() - 1 {
        let mut swapped = false;
        for j in 0..(arr.len() - 1 - i) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                numop += 1;
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
    println!("Number of swaps used:- {}", numop);
}
