use std::{u128, collections::HashMap};

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
    const  MOD : i32 = 10_i32.pow(9_u32) + 7;
    // int MOD = 1000000000 + 7;
    // Map<String, Integer> memo = new HashMap<>();
    // public int numRollsToTarget(int d, int f, int target) {
    //     if (d == 0 && target == 0) {
    //         return 1;
    //     }
    //     if (d == 0 || target == 0) {
    //         return 0;
    //     }
    //     String str = d + " " + target;
    //     if (memo.containsKey(str)) {
    //         return memo.get(str);
    //     }
    //     int res = 0;
    //     for (int i = 1; i <= f; i++) {
    //         if (target >= i) {
    //             res = (res + numRollsToTarget(d - 1, f, target - i)) % MOD;
    //         } else {
    //             break;
    //         }
    //     }
    //     memo.put(str, res);
    //     return res;
    // }
impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        let mut memo: HashMap<(i32, i32), i32> = HashMap::new(); 
        fn recurse(val: i32, rolls: i32, k: i32, memo: &mut HashMap<(i32, i32), i32>) -> i32 {
            if val == 0 && rolls == 0 {
                1
            } else if val < 0 || rolls <= 0 {
                // dbg!("hit bottom: {} {}", val, rolls);
                0
            }
            else if memo.contains_key(&(val, rolls)) {
                *memo.get(&(val, rolls)).unwrap()
            }
            else {
                let mut res = 0;
                for x in 1..=k {
                    if val >= x {
                        res += recurse(val - x, rolls - 1, k, memo) % MOD;
                    } else {
                        break
                    }
                }
                let _ = memo.insert((val, rolls), res);
                res
            }
            // else {
            //     match memo.get(&(val, rolls)) {
            //         Some(n_ways) => *n_ways,
            //         None => {
            //             // let mut n_ways = 0;
            //             // for x in 1..=k {
            //             //     n_ways += 
            //             //
            //             // }
            //             let n_ways: i32 = (1..=k).into_iter()
            //                 .map(|roll| recurse(val-roll, rolls-1, k, memo) )
            //                 .sum();
            //             let _ = memo.insert((val, rolls), n_ways);
            //             n_ways % MOD
            //         }
            //     }
            // }
            // todo!();
        }
        recurse(target, n, k, &mut memo) 
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
