use detaxine_charts::{
    charts::bar_chart::bar_chart::{BarChart, BarChartConfig, DataPoint},
    use_chart_data,
};
use leptos::prelude::*;

fn main() {
    mount_to_body(|| {
        view! {
            <BarChart
                data=use_chart_data(vec![
                    DataPoint::new("Jan", 120),
                    DataPoint::new("Feb", 85),
                    DataPoint::new("Mar", 200),
                    DataPoint::new("Apr", 150),
                    DataPoint::new("May", 95),
                    DataPoint::new("Jun", 175),
                ]).signal()
                config=BarChartConfig::new("#4f46e5", "#e5e7eb", "#111827")
            />
        }
    });
}
