use std::cmp;
use std::collections::HashMap;

// ################## BRUTE FORCE (Time Exceeded)
pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() < 2 { return 0 }
    let mut map : HashMap<i32, i32> = HashMap::new();
    max_profit_from_index(&prices, 0, 2, &mut map)
}

pub fn max_profit_from_index(prices: &[i32], index: usize, max_sell: i32, map: &mut HashMap<i32, i32>) -> i32 {
    if max_sell == 0 || index >= prices.len() { return 0 }
    let mut max = 0;
    for i in index..prices.len(){
        let current;
        if let Some(v) = map.get(&(i as i32)) {
            if max_sell == 1 {current = *v}
            else {
                current = max_profit_at_index(&prices, i, max_sell, map);
                map.insert(i as i32, current);
            }
        } else {
            current = max_profit_at_index(&prices, i, max_sell, map);
            map.insert(i as i32, current);
        }
        if current > max { max = current }
    }
    max
}

pub fn max_profit_at_index(prices: &[i32], index: usize, max_sell: i32, map: &mut HashMap<i32, i32>) -> i32 {
    let mut max = 0;
    for i in index+1..prices.len(){
        let current = prices[i] - prices[index] + max_profit_from_index(prices, i+1, max_sell - 1, map);
        if current > max { max = current }
    }
    max
}

// ############# OPTIMISED O(N) - from Xavier Elon
pub fn max_profit2(prices: Vec<i32>) -> i32 {
    let mut transaction1 = -prices[0];
    let mut transaction2 = -prices[0];
    let mut profit1 = 0;
    let mut total_profit = 0;

    for price in prices{
        transaction1 = cmp::max(transaction1, -price);
        profit1 = cmp::max(profit1, price + transaction1);
        transaction2 = cmp::max(transaction2, profit1 - price);
        total_profit = cmp::max(total_profit, price + transaction2)
    }
    total_profit
}
