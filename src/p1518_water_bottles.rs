// 
// [1518] Water Bottles
// 
// 
// There are numBottles water bottles that are initially full of water. You can exchange numExchange empty water bottles from the market with one full water bottle.
// 
// The operation of drinking a full water bottle turns it into an empty bottle.
// 
// Given the two integers numBottles and numExchange, return the maximum number of water bottles you can drink.
// 
//  
// Example 1:
// 
// Input: numBottles = 9, numExchange = 3
// Output: 13
// Explanation: You can exchange 3 empty bottles to get 1 full water bottle.
// Number of water bottles you can drink: 9 + 3 + 1 = 13.
// 
// 
// Example 2:
// 
// Input: numBottles = 15, numExchange = 4
// Output: 19
// Explanation: You can exchange 4 empty bottles to get 1 full water bottle. 
// Number of water bottles you can drink: 15 + 3 + 1 = 19.
// 
// 
//  
// Constraints:
// 
// 
// 	1 <= numBottles <= 100
// 	2 <= numExchange <= 100
// 
// 
struct Solution;
impl Solution {
    pub fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
        let mut drank = 0;
        let mut carry = 0;
        loop {
            drank += num_bottles;
            let empty = num_bottles + carry;
            carry = empty % num_exchange;
            num_bottles = empty / num_exchange;
            if num_bottles == 0 {
                return drank;
            }
        }
    }
}
#[test]
fn t_drink() {
    assert_eq!(Solution::num_water_bottles(9, 3), 13);
    assert_eq!(Solution::num_water_bottles(1, 3), 1);
    assert_eq!(Solution::num_water_bottles(15, 4), 19);
    assert_eq!(Solution::num_water_bottles(15, 8), 17);
}
