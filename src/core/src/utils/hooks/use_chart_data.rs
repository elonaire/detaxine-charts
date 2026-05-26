use leptos::prelude::*;

/// A reactive data handle returned by `use_chart_data`.
/// Provides ergonomic methods to manipulate chart data
/// without exposing the underlying signal directly.
pub struct ChartData<T: Clone + Send + Sync + 'static> {
    signal: RwSignal<Vec<T>>,
}

impl<T: Clone + Send + Sync + 'static> ChartData<T> {
    /// Returns the underlying signal to pass directly into a chart component.
    pub fn signal(&self) -> Signal<Vec<T>> {
        self.signal.into()
    }

    /// Append a single item to the end of the data.
    pub fn append(&self, item: T) {
        self.signal.update(|data| data.push(item));
    }

    /// Append multiple items to the end of the data.
    pub fn extend(&self, items: impl IntoIterator<Item = T>) {
        self.signal.update(|data| data.extend(items));
    }

    /// Replace the item at the given index.
    pub fn update_at(&self, index: usize, item: T) {
        self.signal.update(|data| {
            if let Some(slot) = data.get_mut(index) {
                *slot = item;
            }
        });
    }

    /// Update the last item — useful for live candle updates where the
    /// current candle's high/low/close changes tick by tick before it closes.
    pub fn update_last(&self, item: T) {
        self.signal.update(|data| {
            if let Some(last) = data.last_mut() {
                *last = item;
            }
        });
    }

    /// Replace all data at once.
    pub fn set(&self, items: Vec<T>) {
        self.signal.set(items);
    }

    /// Remove the item at the given index.
    pub fn remove_at(&self, index: usize) {
        self.signal.update(|data| {
            if index < data.len() {
                data.remove(index);
            }
        });
    }

    /// Remove all items.
    pub fn clear(&self) {
        self.signal.update(|data| data.clear());
    }

    /// Keep only the last `n` items — useful for capping a live feed
    /// to a rolling window without unbounded memory growth.
    pub fn retain_last(&self, n: usize) {
        self.signal.update(|data| {
            if data.len() > n {
                let drain_to = data.len() - n;
                data.drain(..drain_to);
            }
        });
    }

    /// Returns the current length of the data.
    pub fn len(&self) -> usize {
        self.signal.get_untracked().len()
    }

    /// Returns true if the data is empty.
    pub fn is_empty(&self) -> bool {
        self.signal.get_untracked().is_empty()
    }

    /// Read the current data without tracking reactivity.
    pub fn get_untracked(&self) -> Vec<T> {
        self.signal.get_untracked()
    }
}

/// Creates a reactive chart data handle with an initial dataset.
///
/// # Example — static data
/// ```rust
/// use detaxine_charts::use_chart_data;
///
/// let data = use_chart_data(vec![
///     DataPoint::new("Jan", 120),
///     DataPoint::new("Feb", 85),
/// ]);
///
/// view! {
///     <BarChart data=data.signal() />
/// }
/// ```
///
/// # Example — live candlestick feed
/// ```rust
/// use std::time::Duration;
///
/// use detaxine_charts::{
///     charts::candlestick_chart::candlestick_chart::{
///         Candle, CandlestickChart, CandlestickChartConfig,
///     },
///     use_chart_data,
/// };
/// use leptos::prelude::*;
/// use web_sys::js_sys::Math;
///
/// let candles = use_chart_data(vec![
///     Candle::new("09:00", 172.30, 174.50, 170.80, 173.20),
///     Candle::new("09:01", 173.20, 176.80, 172.50, 176.10),
///     Candle::new("09:02", 176.10, 177.30, 173.40, 174.00),
/// ]);
///
/// let candles_signal = candles.signal();
///
/// // simulate a new candle arriving every second
/// let append_handle = set_interval_with_handle(
///     move || {
///         let current_val = candles_signal.get().clone();
///         let last = current_val
///             .last()
///             .cloned()
///             .unwrap_or(Candle::new("", 100.0, 105.0, 95.0, 100.0));
///         let open = last.close;
///         let close = open + (Math::random() - 0.5) * 4.0;
///         let high = open.max(close) + Math::random() * 2.0;
///         let low = (open.min(close) - Math::random() * 2.0).max(1.0);
///         let label = format!("09:{:02}", current_val.len());
///
///         candles.append(Candle::new(&label, open, high, low, close));
///         candles.retain_last(500); // rolling window
///     },
///     Duration::from_secs(1),
/// );
///
/// on_cleanup(move || {
///     if let Ok(handle) = append_handle {
///         handle.clear();
///     }
/// });
///
/// view! {
///     <CandlestickChart data=candles_signal.clone() />
/// }
/// ```
///
/// # Example — resetting on user action
/// ```rust
/// use detaxine_charts::use_chart_data;
///
/// let data = use_chart_data(vec![]);
///
/// let on_load = move |_| {
///     data.set(vec![]);
/// };
/// ```
pub fn use_chart_data<T: Clone + Send + Sync + 'static>(initial: Vec<T>) -> ChartData<T> {
    ChartData {
        signal: RwSignal::new(initial),
    }
}
