use leptos::{
    ev,
    html::{Canvas, Div},
    prelude::*,
};
use web_sys::{
    CanvasRenderingContext2d, HtmlCanvasElement, HtmlElement, wasm_bindgen::JsCast, window,
};

#[derive(Clone, Debug, PartialEq)]
pub struct LineCurveChartConfig {
    pub show_grid: bool,
    pub show_legend: bool,
    pub show_inflection_points: bool,
    pub show_x_axis: bool,
    pub show_y_axis: bool,
    pub show_x_axis_labels: bool,
    pub show_y_axis_labels: bool,
    pub stroke_width: f64,
    pub show_area_chart: bool,
    pub x_axis_title: String,
    pub y_axis_title: String,
}

impl Default for LineCurveChartConfig {
    fn default() -> Self {
        Self {
            show_grid: true,
            show_legend: true,
            show_inflection_points: true,
            show_x_axis: true,
            show_y_axis: true,
            show_x_axis_labels: true,
            show_y_axis_labels: true,
            stroke_width: 2.0,
            show_area_chart: false,
            x_axis_title: String::new(),
            y_axis_title: String::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DataPoint {
    pub y: i32,
}

impl DataPoint {
    pub fn new(y: i32) -> Self {
        Self { y }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Series {
    pub name: String,
    pub color: String,
}

impl Series {
    pub fn new(name: &str, color: &str) -> Self {
        Self {
            name: name.into(),
            color: color.into(),
        }
    }
}

#[derive(Clone, Debug)]
struct PointPos {
    x: f64,
    y: f64,
    label: String,
    value: i32,
    series_name: String,
}

fn get_context(canvas: &HtmlCanvasElement) -> Option<CanvasRenderingContext2d> {
    canvas
        .get_context("2d")
        .ok()??
        .dyn_into::<CanvasRenderingContext2d>()
        .ok()
}

#[component]
pub fn LineCurveChart(
    data: Vec<(Series, Vec<DataPoint>)>,
    x: Vec<String>,
    #[prop(optional, default = Default::default())] config: LineCurveChartConfig,
) -> impl IntoView {
    let canvas_ref = NodeRef::<Canvas>::new();
    let tooltip_ref = NodeRef::<Div>::new();
    let point_positions = StoredValue::new(Vec::<PointPos>::new());

    let data = data.clone();
    let x = x.clone();
    let config = config.clone();

    let draw = StoredValue::new(
        move |canvas: &HtmlCanvasElement, context: &CanvasRenderingContext2d| {
            let canvas = canvas.clone();
            let context = context.clone();
            let data = data.clone();
            let x = x.clone();
            let config = config.clone();
            move || {
                let Some(win) = window() else { return };
                let device_pixel_ratio = win.device_pixel_ratio();
                let Some(parent) = canvas.parent_element() else {
                    return;
                };
                let width = parent.client_width() as f64;
                let height = width * 0.6;

                canvas.set_width((width * device_pixel_ratio) as u32);
                canvas.set_height((height * device_pixel_ratio) as u32);

                if context.reset_transform().is_err() {
                    return;
                };
                if context
                    .scale(device_pixel_ratio, device_pixel_ratio)
                    .is_err()
                {
                    return;
                };

                let props = LineCurveChartProps { data, x, config };
                let positions = draw_multiline_chart(&context, width, height, &props);
                point_positions.set_value(positions);
            }
        },
    );

    Effect::new(move |_| {
        let Some(canvas) = canvas_ref.get() else {
            return;
        };
        let canvas: HtmlCanvasElement = canvas.into();
        let Some(context) = get_context(&canvas) else {
            return;
        };

        draw.get_value()(&canvas, &context)();
    });

    let resize_listener = window_event_listener(ev::resize, move |_| {
        let Some(canvas) = canvas_ref.get() else {
            return;
        };
        let canvas: HtmlCanvasElement = canvas.into();
        let Some(context) = get_context(&canvas) else {
            return;
        };

        draw.get_value()(&canvas, &context)();
    });

    let mousemove_listener = window_event_listener(ev::mousemove, move |e| {
        let Some(canvas) = canvas_ref.get() else {
            return;
        };
        let canvas: HtmlCanvasElement = canvas.into();
        let Some(tooltip) = tooltip_ref.get() else {
            return;
        };
        let Some(win) = window() else { return };

        let rect = canvas.get_bounding_client_rect();
        let x = e.client_x() as f64 - rect.left();
        let y = e.client_y() as f64 - rect.top();

        let device_pixel_ratio = win.device_pixel_ratio();
        let scale_x = canvas.client_width() as f64 / canvas.width() as f64 * device_pixel_ratio;
        let scale_y = canvas.client_height() as f64 / canvas.height() as f64 * device_pixel_ratio;
        let lx = x * scale_x;
        let ly = y * scale_y;

        // hit radius in logical pixels
        let hit_radius = 8.0;
        let hovered = point_positions.get_value().into_iter().find(|p| {
            let dx = lx - p.x;
            let dy = ly - p.y;
            (dx * dx + dy * dy).sqrt() <= hit_radius
        });

        let tooltip_el: HtmlElement = tooltip.into();
        let style = tooltip_el.style();
        if let Some(point) = hovered {
            let _ = style.set_property("display", "block");
            let _ = style.set_property("left", &format!("{}px", x + 10.0));
            let _ = style.set_property("top", &format!("{}px", y - 28.0));
            tooltip_el.set_inner_text(&format!(
                "{} — {}: {}",
                point.series_name, point.label, point.value
            ));
        } else {
            let _ = style.set_property("display", "none");
        }
    });

    on_cleanup(move || {
        resize_listener.remove();
        mousemove_listener.remove();
    });

    view! {
        <div style="width: 90%;">
            <div style="display: flex; flex-direction: row; gap: 5px; flex-wrap: wrap; margin-bottom: 4px;">
            </div>
            <div style="position: relative;">
                <canvas node_ref=canvas_ref style="width: 100%; height: 100%;"></canvas>
                <div
                    node_ref=tooltip_ref
                    style="
                        position: absolute;
                        display: none;
                        background: rgba(0,0,0,0.75);
                        color: white;
                        padding: 4px 8px;
                        border-radius: 4px;
                        font-size: 13px;
                        pointer-events: none;
                    "
                />
            </div>
        </div>
    }
}

fn draw_multiline_chart(
    context: &CanvasRenderingContext2d,
    width: f64,
    height: f64,
    props: &LineCurveChartProps,
) -> Vec<PointPos> {
    let datasets = &props.data;
    let axis_padding = 50.0;

    let max_value = datasets
        .iter()
        .flat_map(|(_, data)| data.iter().map(|d| d.y))
        .max()
        .unwrap_or(0) as f64
        * 1.2;

    let Some(first) = datasets.first() else {
        return vec![];
    };
    let num_points = first.1.len() as f64;
    if num_points < 2.0 {
        return vec![];
    };
    let point_spacing = (width - axis_padding * 2.0) / (num_points - 1.0);

    context.clear_rect(0.0, 0.0, width, height);

    // Draw x-axis
    if props.config.show_x_axis {
        context.set_stroke_style_str("#cccccc");
        context.set_line_width(1.0);
        context.begin_path();
        context.move_to(axis_padding, height - axis_padding);
        context.line_to(width, height - axis_padding);
        context.stroke();
    }

    // Draw y-axis
    if props.config.show_y_axis {
        context.set_stroke_style_str("#cccccc");
        context.set_line_width(1.0);
        context.begin_path();
        context.move_to(axis_padding, 0.0);
        context.line_to(axis_padding, height - axis_padding);
        context.stroke();
    }

    // Draw y-axis grid lines and labels
    let num_grid_lines = 5;
    let step_value = max_value / num_grid_lines as f64;
    let step_height = (height - axis_padding * 2.0) / num_grid_lines as f64;

    context.set_stroke_style_str("#cccccc");
    context.set_line_width(1.0);
    context.set_fill_style_str("black");
    context.set_text_align("right");
    context.set_text_baseline("middle");

    for i in 0..=num_grid_lines {
        let y = height - axis_padding - i as f64 * step_height;

        if props.config.show_grid {
            context.begin_path();
            context.move_to(axis_padding, y);
            context.line_to(width, y);
            context.stroke();
        }

        if props.config.show_y_axis_labels {
            let label = (i as f64 * step_value).round();
            let _ = context.fill_text(&format!("{}", label), axis_padding - 10.0, y);
        }
    }

    let mut point_positions = Vec::new();

    for (series, data) in datasets {
        context.set_stroke_style_str(series.color.as_str());
        context.set_line_width(props.config.stroke_width);

        context.begin_path();
        let first_y =
            height - axis_padding - (data[0].y as f64 / max_value) * (height - axis_padding * 2.0);
        context.move_to(axis_padding, first_y);

        for i in 1..data.len() {
            let x = axis_padding + i as f64 * point_spacing;
            let y = height
                - axis_padding
                - (data[i].y as f64 / max_value) * (height - axis_padding * 2.0);

            let prev_x = axis_padding + (i - 1) as f64 * point_spacing;
            let prev_y = height
                - axis_padding
                - (data[i - 1].y as f64 / max_value) * (height - axis_padding * 2.0);

            let ctrl1_x = prev_x + point_spacing / 3.0;
            let ctrl1_y = prev_y;
            let ctrl2_x = x - point_spacing / 3.0;
            let ctrl2_y = y;

            context.bezier_curve_to(ctrl1_x, ctrl1_y, ctrl2_x, ctrl2_y, x, y);
        }
        context.stroke();

        // Fill area below line
        if props.config.show_area_chart {
            context.line_to(
                axis_padding + (data.len() as f64 - 1.0) * point_spacing,
                height - axis_padding,
            );
            context.line_to(axis_padding, height - axis_padding);
            context.close_path();
            let fill_color = format!("{}33", &series.color);
            context.set_fill_style_str(&fill_color);
            context.fill();
        }

        // Draw inflection points and collect positions
        for (i, datapoint) in data.iter().enumerate() {
            let x = axis_padding + i as f64 * point_spacing;
            let y = height
                - axis_padding
                - (datapoint.y as f64 / max_value) * (height - axis_padding * 2.0);

            if props.config.show_inflection_points {
                context.set_fill_style_str(series.color.as_str());
                context.begin_path();
                let _ = context.arc(x, y, 3.0, 0.0, std::f64::consts::PI * 2.0);
                context.fill();
            }

            point_positions.push(PointPos {
                x,
                y,
                label: props.x.get(i).cloned().unwrap_or_default(),
                value: datapoint.y,
                series_name: series.name.clone(),
            });
        }
    }

    // Draw x-axis labels
    if props.config.show_x_axis_labels {
        context.set_fill_style_str("black");
        context.set_text_align("center");
        context.set_text_baseline("middle");

        for (i, x_label) in props.x.iter().enumerate() {
            let x = axis_padding + i as f64 * point_spacing;
            let y = height - axis_padding / 2.0;
            let _ = context.fill_text(x_label.as_str(), x, y);
        }
    }

    // Draw x-axis title
    if !props.config.x_axis_title.is_empty() {
        context.set_text_align("center");
        context.set_font("bold 12px Arial");
        let _ = context.fill_text(
            &props.config.x_axis_title,
            width / 2.0,
            height - axis_padding / 4.0,
        );
    }

    // Draw y-axis title
    if !props.config.y_axis_title.is_empty() {
        context.set_text_align("center");
        context.set_text_baseline("middle");
        context.set_font("bold 12px Arial");
        context.save();
        let _ = context.rotate(-std::f64::consts::PI / 2.0);
        let _ = context.fill_text(
            &props.config.y_axis_title,
            -(height / 2.0),
            axis_padding / 4.0,
        );
        context.restore();
    }

    point_positions
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    fn mock_context() -> Option<CanvasRenderingContext2d> {
        let document = web_sys::window()?.document()?;
        let canvas = document
            .create_element("canvas")
            .ok()?
            .dyn_into::<HtmlCanvasElement>()
            .ok()?;
        get_context(&canvas)
    }

    #[wasm_bindgen_test]
    fn test_draw_multiline_chart() {
        let Some(context) = mock_context() else {
            return;
        };
        let width = 800.0;
        let height = 600.0;

        let props = LineCurveChartProps {
            data: vec![
                (
                    Series::new("Dataset 1", "#ff0000"),
                    vec![
                        DataPoint::new(10),
                        DataPoint::new(20),
                        DataPoint::new(15),
                        DataPoint::new(40),
                        DataPoint::new(30),
                    ],
                ),
                (
                    Series::new("Dataset 2", "#00ff00"),
                    vec![
                        DataPoint::new(50),
                        DataPoint::new(40),
                        DataPoint::new(30),
                        DataPoint::new(35),
                        DataPoint::new(20),
                    ],
                ),
            ],
            x: vec!["0", "1", "2", "3", "4"]
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
            config: LineCurveChartConfig {
                show_area_chart: true,
                ..Default::default()
            },
        };

        draw_multiline_chart(&context, width, height, &props);
    }
}
