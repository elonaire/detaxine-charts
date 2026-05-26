use detaxine_charts::{
    charts::candlestick_chart::candlestick_chart::{
        Candle, CandlestickChart, CandlestickChartConfig,
    },
    use_chart_data,
};
use leptos::prelude::*;

fn main() {
    mount_to_body(|| {
        view! {
            <CandlestickChart
                data=use_chart_data(vec![
                    // Week 1 - Accumulation phase
                                        Candle::new("Mar 1",  172.30, 174.50, 170.80, 173.20),
                                        Candle::new("Mar 2",  173.20, 176.80, 172.50, 176.10),
                                        Candle::new("Mar 3",  176.10, 177.30, 173.40, 174.00),
                                        Candle::new("Mar 4",  174.00, 175.20, 171.60, 172.10),
                                        Candle::new("Mar 5",  172.10, 173.80, 169.90, 170.50),

                                        // Week 2 - Bearish breakdown
                                        Candle::new("Mar 8",  170.50, 171.20, 165.30, 166.00),
                                        Candle::new("Mar 9",  166.00, 167.50, 162.80, 163.40),
                                        Candle::new("Mar 10", 163.40, 164.20, 159.60, 160.10),
                                        Candle::new("Mar 11", 160.10, 163.50, 158.90, 162.80),
                                        Candle::new("Mar 12", 162.80, 165.40, 161.20, 164.50),

                                        // Week 3 - Recovery and consolidation
                                        Candle::new("Mar 15", 164.50, 168.90, 163.80, 167.70),
                                        Candle::new("Mar 16", 167.70, 170.20, 166.50, 169.40),
                                        Candle::new("Mar 17", 169.40, 171.80, 168.10, 168.90),
                                        Candle::new("Mar 18", 168.90, 170.50, 167.30, 169.80),
                                        Candle::new("Mar 19", 169.80, 172.40, 169.10, 171.60),

                                        // Week 4 - Bullish breakout
                                        Candle::new("Mar 22", 171.60, 176.30, 171.20, 175.80),
                                        Candle::new("Mar 23", 175.80, 179.50, 174.90, 178.40),
                                        Candle::new("Mar 24", 178.40, 181.20, 177.30, 180.50),
                                        Candle::new("Mar 25", 180.50, 182.80, 178.60, 179.20),
                                        Candle::new("Mar 26", 179.20, 180.10, 175.40, 176.00),

                                        // Week 5 - Profit taking and rejection
                                        Candle::new("Mar 29", 176.00, 178.30, 173.50, 174.20),
                                        Candle::new("Mar 30", 174.20, 175.80, 170.90, 171.50),
                                        Candle::new("Mar 31", 171.50, 174.60, 170.20, 173.80),
                ]).signal()
                config=CandlestickChartConfig {
                    bullish_color: "#16a34a".to_string(),
                    bearish_color: "#e11d48".to_string(),
                    wick_color: "#6b7280".to_string(),
                    show_grid: true,
                }
            />
        }
    });
}
