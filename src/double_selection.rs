pub fn double_selection<T>(arr: &mut [T])
where
    T: PartialOrd + Clone + std::fmt::Debug,
{
    let len = arr.len();
    let mut numop = 0;
    let mut numcmp = 0;
    for i in 0..len {
        if i >= len - i - 1 {
            break;
        }
        let mut smallest_idx = i;
        for j in (i + 1)..(len - i) {
            if arr[j] < arr[smallest_idx] {
                smallest_idx = j;
                numcmp += 1;
            }
        }
        arr.swap(i, smallest_idx);
        numop += 1;
        let mut larget_idx = len - i - 1;
        for j in (i + 1)..(len - i - 1) {
            if arr[j] > arr[larget_idx] {
                larget_idx = j;
                numcmp += 1;
            }
        }
        arr.swap(len - i - 1, larget_idx);
        numop += 1;
        // println!("{arr:?} {i} ");
    }
    println!("Number of swaps used:- {numop}");
    println!("Number of comparisions used:- {numcmp}");
}
