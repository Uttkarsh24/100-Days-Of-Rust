// this function searches an array to find a subarray with the given sum
// it returns the index where the subarray starts along with the length of the subarray
// if the array does not include any subarray with the sum, it returns a tuple with length or array
fn find_subarray(nums: &[i32], sum: i32) -> (usize, usize) {
    for len in (1..nums.len() + 1).rev() {
        for start in 0..nums.len() - len + 1 {
            if array_sum(&nums[start..start+len]) == sum {
                return (start, len);
            }
        }
    }
    (nums.len(), nums.len())
}
fn array_sum(nums:&[i32]) -> i32 {
    let mut res = 0;
    for num in nums {
        res += num;
    }
    res
}
fn main()
{
    let nums = [1, 1, 2, 3, 5, 8, 13];
    let res = find_subarray(&nums[..], 16);
    if res.0 == nums.len() {
        println!("No subarray found");
    } else {
        println!("Subarray found: {:?}", &nums[res.0..res.0 + res.1]);
    }
}