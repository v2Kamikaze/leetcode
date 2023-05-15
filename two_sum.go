package main

import "fmt"

func twoSum(nums []int, target int) []int {
	result := make([]int, 2)
	length := len(nums)

	for i := 0; i < length; i++ {
		for j := 0; j < i+1; j++ {
			if j != i && nums[i]+nums[j] == target {
				result[0] = j
				result[1] = i
				return result
			}
		}
	}

	return result
}

func main() {
	fmt.Println(twoSum([]int{3, 2, 4}, 6))
}
