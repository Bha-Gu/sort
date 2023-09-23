pub fn binary_insertion<T>(arr: &mut [T])
where
    T: PartialOrd + Clone + std::fmt::Debug,
{
    let len = arr.len();
    let mut numop = 0;
    let mut numcmp = 0;
    // let mut num_idx_cmp = 0;
    for i in 0..len {
        let mut lo = 0;
        let mut hi = i;
        while hi - lo > 0 {
            let mid = (lo + hi) / 2;
            numcmp += 1;
            if arr[mid] > arr[i] {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        let place_idx = hi;
        for j in (place_idx..i).rev() {
            arr.swap(j, j + 1);
            numop += 1;
        }
    }
    println!("Number of swaps used:- {numop}");
    println!("Number of comparisions used:- {numcmp}");
    // println!("Number of index comparisions used:- {num_idx_cmp}");
}
