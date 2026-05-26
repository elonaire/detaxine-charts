use std::time::Duration;

use detaxine_charts::{
    bar_chart::{BarChart, BarChartConfig, DataPoint as BarPoint},
    candlestick_chart::{Candle, CandlestickChart, CandlestickChartConfig},
    doughnut_chart::{DoughnutChart, DoughnutChartConfig},
    line_chart::{DataPoint as LinePoint, LineCurveChart, LineCurveChartConfig, Series},
    pie_chart::{DataPoint as PiePoint, PieChart, PieChartConfig},
    use_chart_data,
};
use leptos::prelude::*;
use web_sys::js_sys::Math;

fn initial_candles() -> Vec<Candle> {
    vec![
        // Week 1 - Accumulation phase
        Candle::new("Mar 1", 172.30, 174.50, 170.80, 173.20),
        Candle::new("Mar 2", 173.20, 176.80, 172.50, 176.10),
        Candle::new("Mar 3", 176.10, 177.30, 173.40, 174.00),
        Candle::new("Mar 4", 174.00, 175.20, 171.60, 172.10),
        Candle::new("Mar 5", 172.10, 173.80, 169.90, 170.50),
        // Week 2 - Bearish breakdown
        Candle::new("Mar 8", 170.50, 171.20, 165.30, 166.00),
        Candle::new("Mar 9", 166.00, 167.50, 162.80, 163.40),
        Candle::new("Mar 10", 163.40, 164.20, 159.60, 160.10),
        Candle::new("Mar 11", 160.10, 163.50, 158.90, 162.80),
        Candle::new("Mar 12", 162.80, 165.40, 161.20, 164.50),
        // Week 3 - Recovery
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
        // Week 5 - Profit taking
        Candle::new("Mar 29", 176.00, 178.30, 173.50, 174.20),
        Candle::new("Mar 30", 174.20, 175.80, 170.90, 171.50),
        Candle::new("Mar 31", 171.50, 174.60, 170.20, 173.80),
    ]
}

fn initial_volume() -> Vec<BarPoint> {
    vec![
        BarPoint::new("Mar 1", 82_400),
        BarPoint::new("Mar 2", 91_200),
        BarPoint::new("Mar 3", 78_900),
        BarPoint::new("Mar 4", 95_600),
        BarPoint::new("Mar 5", 88_100),
        BarPoint::new("Mar 8", 120_300),
        BarPoint::new("Mar 9", 145_700),
        BarPoint::new("Mar 10", 162_400),
        BarPoint::new("Mar 11", 138_900),
        BarPoint::new("Mar 12", 110_200),
        BarPoint::new("Mar 15", 98_700),
        BarPoint::new("Mar 16", 87_300),
        BarPoint::new("Mar 17", 76_500),
        BarPoint::new("Mar 18", 82_100),
        BarPoint::new("Mar 19", 91_400),
        BarPoint::new("Mar 22", 134_600),
        BarPoint::new("Mar 23", 158_200),
        BarPoint::new("Mar 24", 172_900),
        BarPoint::new("Mar 25", 143_500),
        BarPoint::new("Mar 26", 119_800),
        BarPoint::new("Mar 29", 102_300),
        BarPoint::new("Mar 30", 94_700),
        BarPoint::new("Mar 31", 88_500),
    ]
}

fn initial_metrics() -> Vec<(Series, Vec<LinePoint>)> {
    vec![
        (
            Series::new("Revenue", "#4f46e5"),
            vec![
                LinePoint::new(142),
                LinePoint::new(158),
                LinePoint::new(149),
                LinePoint::new(163),
                LinePoint::new(171),
                LinePoint::new(168),
                LinePoint::new(175),
            ],
        ),
        (
            Series::new("Expenses", "#e11d48"),
            vec![
                LinePoint::new(98),
                LinePoint::new(104),
                LinePoint::new(99),
                LinePoint::new(112),
                LinePoint::new(108),
                LinePoint::new(115),
                LinePoint::new(110),
            ],
        ),
    ]
}

fn initial_x_labels() -> Vec<String> {
    vec![
        "Mon".to_string(),
        "Tue".to_string(),
        "Wed".to_string(),
        "Thu".to_string(),
        "Fri".to_string(),
        "Sat".to_string(),
        "Sun".to_string(),
    ]
}

fn portfolio_allocation() -> Vec<PiePoint> {
    vec![
        PiePoint::new("Tech", 42, "#4f46e5"),
        PiePoint::new("Healthcare", 18, "#0891b2"),
        PiePoint::new("Finance", 15, "#16a34a"),
        PiePoint::new("Energy", 12, "#d97706"),
        PiePoint::new("Consumer", 8, "#e11d48"),
        PiePoint::new("Other", 5, "#9333ea"),
    ]
}

fn sector_exposure() -> Vec<(String, i32, String)> {
    vec![
        ("US Equities".to_string(), 45, "#4f46e5".to_string()),
        ("International".to_string(), 25, "#0891b2".to_string()),
        ("Fixed Income".to_string(), 15, "#16a34a".to_string()),
        ("Commodities".to_string(), 8, "#d97706".to_string()),
        ("Cash".to_string(), 7, "#9333ea".to_string()),
    ]
}

