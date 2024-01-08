use std::{collections::HashMap, u128};

//
// [1155] Number of Dice Rolls With Target Sum
//
//
// You have n dice, and each die has k faces numbered from 1 to k.
//
// Given three integers n, k, and target, return the number of possible ways (out of the kn total ways) to roll the dice, so the sum of the face-up numbers equals target. Since the answer may be too large, return it modulo 10⁹ + 7.
//
//
// Example 1:
//
// Input: n = 1, k = 6, target = 3
// Output: 1
// Explanation: You throw one die with 6 faces.
// There is only one way to get a sum of 3.
//
//
// Example 2:
//
// Input: n = 2, k = 6, target = 7
// Output: 6
// Explanation: You throw two dice, each with 6 faces.
// There are 6 ways to get a sum of 7: 1+6, 2+5, 3+4, 4+3, 5+2, 6+1.
//
//
// Example 3:
//
// Input: n = 30, k = 30, target = 500
// Output: 222616187
// Explanation: The answer must be returned modulo 10⁹ + 7.
//
//
//
// Constraints:
//
//
// 	1 <= n, k <= 30
// 	1 <= target <= 1000
//
//
struct Solution;
const MOD: i32 = 10_i32.pow(9_u32) + 7;
impl Solution {
    // pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
    //     // n is the number of die you have
    //     // k is the faces on the die
    //     if target < n {
    //         return 0;
    //     } // if you have 10 dice and you have
    //       // a target of 5 then there is no way
    //       // to get a score of less then 10
    //       //
    //     if target >= n * k {
    //         return 0;
    //     } // target too big
    //
    //     let mut n_ways: [i32; 1001] = [0; 1001];
    //     // first roll of the die
    //     // 1 way to roll a number from 1 to
    //     for j in 1..=k {
    //         n_ways[j as usize] = 1;
    //     }
    //
    //     // do the n die rolls
    //     // but start at one since we've already simulated the first one
    //     for i in 2..=n {
    //         let mut tmp: [i32; 1001] = [0; 1001];
    //         for j in 1..=k {
    //             // operate on a range of value
    //             let a = (target - (n - i) * k - j).max(0);
    //             let b = (target - (n - i) - j).max(0);
    //             for idx in a..=b {
    //                 tmp[(idx+j) as usize] = (tmp[(idx+j) as usize] + n_ways[idx as usize]) % MOD;
    //             }
    //         }
    //         n_ways = tmp;
    //     }
    //     n_ways[target as usize]
    // }
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 
    {
        // [1] trivial cases
        if target > n*k { return 0; }
        if target < n   { return 0; }
        
        // [2] first roll of the die (i.e., 1 way to roll a number from 1 to k)
        let mut n_of_ways: [i32;1001] = [0;1001];
        for j in 1..=k { n_of_ways[j as usize] = 1; }
        
        // [3] iterate over subsequent rolls 
        for i in 2..=n
        {
            let mut n_of_ways_: [i32;1001] = [0;1001];
            for j in 1..=k
            {
                // [4] for each value of face-up number we care only 
                //     about a certain range of sums that allows us  
                //     to finally reach a target value
                let beg = (target - (n-i)*k - j).max(0);
                let end = (target - (n-i) - j).max(0);
                
                for s in beg..=end
                {
                    n_of_ways_[(s+j) as usize] = 
                        (n_of_ways_[(s+j) as usize] + n_of_ways[s as usize]) % 1000000007;
                }
            }
            
            // [5] update the number of ways to reach calculated
            //     sums after i-th roll of the dice
            n_of_ways = n_of_ways_;
        }

        // [6] among calculated values, there is a corresponding
        //     number of ways for 'target', just return it
        n_of_ways[target as usize]
    }
}

#[test]
fn testfn1() {
    let res = Solution::num_rolls_to_target(1, 6, 3);
    dbg!(res);
    assert!(res == 1);
}

#[test]
fn testfn() {
    let res = Solution::num_rolls_to_target(2, 6, 7);
    dbg!(res);
    assert!(res == 6);
}

#[test]
fn big_input() {
    let res = Solution::num_rolls_to_target(30, 30, 500);
    dbg!(res);
    assert!(res == 222616187);
}
