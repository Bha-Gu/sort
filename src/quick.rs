fn qs<T>(
    arr: &mut [T],
    lo: usize,
    hi: usize,
    numop: &mut usize,
    numcmp: &mut usize,
    num_idx_cmp: &mut usize,
) where
    T: PartialOrd + Clone + core::fmt::Debug,
{
    *num_idx_cmp += 2;

    let pivot_idx = partition(arr, lo, hi, numop, numcmp, num_idx_cmp);
    if lo < pivot_idx {
        qs(arr, lo, pivot_idx - 1, numop, numcmp, num_idx_cmp);
    }
    if hi > pivot_idx {
        qs(arr, pivot_idx + 1, hi, numop, numcmp, num_idx_cmp);
    }
}

fn partition<T>(
    arr: &mut [T],
    lo: usize,
    hi: usize,
    numop: &mut usize,
    numcmp: &mut usize,
    num_idx_cmp: &mut usize,
) -> usize
where
    T: PartialOrd + Clone + std::fmt::Debug,
{
    // let mut numop: usize = 0;
    let pivot: T = arr[hi].to_owned();
    let mut idx = lo;
    for i in lo..hi {
        *numcmp += 1;
        if arr[i] <= pivot {
            *num_idx_cmp += 1;
            if i != idx {
                *numop += 1;
                arr.swap(i, idx);
                // println!("{arr:?}");
            }
            idx += 1;
        }
    }
    *num_idx_cmp += 1;
    if hi != idx {
        *numop += 1;
        arr.swap(hi, idx);
        // println!("{arr:?}");
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
    let mut num_idx_cmp = 0;
    qs(arr, lo, hi, &mut numop, &mut numcmp, &mut num_idx_cmp);
    println!("Number of swaps used:- {numop}");
    println!("Number of comparisions used:- {numcmp}");
    println!("Number of index comparisions used:- {num_idx_cmp}");
}
