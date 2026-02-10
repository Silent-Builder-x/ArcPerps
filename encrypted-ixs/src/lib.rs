use arcis::*;

#[encrypted]
mod perps_engine {
    use arcis::*;

    pub struct PositionState {
        pub entry_price: u64,    // 加密的开仓价格 (x10^6)
        pub leverage: u64,       // 加密的杠杆倍数 (x10)
        pub collateral: u64,     // 加密的抵押品金额
    }

    pub struct MarketUpdate {
        pub mark_price: u64,     // 当前标记价格 (x10^6)
        pub maintenance_ratio: u64, // 维持保证金率 (例如 50 代表 5%)
    }

    pub struct LiquidationResult {
        pub is_liquidatable: u64, // 1 = 触发清算, 0 = 安全
        pub upnl_positive: u64,   // 1 = 盈利, 0 = 亏损
        pub abs_upnl: u64,        // 绝对盈亏金额
    }

    #[instruction]
    pub fn check_perp_liquidation(
        pos_ctxt: Enc<Shared, PositionState>,
        market_ctxt: Enc<Shared, MarketUpdate>
    ) -> Enc<Shared, LiquidationResult> {
        let pos = pos_ctxt.to_arcis();
        let market = market_ctxt.to_arcis();

        // 1. 计算未实现盈亏 (假设多单逻辑)
        // uPnL = (MarkPrice - EntryPrice) * Size
        // Size = (Collateral * Leverage) / EntryPrice
        let price_diff_is_pos = market.mark_price >= pos.entry_price;
        let abs_price_diff = if price_diff_is_pos {
            market.mark_price - pos.entry_price
        } else {
            pos.entry_price - market.mark_price
        };

        let size = (pos.collateral * pos.leverage) / 10; // 考虑杠杆缩放
        let abs_upnl = (abs_price_diff * size) / pos.entry_price;

        // 2. 计算当前权益 (Equity)
        let equity = if price_diff_is_pos {
            pos.collateral + abs_upnl
        } else {
            // 防止 equity 溢出为负数
            if pos.collateral >= abs_upnl { pos.collateral - abs_upnl } else { 0u64 }
        };

        // 3. 强平判定: Equity < MaintenanceMargin
        // MaintenanceMargin = Size * maintenance_ratio / 1000
        let maint_margin = (size * market.maintenance_ratio) / 1000;
        let liquidatable = if equity < maint_margin { 1u64 } else { 0u64 };

        let result = LiquidationResult {
            is_liquidatable: liquidatable,
            upnl_positive: if price_diff_is_pos { 1u64 } else { 0u64 },
            abs_upnl,
        };

        pos_ctxt.owner.from_arcis(result)
    }
}
