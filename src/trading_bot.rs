use std::error::Error;
use crate::loggers::Logger;
use crate::markets::Market;
use crate::trading_strategy::TradingStrategy;

pub struct TradingBot {
    pub strategy: Box<dyn TradingStrategy>,
    pub market: Box<dyn Market>,
    pub state: TradingState,
    pub logger: Box<dyn Logger>,
}

pub enum TradingState {
    Buy,
    Sell,
}

impl TradingBot {
    // main trading logic
    // current alg: buy low, sell high
    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let current_price = self.market.get_market_price().await?;
        self.logger.write_to_log(format!("[PRICE] current market price: {:?} $", current_price)).await?;

        if self.strategy.should_buy(&self.state, current_price) {
            self.try_to_buy(0.0).await?;
        }

        Ok(())
    }

    async fn try_to_buy(&mut self, diff: f32) -> Result<f32, Box<dyn Error>> {
        Ok(0.0)
    }

    pub fn new(strategy: Box<dyn TradingStrategy>, market: Box<dyn Market>, logger: Box<dyn Logger>) -> Self {
        // initial state Buy?
        TradingBot {
            strategy,
            market,
            state: TradingState::Buy,
            logger
        }
    }
}