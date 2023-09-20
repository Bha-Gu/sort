fn qs<T>(arr: &mut [T], lo: usize, hi: usize, numop: &mut usize, numcmp: &mut usize)
where
    T: PartialOrd + Clone + core::fmt::Debug,
{
    if lo >= hi {
        //Base Case
    } else {
        let pivot_idx = partition(arr, lo, hi, numop, numcmp);
        qs(arr, lo, pivot_idx - 1, numop, numcmp);
        qs(arr, pivot_idx + 1, hi, numop, numcmp);
    }
}

fn partition<T>(arr: &mut [T], lo: usize, hi: usize, numop: &mut usize, numcmp: &mut usize) -> usize
where
    T: PartialOrd + Clone,
{
    // let mut numop: usize = 0;
    let pivot: T = arr[hi].to_owned();
    let mut idx = lo;
    for i in lo..hi {
        if arr[i] <= pivot {
            *numcmp += 1;
            if i != idx {
                *numop += 1;
                arr.swap(i, idx);
            }
            idx += 1;
        }
    }
    if hi != idx {
        *numop += 1;
        arr.swap(hi, idx);
    }

    // print!("+{numop}");
    idx
}

pub fn quick<T>(arr: &mut [T])
where
    T: PartialOrd + Clone + core::fmt::Debug,
{
    let lo = 0;
    let len = arr.len();
    let hi = if len > 1 { len - 1 } else { 0 };
    let mut numop: usize = 0;
    let mut numcmp = 0;
    qs(arr, lo, hi, &mut numop, &mut numcmp);
    println!("Number of swaps used:- {numop}");
    println!("Number of comparisions used:- {numcmp}");
}
