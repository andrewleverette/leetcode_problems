use std::cmp;

/// Calculates the maximum profit from a given list of stock
/// prices where the the `nth` element is the price of a give stock
/// on day `n`.
/// 
/// # Arguments
/// 
/// * `prices` - List of stock prices
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut cost = std::i32::MIN;
    let mut prev_cost = 0;
    let mut profit = 0;
    let mut prev_profit = 0;

    for price in prices {
        prev_cost = cost;
        cost = cmp::max(cost, prev_profit - price);
        prev_profit = profit;
        profit = cmp::max(profit, prev_cost + price);
    }

    profit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let prices = vec![1,2,3,0,2];
        assert_eq!(max_profit(prices), 3);
    }

    #[test]
    fn test_example_2() {
        let prices = vec![1,4,2];
        assert_eq!(max_profit(prices), 3);
    }

    #[test]
    fn test_example_3() {
        let prices = vec![2,1,2,0,1];
        assert_eq!(max_profit(prices), 1);
    } 
}