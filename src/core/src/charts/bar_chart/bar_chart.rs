use leptos::{
    ev,
    html::{Canvas, Div},
    prelude::*,
};
use web_sys::{
    CanvasRenderingContext2d, HtmlCanvasElement, HtmlElement, wasm_bindgen::JsCast, window,
};

#[derive(Clone, Debug, PartialEq)]
pub struct BarChartConfig {
    pub bar_color: String,
    pub grid_color: String,
    pub axis_color: String,
}

impl Default for BarChartConfig {
    fn default() -> Self {
        Self {
            bar_color: "blue".into(),
            grid_color: "#cccccc".into(),
            axis_color: "black".into(),
        }
    }
}

impl BarChartConfig {
    pub fn new(bar_color: &str, grid_color: &str, axis_color: &str) -> Self {
        Self {
            bar_color: bar_color.into(),
            grid_color: grid_color.into(),
            axis_color: axis_color.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DataPoint {
    pub name: String,
    pub value: i32,
}

impl DataPoint {
    pub fn new(name: &str, value: i32) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }
}

#[derive(Clone, Debug)]
struct BarRect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    label: String,
    value: i32,
}

fn get_context(canvas: &HtmlCanvasElement) -> Option<CanvasRenderingContext2d> {
    canvas
        .get_context("2d")
        .ok()??
        .dyn_into::<CanvasRenderingContext2d>()
        .ok()
}

#[component]
pub fn BarChart(
    data: Vec<DataPoint>,
    #[prop(optional, default = Default::default())] config: BarChartConfig,
) -> impl IntoView {
    let canvas_ref = NodeRef::<Canvas>::new();
    let tooltip_ref = NodeRef::<Div>::new();
    let bar_rects = StoredValue::new(Vec::<BarRect>::new());

    let config = config.clone();
    let data = data.clone();
    let draw = StoredValue::new(
        move |canvas: &HtmlCanvasElement, context: &CanvasRenderingContext2d| {
            let canvas = canvas.clone();
            let context = context.clone();
            let data = data.clone();
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

                let props = BarChartProps { data, config };
                let rects = draw_bar_chart(&context, width, height, &props);
                bar_rects.set_value(rects);
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

        let hovered = bar_rects
            .get_value()
            .into_iter()
            .find(|b| lx >= b.x && lx <= b.x + b.width && ly >= b.y && ly <= b.y + b.height);

        let tooltip_el: HtmlElement = tooltip.into();
        let style = tooltip_el.style();
        if let Some(bar) = hovered {
            let _ = style.set_property("display", "block");
            let _ = style.set_property("left", &format!("{}px", x + 10.0));
            let _ = style.set_property("top", &format!("{}px", y - 28.0));
            tooltip_el.set_inner_text(&format!("{}: {}", bar.label, bar.value));
        } else {
            let _ = style.set_property("display", "none");
        }
    });

    on_cleanup(move || {
        resize_listener.remove();
        mousemove_listener.remove();
    });

    view! {
        <div style="position: relative; width: 90%;">
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
    }
}

fn draw_bar_chart(
    context: &CanvasRenderingContext2d,
    width: f64,
    height: f64,
    props: &BarChartProps,
) -> Vec<BarRect> {
    let data = props
        .data
        .iter()
        .map(|point| point.value)
        .collect::<Vec<i32>>();
    let num_bars = (data.len() + 2) as f64;
    let total_spacing = width * 0.1;
    let total_bar_width = width - total_spacing;
    let bar_width = total_bar_width / (num_bars * 3.0);
    let bar_spacing = total_spacing / (num_bars - 1.0);
    let axis_padding = 50.0;

    context.clear_rect(0.0, 0.0, width, height);

    let Some(&max_raw) = data.iter().max() else {
        return vec![];
    };
    let max_value = max_raw as f64 * 1.2;
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
        context.begin_path();
        context.move_to(axis_padding, y);
        context.line_to(width, y);
        context.stroke();

        let label = (i as f64 * step_value).round();
        let _ = context.fill_text(&format!("{}", label), axis_padding - 10.0, y);
    }

    let mut bar_rects = Vec::new();
    context.set_fill_style_str(props.config.bar_color.as_str());
    for (i, &value) in data.iter().enumerate() {
        let x = axis_padding + i as f64 * (bar_width + bar_spacing);
        let y = height - axis_padding - value as f64 * ((height - axis_padding * 2.0) / max_value);
        let bar_height = height - axis_padding - y;
        context.fill_rect(x, y, bar_width, bar_height);
        bar_rects.push(BarRect {
            x,
            y,
            width: bar_width,
            height: bar_height,
            label: props.data[i].name.clone(),
            value,
        });
    }

    context.set_fill_style_str("black");
    context.set_text_align("center");
    context.set_text_baseline("middle");
    for (i, point) in props.data.iter().enumerate() {
        let x = axis_padding + i as f64 * (bar_width + bar_spacing) + bar_width / 2.0;
        let y = height - axis_padding / 2.0;
        let _ = context.fill_text(&point.name, x, y);
    }

    bar_rects
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
    fn test_draw_bar_chart() {
        let Some(context) = mock_context() else {
            return;
        };
        let width = 500.0;
        let height = 400.0;

        let data = vec![
            DataPoint::new("A", 10),
            DataPoint::new("B", 20),
            DataPoint::new("C", 15),
        ];

        let props = BarChartProps {
            data,
            config: BarChartConfig::new("blue", "gray", "black"),
        };

        draw_bar_chart(&context, width, height, &props);
    }
}
