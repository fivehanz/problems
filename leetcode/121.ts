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

/**
 * Calculates the maximum profit that can be achieved by buying and selling stocks.
 *
 * @param {number[]} prices - An array of stock prices.
 * @return {number} - The maximum profit that can be achieved.
 */
function maxProfit(prices: number[]): number {
  let buy = 0; // index of the potential buying point
  let sell = 1; // index of the potential selling point
  let maxProfit = 0; // maximum profit achieved so far

  while (sell < prices.length) {
    if (prices[buy] < prices[sell]) {
      let profit = prices[sell] - prices[buy]; // calculate the profit
      maxProfit = Math.max(profit, maxProfit); // update the maxProfit if necessary
    } else {
      buy = sell; // update the buying point
    }
    sell++; // move to the next potential selling point
  }

  return maxProfit; // return the maximum profit achieved
}
