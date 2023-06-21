fn qs<T>(arr: &mut Vec<T>, lo: usize, hi: usize)
where
    T: PartialOrd + Clone,
{
    if lo >= hi {
    } else {
        let pivot_idx = partition(arr, lo, hi);

        qs(arr, lo, pivot_idx - 1);
        qs(arr, pivot_idx + 1, hi);
    }
}

fn partition<T>(arr: &mut Vec<T>, lo: usize, hi: usize) -> usize
where
    T: PartialOrd + Clone,
{
    let pivot: T = arr[hi].to_owned();
    let mut idx = lo;
    for i in lo..hi {
        if arr[i] <= pivot {
            arr.swap(i, idx);
            idx += 1;
        }
    }

    idx += 1;
    arr.swap(hi, idx);
    idx
}

pub fn quick<T>(arr: &mut Vec<T>)
where
    T: PartialOrd + Clone,
{
    let lo = 0;
    let hi = arr.len();
    qs(arr, lo, hi);
}
