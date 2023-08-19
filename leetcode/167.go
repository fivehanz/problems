/*
* Leetcode 167M Sorted TwoSum
* Key: Two Pointer
* T: O(n)
 */

// twoSum finds two numbers in a given slice of integers that add up to a target value.
//
// numbers: the slice of integers to search through.
// target: the target value to find the pair for.
// []int: a slice containing the indices of the two numbers that add up to the target value.
func twoSum(numbers []int, target int) []int {
	// Initialize variables
	var start, sum int
	var end int = len(numbers) - 1

	// Create a slice to store the indices
	indices := make([]int, 2)

	// Iterate through the slice
	for start < end {
		// Calculate the sum of the numbers at start and end indices
		sum = numbers[start] + numbers[end]

		// If the sum is less than the target, increment start
		if sum < target {
			start++
		} 
		// If the sum is greater than the target, decrement end
		else if sum > target {
			end--
		} 
		// If the sum is equal to the target, store the indices and exit the loop
		else if sum == target {
			indices[0] = start + 1
			indices[1] = end + 1
			break
		}
	}

	// Return the indices
	return indices
}
