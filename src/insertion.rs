pub fn insertion<T>(arr: &mut [T])
where
    T: PartialOrd + Clone + std::fmt::Debug,
{
    let len = arr.len();
    let mut numop = 0;
    let mut numcmp = 0;
    // let mut numicmp = 0;
    for i in 0..len {
        let mut place_idx = i;
        let mut idx_changed = false;
        for j in (0..i).rev() {
            numcmp += 1;
            if arr[i] > arr[j] {
                place_idx = j + 1;
                idx_changed = true;
                break;
            }
        }
        if !idx_changed {
            place_idx = 0;
        }
        for j in (place_idx..i).rev() {
            arr.swap(j, j + 1);
            numop += 1;
        }
    }
    println!("Number of swaps used:- {numop}");
    println!("Number of comparisions used:- {numcmp}");
    // println!("Number of index comparisions used:- {numicmp}");
}
