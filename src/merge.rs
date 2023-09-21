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
    let mut aux: Vec<T> = Vec::with_capacity(arr.len());
    for i in &*arr {
        aux.push(i.clone());
    }
    let aux = aux;
    let mut i = lo;
    let mut j = mid + 1;
    let mut k = lo;
    while i <= mid && j <= hi {
        // monitor[2] += 2;
        monitor[1] += 1;
        arr[k] = if aux[i] <= aux[j] {
            monitor[0] += 1;
            let a = aux[i].clone();
            i += 1;
            a
        } else {
            monitor[0] += 1;
            let a = aux[j].clone();
            j += 1;
            a
        };
        k += 1;
    }
    monitor[2] += 2;
    if i > mid {
        while j <= hi {
            monitor[0] += 1;
            arr[k] = aux[j].clone();
            j += 1;
            k += 1;
        }
    }
    if j > hi {
        while i <= mid {
            monitor[0] += 1;
            arr[k] = aux[i].clone();
            i += 1;
            k += 1;
        }
    }
}
