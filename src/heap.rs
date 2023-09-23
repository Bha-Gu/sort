pub fn heap<T>(arr: &mut [T])
where
    T: PartialOrd + Clone + std::fmt::Debug,
{
    let n = arr.len();

    let mut numop = [0; 3];

    // Build a max heap (convert the array into a max heap)
    for i in (0..n / 2).rev() {
        heapify(arr, n, i, &mut numop);
    }

    // One by one extract elements from the max heap
    for i in (1..n).rev() {
        // Move the current root (maximum element) to the end of the array
        arr.swap(0, i);
        numop[0] += 1;
        // Call heapify on the reduced heap
        heapify(arr, i, 0, &mut numop);
    }

    println!("Number of swaps used:- {}", numop[0]);
    println!("Number of comparisions used:- {}", numop[1]);
    println!("Number of index comparisions used:- {}", numop[2]);
}

fn heapify<T: PartialOrd + Clone + std::fmt::Debug>(
    arr: &mut [T],
    n: usize,
    i: usize,
    numop: &mut [usize; 3],
) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    // If left child is larger than root
    numop[1] += 1;
    numop[2] += 1;
    if left < n && arr[left] > arr[largest] {
        largest = left;
    }

    // If right child is larger than largest so far
    numop[1] += 1;
    numop[2] += 1;
    if right < n && arr[right] > arr[largest] {
        largest = right;
    }

    // If largest is not the root
    numop[2] += 1;
    if largest != i {
        arr.swap(i, largest);
        numop[1] += 1;
        // Recursively heapify the affected sub-tree
        heapify(arr, n, largest, numop);
    }
}
