/*
* Leetcode 167M Sorted TwoSum
* Key: Two Pointer
* T: O(n)
 */

func twoSum(numbers []int, target int) []int {
	var start, sum int
	var end int = len(numbers) - 1

	indices := make([]int, 2)

	for start < end {
		sum = numbers[start] + numbers[end]
		if sum < target {
			start++
		} else if sum > target {
			end--
		} else if sum == target {
			indices[0] = start + 1
			indices[1] = end + 1
			break
		}
	}

	return indices
}
