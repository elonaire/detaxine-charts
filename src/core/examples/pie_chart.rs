use detaxine_charts::charts::pie_chart::pie_chart::{DataPoint, PieChart, PieChartConfig};
use leptos::prelude::*;

fn main() {
    mount_to_body(|| {
        view! {
            <PieChart
                data=vec![
                    DataPoint::new("Housing", 35, "#4f46e5"),
                    DataPoint::new("Food", 20, "#e11d48"),
                    DataPoint::new("Transport", 15, "#0891b2"),
                    DataPoint::new("Healthcare", 10, "#16a34a"),
                    DataPoint::new("Entertainment", 12, "#d97706"),
                    DataPoint::new("Savings", 8, "#9333ea"),
                ]
                config=PieChartConfig {
                    show_legend: true,
                }
            />
        }
    });
}
