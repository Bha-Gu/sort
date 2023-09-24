const SHRINK_FACTOR: f64 = 1.3;

pub fn comb<T>(arr: &mut [T])
where
    T: PartialOrd,
{
    let mut numop: usize = 0;
    let mut numcmp = 0;

    let len = arr.len();
    let mut gap = len as f64;
    let mut sorted = false;

    while !sorted {
        gap /= SHRINK_FACTOR;
        if gap <= 1.0 {
            gap = 1.0;
            sorted = true;
        }
        let gap = gap as usize;

        for i in 0..len - gap {
            numcmp += 1;
            if arr[i] > arr[i + gap] {
                numop += 1;
                arr.swap(i, i + gap);
                sorted = false;
            }
        }
    }

    println!("Number of swaps used:- {numop}");
    println!("Number of comparisions used:- {numcmp}");
}
