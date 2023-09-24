const SHRINK_FACTOR: f64 = 2.3;

pub fn shell<T>(arr: &mut [T])
where
    T: PartialOrd + Clone + std::fmt::Debug,
{
    let len = arr.len();
    let mut gap = len as f64;
    let mut sorted = false;

    let mut numop = 0;
    let mut numcmp = 0;
    while !sorted {
        gap /= SHRINK_FACTOR;
        if gap <= 1.0 {
            gap = 1.0;
            sorted = true;
        }
        for _ in 0..gap as usize {
            for i in (0..len).step_by(gap as usize) {
                let mut place_idx = i;
                let mut idx_changed = false;
                for j in (0..i).step_by(gap as usize).rev() {
                    numcmp += 1;
                    if arr[i] > arr[j] {
                        place_idx = j + gap as usize;
                        idx_changed = true;
                        break;
                    }
                }
                if !idx_changed {
                    place_idx = 0;
                }
                for j in (place_idx..i).step_by(gap as usize).rev() {
                    arr.swap(j, j + gap as usize);
                    numop += 1;
                }
            }
        }
    }

    println!("Number of swaps used:- {numop}");
    println!("Number of comparisions used:- {numcmp}");
    // let mut num_idx_cmp = 0;
}