fn main() {
    mount_to_body(|| view! { <Dashboard /> });
}

#[component]
fn Dashboard() -> impl IntoView {
    let candles = use_chart_data(initial_candles());
    let volume = use_chart_data(initial_volume());
    let metrics = use_chart_data(initial_metrics());
    let x_labels = RwSignal::new(initial_x_labels());
    let allocation = use_chart_data(portfolio_allocation());
    let exposure = use_chart_data(sector_exposure());

    let live_candles = use_chart_data(vec![
        Candle::new("09:00", 172.30, 174.50, 170.80, 173.20),
        Candle::new("09:01", 173.20, 176.80, 172.50, 176.10),
        Candle::new("09:02", 176.10, 177.30, 173.40, 174.00),
    ]);
    let live_candles_signal = live_candles.signal();

    // simulate a new candle arriving every second
    let append_handle = set_interval_with_handle(
        move || {
            let current_val = live_candles_signal.get().clone();
            let last = current_val
                .last()
                .cloned()
                .unwrap_or(Candle::new("", 100.0, 105.0, 95.0, 100.0));
            let open = last.close;
            let close = open + (Math::random() - 0.5) * 4.0;
            let high = open.max(close) + Math::random() * 2.0;
            let low = (open.min(close) - Math::random() * 2.0).max(1.0);
            let label = format!("09:{:02}", current_val.len());

            live_candles.append(Candle::new(&label, open, high, low, close));
            live_candles.retain_last(500); // rolling window
        },
        Duration::from_secs(1),
    );

    on_cleanup(move || {
        if let Ok(handle) = append_handle {
            handle.clear();
        }
    });

    view! {
        <div style="
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 24px;
            padding: 24px;
            background: #f9fafb;
            min-height: 100vh;
            box-sizing: border-box;
        ">
            // header
            <div style="grid-column: 1 / -1;">
                <h1 style="margin: 0 0 4px 0; font-size: 22px; color: #111827;">"Detaxine Charts"</h1>
                <p style="margin: 0; font-size: 13px; color: #6b7280;">"Example Dashboard"</p>
            </div>

            // candlestick — full width
            <div style="background: white; border-radius: 8px; padding: 16px; box-shadow: 0 1px 3px rgba(0,0,0,0.08);">
                <h2 style="margin: 0 0 12px 0; font-size: 15px; color: #374151;">"Candlestick Chart"</h2>
                <CandlestickChart
                    data=candles.signal()
                    config=CandlestickChartConfig::default()
                />
            </div>

            // candlestick — full width
            <div style="background: white; border-radius: 8px; padding: 16px; box-shadow: 0 1px 3px rgba(0,0,0,0.08);">
                <h2 style="margin: 0 0 12px 0; font-size: 15px; color: #374151;">"Candlestick Chart - Live Data"</h2>
                <CandlestickChart
                    data=live_candles_signal.clone()
                    config=CandlestickChartConfig::default()
                />
            </div>

            // volume bar chart
            <div style="background: white; border-radius: 8px; padding: 16px; box-shadow: 0 1px 3px rgba(0,0,0,0.08);">
                <h2 style="margin: 0 0 12px 0; font-size: 15px; color: #374151;">"Bar Chart"</h2>
                <BarChart
                    data=volume.signal()
                    config=BarChartConfig::new("#6366f1", "#e5e7eb", "#111827")
                />
            </div>

            // revenue vs expenses line chart
            <div style="background: white; border-radius: 8px; padding: 16px; box-shadow: 0 1px 3px rgba(0,0,0,0.08);">
                <h2 style="margin: 0 0 12px 0; font-size: 15px; color: #374151;">"Line Chart"</h2>
                <LineCurveChart
                    data=metrics.signal()
                    x=x_labels.into()
                    config=LineCurveChartConfig {
                        show_area_chart: true,
                        x_axis_title: "Day".to_string(),
                        y_axis_title: "USD (k)".to_string(),
                        ..Default::default()
                    }
                />
            </div>

            // portfolio allocation pie
            <div style="background: white; border-radius: 8px; padding: 16px; box-shadow: 0 1px 3px rgba(0,0,0,0.08);">
                <h2 style="margin: 0 0 12px 0; font-size: 15px; color: #374151;">"Pie Chart"</h2>
                <PieChart
                    data=allocation.signal()
                    config=PieChartConfig { show_legend: true }
                />
            </div>

            // sector exposure doughnut
            <div style="background: white; border-radius: 8px; padding: 16px; box-shadow: 0 1px 3px rgba(0,0,0,0.08);">
                <h2 style="margin: 0 0 12px 0; font-size: 15px; color: #374151;">"Doughnut Chart"</h2>
                <DoughnutChart
                    data=exposure.signal()
                    config=DoughnutChartConfig { show_legend: true }
                />
            </div>
        </div>
    }
}
