/* 
Brute force Algorithm:
loop i = 0 to n:
    loop j = i+1 to n:
        if nums[i] + nums[j] === target
            return [i, j]
Time: O(n^2)
Space: O(1)

Optimal: nums[a] + nums[b] = target
mapOfIndices = hashmap

loop i=0 to n:
    // nums[b] = t - nums[a]
    diff = target - nums[a] 
    
    check if diff is in map
        if true: return [a, mapOfIndices[diff]]
    
    push nums[a] = i to mapOfIndices

Time: O(n)
Space: O(n)
*/

function twoSum(nums: number[], target: number): number[] {
    let mapOfIndices : Map<number, number> = new Map();
    
    for (let i = 0; i < nums.length; i++) {
        const diff: number = target - nums[i]
        
        if (mapOfIndices.has(diff)) {
            return [mapOfIndices.get(diff)!, i]
        }
        
        mapOfIndices.set(nums[i], i)
    }
    return [];
};
