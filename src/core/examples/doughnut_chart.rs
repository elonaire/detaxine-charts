use detaxine_charts::charts::doughnut_chart::doughnut_chart::{DoughnutChart, DoughnutChartConfig};
use leptos::prelude::*;

fn main() {
    mount_to_body(|| {
        view! {
            <DoughnutChart
                data=vec![
                    ("Housing".to_string(), 35, "#4f46e5".to_string()),
                    ("Food".to_string(), 20, "#e11d48".to_string()),
                    ("Transport".to_string(), 15, "#0891b2".to_string()),
                    ("Healthcare".to_string(), 10, "#16a34a".to_string()),
                    ("Entertainment".to_string(), 12, "#d97706".to_string()),
                    ("Savings".to_string(), 8, "#9333ea".to_string()),
                ]
                config=DoughnutChartConfig {
                    show_legend: true,
                }
            />
        }
    });
}
