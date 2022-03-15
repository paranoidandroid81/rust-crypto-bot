use crate::trading_bot::TradingState;

pub trait TradingStrategy {
    pub fn should_buy(&self, state: &TradingState, current_price: f32) -> bool;
}

pub struct BuyLowSellHighStrat {
    pub coin_name: String,
    pub last_op_price: f32,
    pub upward_threshold: f32,
    pub dip_threshold: f32,
}

impl TradingStrategy for BuyLowSellHighStrat {
    pub fn should_buy(&self, state: &TradingState, current_price: f32) -> bool {
        let perc_diff = (current_price - self.last_op_price)
            / self.last_op_price
            * 100 as f32;
        
        if perc_diff >= self.upward_threshold ||
           perc_diff <= self.dip_threshold {
            return true;
           }
        false
    }
}