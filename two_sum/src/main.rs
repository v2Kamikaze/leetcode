pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![0; 2];
    let n: usize = nums.len();

    for i in 0..n {
        for j in (i + 1)..n {
            if nums[i] + nums[j] == target {
                result[0] = i as i32;
                result[1] = j as i32;
                return result;
            }
        }
    }

    result
}

fn main() {
    let input = vec![2, 7, 11, 15, 2];
    let target = 9;
    println!("{:?}", two_sum(input, target));
}
