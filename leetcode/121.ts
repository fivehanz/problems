/*
greedy?

let left = 0
let right = 1
maxProfit = 0

loop until right -> prices:
    if (prices[left] < prices[right]):
        profit = prices[right] - prices[left]
        maxProfit = Math.max(profit, maxProfit)
    else:
        left = right
    right++
    
return maxProfit

TC: O(n)
SC: O(1)
    
*/

function maxProfit(prices: number[]): number {
    let buy = 0
    let sell = 1
    let maxProfit = 0
    
    while (sell < prices.length) {
        if (prices[buy] < prices[sell]) {
            let profit = prices[sell] - prices[buy]
            maxProfit = Math.max(profit, maxProfit)
        } else {
            buy = sell
        }
        sell++
    }
    
    return maxProfit
};
