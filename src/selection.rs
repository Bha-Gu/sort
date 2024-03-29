pub fn selection<T>(arr: &mut [T])
where
    T: PartialOrd + Clone + std::fmt::Debug,
{
    let len = arr.len();
    let mut numop = 0;
    let mut numcmp = 0;
    let mut num_idx_cmp = 0;
    for i in 0..len {
        let mut smallest_idx = i;
        for j in i + 1..len {
            if arr[j] < arr[smallest_idx] {
                smallest_idx = j;
            }
            numcmp += 1;
        }
        num_idx_cmp += 1;
        if i == smallest_idx {
            continue;
        }
        arr.swap(i, smallest_idx);
        numop += 1;
    }
    println!("Number of swaps used:- {numop}");
    println!("Number of comparisions used:- {numcmp}");
    println!("Number of index comparisions used:- {num_idx_cmp}");
}
