use arcis::*;

#[encrypted]
mod perps_engine {
    use arcis::*;

    pub struct PositionData {
        pub entry_price: u64, // Entry price
        pub size: u64,        // Position size (quantity)
        pub collateral: u64,  // Collateral
        pub side: u64,        // 1 = Long, 2 = Short
    }

    pub struct MarketData {
        pub mark_price: u64,  // Current mark price
        pub maint_margin_bps: u64, // Maintenance margin rate (basis points, e.g., 500 = 5%)
    }

    pub struct RiskResult {
        pub is_liquidatable: u64, // 1 = Yes, 0 = No
        pub equity: u64,          // Current equity (Collateral + PnL)
        pub pnl_is_positive: u64, // PnL direction (positive or negative)
    }

    #[instruction]
    pub fn check_risk(
        pos_ctxt: Enc<Shared, PositionData>,
        mkt_ctxt: Enc<Shared, MarketData>
    ) -> Enc<Shared, RiskResult> {
        let pos = pos_ctxt.to_arcis();
        let mkt = mkt_ctxt.to_arcis();

        // --- 1. Calculate unrealized PnL (uPnL) ---
        // This logic avoids negative numbers and directly calculates Delta
        
        let is_long = pos.side == 1;
        
        // Long PnL = (Mark - Entry) * Size
        // Short PnL = (Entry - Mark) * Size
        
        // Calculate price difference (Delta) and direction
        // If Long and Mark > Entry -> Profit
        // If Short and Entry > Mark -> Profit
        
        let mark_gt_entry = mkt.mark_price >= pos.entry_price;
        
        let (delta, is_profit) = if is_long {
            if mark_gt_entry {
                (mkt.mark_price - pos.entry_price, 1u64) // Long profit
            } else {
                (pos.entry_price - mkt.mark_price, 0u64) // Long loss
            }
        } else { // Short
            if mark_gt_entry {
                (mkt.mark_price - pos.entry_price, 0u64) // Short loss
            } else {
                (pos.entry_price - mkt.mark_price, 1u64) // Short profit
            }
        };

        let abs_pnl = delta * pos.size;

        // --- 2. Calculate dynamic equity ---
        // Equity = Collateral +/- PnL
        
        let equity = if is_profit == 1 {
            pos.collateral + abs_pnl
        } else {
            // If loss > collateral, equity becomes zero (bankruptcy)
            if pos.collateral >= abs_pnl {
                pos.collateral - abs_pnl
            } else {
                0u64
            }
        };

        // --- 3. Maintenance margin requirement ---
        // MM = (MarkPrice * Size) * Ratio / 10000
        let position_value = mkt.mark_price * pos.size;
        let maint_req = (position_value * mkt.maint_margin_bps) / 10000;

        // --- 4. Liquidation determination ---
        let is_unsafe = equity < maint_req;
        
        let result = RiskResult {
            is_liquidatable: if is_unsafe { 1u64 } else { 0u64 },
            equity: equity,
            pnl_is_positive: is_profit,
        };

        // Encrypt the result and return it to the requester (Keeper or Frontend)
        pos_ctxt.owner.from_arcis(result)
    }
}