/*
* Leetcode 167M Sorted TwoSum
* Key: Two Pointer
* T: O(n)
 */

func twoSum(numbers []int, target int) []int {
	var start int
	var end int = len(numbers) - 1

	indices := make([]int, 2)

	for start < end {
		if numbers[start]+numbers[end] < target {
			start++
		} else if numbers[start]+numbers[end] > target {
			end--
		} else if numbers[start]+numbers[end] == target {
			indices[0] = start + 1
			indices[1] = end + 1
			break
		}
	}

	return indices
}
