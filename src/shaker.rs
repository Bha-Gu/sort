pub fn shaker<T>(arr: &mut [T])
where
    T: PartialOrd,
{
    let mut numop: usize = 0;
    let mut numcmp = 0;
    for i in 0..arr.len() - 1 {
        let mut swapped = false;
        for j in i..(arr.len() - 1 - i) {
            numcmp += 1;
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                numop += 1;
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
        for j in (i..(arr.len() - i - 2)).rev() {
            numcmp += 1;
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                numop += 1;
            }
        }
    }
    println!("Number of swaps used:- {numop}");
    println!("Number of comparisions used:- {numcmp}");
}
