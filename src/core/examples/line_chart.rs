use detaxine_charts::{
    charts::line_chart::line_chart::{DataPoint, LineCurveChart, LineCurveChartConfig, Series},
    use_chart_data,
};
use leptos::prelude::*;

fn main() {
    mount_to_body(|| {
        view! {
            <LineCurveChart
                data=use_chart_data(vec![
                    (
                        Series::new("Revenue", "#4f46e5"),
                        vec![
                            DataPoint::new(120),
                            DataPoint::new(85),
                            DataPoint::new(200),
                            DataPoint::new(150),
                            DataPoint::new(95),
                            DataPoint::new(175),
                        ],
                    ),
                    (
                        Series::new("Expenses", "#e11d48"),
                        vec![
                            DataPoint::new(80),
                            DataPoint::new(90),
                            DataPoint::new(110),
                            DataPoint::new(95),
                            DataPoint::new(120),
                            DataPoint::new(100),
                        ],
                    ),
                ]).signal()
                x=use_chart_data(vec![
                    "Jan".to_string(),
                    "Feb".to_string(),
                    "Mar".to_string(),
                    "Apr".to_string(),
                    "May".to_string(),
                    "Jun".to_string(),
                ]).signal()
                config=LineCurveChartConfig {
                    show_area_chart: true,
                    x_axis_title: "Month".to_string(),
                    y_axis_title: "Amount($)".to_string(),
                    ..Default::default()
                }
            />
        }
    });
}
