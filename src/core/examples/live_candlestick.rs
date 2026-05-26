use std::time::Duration;

use detaxine_charts::{
    charts::candlestick_chart::candlestick_chart::{
        Candle, CandlestickChart, CandlestickChartConfig,
    },
    use_chart_data,
};
use leptos::prelude::*;
use web_sys::js_sys::Math;

fn main() {
    mount_to_body(|| {
        view! { <LiveCandlestickDemo /> }
    });
}

#[component]
fn LiveCandlestickDemo() -> impl IntoView {
    let candles = use_chart_data(vec![
        Candle::new("09:00", 172.30, 174.50, 170.80, 173.20),
        Candle::new("09:01", 173.20, 176.80, 172.50, 176.10),
        Candle::new("09:02", 176.10, 177.30, 173.40, 174.00),
    ]);
    let candles_signal = candles.signal();

    // simulate a new candle arriving every second
    let append_handle = set_interval_with_handle(
        move || {
            let current_val = candles_signal.get().clone();
            let last = current_val
                .last()
                .cloned()
                .unwrap_or(Candle::new("", 100.0, 105.0, 95.0, 100.0));
            let open = last.close;
            let close = open + (Math::random() - 0.5) * 4.0;
            let high = open.max(close) + Math::random() * 2.0;
            let low = (open.min(close) - Math::random() * 2.0).max(1.0);
            let label = format!("09:{:02}", current_val.len());

            candles.append(Candle::new(&label, open, high, low, close));
            candles.retain_last(500); // rolling window
        },
        Duration::from_secs(1),
    );

    on_cleanup(move || {
        if let Ok(handle) = append_handle {
            handle.clear();
        }
    });

    view! {
        <CandlestickChart
            data=candles_signal.clone()
            config=CandlestickChartConfig::default()
        />
    }
}
