/*
* LC 1E Two Sum
* Key: use map/hashmap
 */

func twoSum(nums []int, target int) []int {

	ans := make([]int, 2, 2)
	mapOfIndices := make(map[int]int)

	for index, value := range nums {
		sub := target - value

		if _, ok := mapOfIndices[sub]; ok {
			ans[0], ans[1] = mapOfIndices[sub], index
			break
		}

		mapOfIndices[value] = index
	}

	return ans
}
