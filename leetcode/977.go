/*
* LC 977E Squares of a Sorted Array
* Key: Two Pointers, compare left-element square with right-element square
* TC: O(n) SC: O(n)
 */

func sortedSquares(nums []int) []int {
	var N int = len(nums)
	var start, end, index int = 0, N - 1, N - 1

	squared := make([]int, N)

	for start <= end {
		l := nums[start] * nums[start]
		r := nums[end] * nums[end]

		if l > r {
			squared[index] = l
			start++
		} else {
			squared[index] = r
			end--
		}

		index--
	}

	return squared
}
