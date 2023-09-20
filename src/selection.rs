pub fn selection<T>(arr: &mut [T])
where
    T: PartialOrd + Clone + std::fmt::Debug,
{
    let len = arr.len();
    let mut numop = 0;
    let mut numcmp = 0;
    for i in 0..len {
        let mut smallest_idx = i;
        for j in i..len {
            if arr[j] < arr[smallest_idx] {
                smallest_idx = j;
                numcmp += 1;
            }
        }
        arr.swap(i, smallest_idx);
        numop += 1;
    }
    println!("Number of swaps used:- {numop}");
    println!("Number of comparisions used:- {numcmp}");
}
