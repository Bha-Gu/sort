pub fn merge<T>(arr: &mut [T])
where
    T: PartialOrd + Clone + std::fmt::Debug,
{
    let len = arr.len();
    let lo = 0;
    let hi = len - 1;
    let mut monitor = [0; 3];
    ms(arr, lo, hi, &mut monitor);
    println!("Number of writes to main array used:- {}", monitor[0]);
    println!("Number of comparisions used:- {}", monitor[1]);
    println!("Number of index comparisions used:- {}", monitor[2]);
}

fn ms<T>(arr: &mut [T], lo: usize, hi: usize, monitor: &mut [usize; 3])
where
    T: PartialOrd + Clone + std::fmt::Debug,
{
    let mid = (lo + hi) / 2;
    monitor[2] += 2;
    if lo < mid {
        ms(arr, lo, mid, monitor);
    }
    if hi > mid {
        ms(arr, mid + 1, hi, monitor);
    }
    merge_arr(arr, lo, mid, hi, monitor);
}

fn merge_arr<T>(arr: &mut [T], lo: usize, mid: usize, hi: usize, monitor: &mut [usize; 3])
where
    T: PartialOrd + Clone + std::fmt::Debug,
{
    monitor[2] += 1;
    if hi == lo {
        return;
    }
    monitor[1] += 1;
    if arr[mid] <= arr[mid + 1] {
        return;
    }
    let mut aux: Vec<T> = Vec::with_capacity(mid - lo + 1);
    (lo..=mid).for_each(|i| {
        aux.push(arr[i].clone());
    });
    let aux = aux;
    let mut i = 0;
    let mut j = mid + 1;
    let mut k = lo;
    while i <= mid - lo && j <= hi {
        // monitor[2] += 2;
        monitor[1] += 1;
        monitor[0] += 1;
        arr[k] = if aux[i] <= arr[j] {
            let a = aux[i].clone();
            i += 1;
            a
        } else {
            let a = arr[j].clone();
            j += 1;
            a
        };
        k += 1;
    }
    monitor[2] += 2;
    if i > mid - lo {
        while j <= hi {
            monitor[0] += 1;
            arr[k] = arr[j].clone();
            j += 1;
            k += 1;
        }
    }
    if j > hi {
        while i <= mid - lo {
            monitor[0] += 1;
            arr[k] = aux[i].clone();
            i += 1;
            k += 1;
        }
    }
}
