/*
* LC 977E Squares of a Sorted Array
* Key: Two Pointers, compare left-element square with right-element square
* TC: O(n) SC: O(n)
 */

// sortedSquares squares each element of the given slice of integers and returns a new slice with the squared values in non-decreasing order.
//
// nums: a slice of integers.
// []int: a new slice of integers with squared values in non-decreasing order.
func sortedSquares(nums []int) []int {
    // Get the length of the input slice
    var N int = len(nums)

    // Set the start, end, and index pointers
    var start, end, index int = 0, N - 1, N - 1

    // Create a new slice to store the squared values
    squared := make([]int, N)

    // Iterate over the input slice from both ends towards the middle
    for start <= end {
        // Calculate the squared values for the elements at start and end pointers
        l := nums[start] * nums[start]
        r := nums[end] * nums[end]

        // Compare the squared values
        if l > r {
            // Store the squared value at the index pointer and move the start pointer to the next element
            squared[index] = l
            start++
        } else {
            // Store the squared value at the index pointer and move the end pointer to the previous element
            squared[index] = r
            end--
        }

        // Move the index pointer to the previous position
        index--
    }

    // Return the new slice with squared values in non-decreasing order
    return squared
}
