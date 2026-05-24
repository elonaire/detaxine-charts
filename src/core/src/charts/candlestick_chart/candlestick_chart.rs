use leptos::{
    ev,
    html::{Canvas, Div},
    prelude::*,
};
use web_sys::{
    CanvasRenderingContext2d, HtmlCanvasElement, HtmlElement, wasm_bindgen::JsCast, window,
};

#[derive(Clone, Debug, PartialEq)]
pub struct CandlestickChartConfig {
    pub bullish_color: String,
    pub bearish_color: String,
    pub wick_color: String,
    pub show_grid: bool,
}

impl Default for CandlestickChartConfig {
    fn default() -> Self {
        Self {
            bullish_color: "#16a34a".into(),
            bearish_color: "#e11d48".into(),
            wick_color: "#6b7280".into(),
            show_grid: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Candle {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub label: String,
}

impl Candle {
    pub fn new(label: &str, open: f64, high: f64, low: f64, close: f64) -> Self {
        Self {
            open,
            high,
            low,
            close,
            label: label.into(),
        }
    }
}

#[derive(Clone, Debug)]
struct CandlePos {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    label: String,
}

fn get_context(canvas: &HtmlCanvasElement) -> Option<CanvasRenderingContext2d> {
    canvas
        .get_context("2d")
        .ok()??
        .dyn_into::<CanvasRenderingContext2d>()
        .ok()
}

#[component]
pub fn CandlestickChart(
    data: Vec<Candle>,
    #[prop(optional, default = Default::default())] config: CandlestickChartConfig,
) -> impl IntoView {
    let canvas_ref = NodeRef::<Canvas>::new();
    let tooltip_ref = NodeRef::<Div>::new();
    let candle_positions = StoredValue::new(Vec::<CandlePos>::new());

    let total = data.len();
    let view_start = RwSignal::new(0usize);
    let view_end = RwSignal::new(total);

    // drag state
    let is_dragging = StoredValue::new(false);
    let drag_start_x = StoredValue::new(0.0f64);
    let drag_start_view = StoredValue::new((0usize, total));

    let data = StoredValue::new(data);
    let config = StoredValue::new(config);

    let redraw = move || {
        let Some(canvas) = canvas_ref.get() else {
            return;
        };
        let canvas: HtmlCanvasElement = canvas.into();
        let Some(context) = get_context(&canvas) else {
            return;
        };

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

        let start = view_start.get();
        let end = view_end.get();
        let slice = data.get_value()[start..end].to_vec();

        let props = CandlestickChartProps {
            data: slice,
            config: config.get_value(),
        };

        let candles = draw_candlestick_chart(&context, width, height, &props);
        candle_positions.set_value(candles);
    };

    // re-draw when view signals change
    Effect::new(move |_| {
        let _ = view_start.get();
        let _ = view_end.get();
        redraw();
    });

    let resize_listener = window_event_listener(ev::resize, move |_| {
        redraw();
    });

    // zoom on wheel
    let wheel_listener = window_event_listener(ev::wheel, move |e| {
        let Some(canvas) = canvas_ref.get() else {
            return;
        };
        let canvas: HtmlCanvasElement = canvas.into();

        let rect = canvas.get_bounding_client_rect();
        let x = e.client_x() as f64 - rect.left();
        let axis_padding = 60.0;
        let chart_width = canvas.client_width() as f64 - axis_padding * 2.0;

        let start = view_start.get();
        let end = view_end.get();
        let visible = end - start;

        let mouse_ratio = ((x - axis_padding) / chart_width).clamp(0.0, 1.0);
        let center = start + (mouse_ratio * visible as f64) as usize;

        // delta_y can be large on trackpads, normalise it
        let delta_y = e.delta_y().signum() as isize;
        let new_visible = ((visible as isize + delta_y).max(2) as usize).min(total);

        let new_start = center.saturating_sub((mouse_ratio * new_visible as f64) as usize);
        let new_end = (new_start + new_visible).min(total);
        let new_start = new_end.saturating_sub(new_visible);

        view_start.set(new_start);
        view_end.set(new_end);

        e.prevent_default();
    });

    // pan on drag
    let mousedown_listener = window_event_listener(ev::mousedown, move |e| {
        let Some(canvas) = canvas_ref.get() else {
            return;
        };
        let canvas: HtmlCanvasElement = canvas.into();
        let rect = canvas.get_bounding_client_rect();
        let x = e.client_x() as f64 - rect.left();

        is_dragging.set_value(true);
        drag_start_x.set_value(x);
        drag_start_view.set_value((view_start.get(), view_end.get()));
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

        // pan
        if is_dragging.get_value() {
            let axis_padding = 60.0;
            let chart_width = canvas.client_width() as f64 - axis_padding * 2.0;
            let (drag_start, drag_end) = drag_start_view.get_value();
            let visible = drag_end - drag_start;
            let candle_width = chart_width / visible as f64;
            let delta_candles = ((drag_start_x.get_value() - x) / candle_width).round() as isize;

            let new_start =
                (drag_start as isize + delta_candles).clamp(0, (total - visible) as isize) as usize;
            let new_end = new_start + visible;

            view_start.set(new_start);
            view_end.set(new_end);
        }

        // tooltip hit-test
        let device_pixel_ratio = win.device_pixel_ratio();
        let scale_x = canvas.client_width() as f64 / canvas.width() as f64 * device_pixel_ratio;
        let scale_y = canvas.client_height() as f64 / canvas.height() as f64 * device_pixel_ratio;
        let lx = x * scale_x;
        let ly = y * scale_y;

        let hovered = candle_positions
            .get_value()
            .into_iter()
            .find(|c| lx >= c.x && lx <= c.x + c.width && ly >= c.y && ly <= c.y + c.height);

        let tooltip_el: HtmlElement = tooltip.into();
        let style = tooltip_el.style();
        if let Some(candle) = hovered {
            let _ = style.set_property("display", "block");
            let _ = style.set_property("left", &format!("{}px", x + 10.0));
            let _ = style.set_property("top", &format!("{}px", y - 28.0));
            tooltip_el.set_inner_html(&format!(
                "<strong>{}</strong><br/>O: {:.2}  H: {:.2}  L: {:.2}  C: {:.2}",
                candle.label, candle.open, candle.high, candle.low, candle.close,
            ));
        } else {
            let _ = style.set_property("display", "none");
        }
    });

    let mouseup_listener = window_event_listener(ev::mouseup, move |_| {
        is_dragging.set_value(false);
    });

    on_cleanup(move || {
        resize_listener.remove();
        wheel_listener.remove();
        mousedown_listener.remove();
        mousemove_listener.remove();
        mouseup_listener.remove();
    });

    view! {
        <div style="width: 100%;">
            <div style="position: relative;">
                <canvas
                    node_ref=canvas_ref
                    style="width: 100%; height: 100%; cursor: grab;"
                />
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
                        line-height: 1.6;
                    "
                />
            </div>
            <div style="display: flex; gap: 8px; margin-top: 6px; font-size: 12px; color: #6b7280;">
                <span>"Scroll to zoom"</span>
                <span>"·"</span>
                <span>"Drag to pan"</span>
                <span>"·"</span>
                {move || {
                    let start = view_start.get();
                    let end = view_end.get();
                    format!("Showing {} of {} candles", end - start, total)
                }}
            </div>
        </div>
    }
}

fn draw_candlestick_chart(
    context: &CanvasRenderingContext2d,
    width: f64,
    height: f64,
    props: &CandlestickChartProps,
) -> Vec<CandlePos> {
    let data = &props.data;
    if data.is_empty() {
        return vec![];
    };

    let axis_padding = 60.0;
    let candle_margin = 0.2;

    let max_value = data
        .iter()
        .map(|c| c.high)
        .fold(f64::NEG_INFINITY, f64::max)
        * 1.05;
    let min_value = data.iter().map(|c| c.low).fold(f64::INFINITY, f64::min) * 0.95;
    let value_range = max_value - min_value;

    if value_range == 0.0 {
        return vec![];
    };

    let chart_width = width - axis_padding * 2.0;
    let chart_height = height - axis_padding * 2.0;
    let slot_width = chart_width / data.len() as f64;
    let candle_width = (slot_width * (1.0 - candle_margin)).max(0.5);

    let to_y = |value: f64| -> f64 {
        height - axis_padding - ((value - min_value) / value_range) * chart_height
    };

    context.clear_rect(0.0, 0.0, width, height);

    // y-axis grid and labels
    let num_grid_lines = 5;
    let step_value = (max_value - min_value) / num_grid_lines as f64;
    let step_height = chart_height / num_grid_lines as f64;

    context.set_line_width(1.0);
    context.set_fill_style_str("black");
    context.set_text_align("right");
    context.set_text_baseline("middle");

    for i in 0..=num_grid_lines {
        let y = height - axis_padding - i as f64 * step_height;
        let label = min_value + i as f64 * step_value;

        if props.config.show_grid {
            context.set_stroke_style_str("#e5e7eb");
            context.begin_path();
            context.move_to(axis_padding, y);
            context.line_to(width - axis_padding, y);
            context.stroke();
        }

        let _ = context.fill_text(&format!("{:.2}", label), axis_padding - 8.0, y);
    }

    // x-axis baseline
    context.set_stroke_style_str("#e5e7eb");
    context.begin_path();
    context.move_to(axis_padding, height - axis_padding);
    context.line_to(width - axis_padding, height - axis_padding);
    context.stroke();

    let mut candle_positions = Vec::new();

    for (i, candle) in data.iter().enumerate() {
        let slot_x = axis_padding + i as f64 * slot_width;
        let candle_x = slot_x + (slot_width - candle_width) / 2.0;
        let wick_x = candle_x + candle_width / 2.0;

        let is_bullish = candle.close >= candle.open;
        let color = if is_bullish {
            props.config.bullish_color.as_str()
        } else {
            props.config.bearish_color.as_str()
        };

        let body_top = to_y(candle.open.max(candle.close));
        let body_bottom = to_y(candle.open.min(candle.close));
        let body_height = (body_bottom - body_top).max(1.0);

        // upper wick
        context.set_stroke_style_str(props.config.wick_color.as_str());
        context.set_line_width(1.0_f64.min(candle_width));
        context.begin_path();
        context.move_to(wick_x, to_y(candle.high));
        context.line_to(wick_x, body_top);
        context.stroke();

        // lower wick
        context.begin_path();
        context.move_to(wick_x, body_bottom);
        context.line_to(wick_x, to_y(candle.low));
        context.stroke();

        // body — degrade to a line when candles are very thin
        if candle_width < 2.0 {
            context.set_stroke_style_str(color);
            context.set_line_width(candle_width);
            context.begin_path();
            context.move_to(wick_x, body_top);
            context.line_to(wick_x, body_bottom);
            context.stroke();
        } else {
            context.set_fill_style_str(color);
            context.fill_rect(candle_x, body_top, candle_width, body_height);
            context.set_stroke_style_str(color);
            context.set_line_width(1.0);
            context.stroke_rect(candle_x, body_top, candle_width, body_height);
        }

        // x-axis labels — slanted, hidden when too dense
        if slot_width > 30.0 {
            context.set_fill_style_str("black");
            context.set_text_align("right");
            context.set_text_baseline("middle");
            context.save();
            let _ = context.translate(wick_x, height - axis_padding / 2.0);
            let _ = context.rotate(-std::f64::consts::PI / 4.0);
            let _ = context.fill_text(&candle.label, 0.0, 0.0);
            context.restore();
        }

        candle_positions.push(CandlePos {
            x: candle_x,
            y: to_y(candle.high),
            width: candle_width,
            height: to_y(candle.low) - to_y(candle.high),
            open: candle.open,
            high: candle.high,
            low: candle.low,
            close: candle.close,
            label: candle.label.clone(),
        });
    }

    candle_positions
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
    fn test_draw_candlestick_chart() {
        let Some(context) = mock_context() else {
            return;
        };

        let props = CandlestickChartProps {
            data: vec![
                Candle::new("Mon", 100.0, 115.0, 95.0, 110.0),
                Candle::new("Tue", 110.0, 120.0, 105.0, 108.0),
                Candle::new("Wed", 108.0, 112.0, 100.0, 102.0),
                Candle::new("Thu", 102.0, 118.0, 101.0, 115.0),
                Candle::new("Fri", 115.0, 125.0, 113.0, 120.0),
            ],
            config: CandlestickChartConfig::default(),
        };

        draw_candlestick_chart(&context, 800.0, 480.0, &props);
    }
}
