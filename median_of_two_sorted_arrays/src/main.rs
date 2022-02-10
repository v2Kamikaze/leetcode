pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let length = nums1.len() + nums2.len();
    let mut merged = vec![0; length];
    let mut i: usize = 0;

    for v in nums1.iter() {
        merged[i] = *v;
        i = i + 1;
    }

    for v in nums2.iter() {
        merged[i] = *v;
        i = i + 1;
    }

    merged.sort();

    if merged.len() % 2 == 0 {
        let start_idx = length / 2;
        let end_idx = start_idx - 1;
        return (merged[start_idx] + merged[end_idx]) as f64 / 2.0
    }

    merged[length/2] as f64
}

pub fn find_median_sorted_arrays_iter(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let length = nums1.len() + nums2.len();
    let mut merged = vec![0; length];
    let mut i: usize = 0;

    nums1.iter().chain(nums2.iter()).into_iter().for_each(|x| {
        merged[i] = *x;
        i = i + 1;
    });

    merged.sort();

    if merged.len() % 2 == 0 {
        let start_idx = length / 2;
        let end_idx = start_idx - 1;
        return (merged[start_idx] + merged[end_idx]) as f64 / 2.0
    }

    merged[length/2] as f64
}

pub fn find_median_sorted_arrays_iter_2(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut merged: Vec<i32> = nums1.iter().chain(nums2.iter()).map(|x| *x).collect::<Vec<_>>();
    merged.sort();

    let length = merged.len();

    if merged.len() % 2 == 0 {
        return (merged[length / 2] + merged[(length / 2) - 1]) as f64 / 2.0;
    }

    merged[length/2] as f64
}

fn main() {
    let nums1 = vec![1,3]; 
    let nums2 = vec![2];

    println!("{:?}", find_median_sorted_arrays(nums1.clone(), nums2.clone()));
    println!("{:?}", find_median_sorted_arrays_iter(nums1.clone(), nums2.clone()));
    println!("{:?}", find_median_sorted_arrays_iter_2(nums1.clone(), nums2.clone()))
}