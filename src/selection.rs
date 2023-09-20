pub fn selection<T>(arr: &mut [T])
where
    T: PartialOrd + Clone + std::fmt::Debug,
{
    let len = arr.len();
    let mut numop = 0;
    let mut numcmp = 0;
    let mut numicmp = 0;
    for i in 0..len {
        let mut smallest_idx = i;
        for j in i..len {
            if arr[j] < arr[smallest_idx] {
                smallest_idx = j;
            }
            numcmp += 1;
        }
        numicmp += 1;
        if i == smallest_idx {
            continue;
        }
        arr.swap(i, smallest_idx);
        numop += 1;
    }
    println!("Number of swaps used:- {numop}");
    println!("Number of comparisions used:- {numcmp}");
    println!("Number of index comparisions used:- {numicmp}");
}
