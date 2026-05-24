//! SVG chart generation: line, scatter, bar, histogram, and heatmap.
//!
//! Each function accepts a [`ChartConfig`] to customize dimensions,
//! margins, title, axis labels, and grid.

const PALETTE: [&str; 10] = [
    "#2196F3", "#FF5722", "#4CAF50", "#FFC107", "#9C27B0", "#00BCD4", "#E91E63", "#8BC34A",
    "#FF9800", "#607D8B",
];

fn palette(i: usize) -> &'static str {
    PALETTE[i % PALETTE.len()]
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Configuration for SVG chart rendering.
#[derive(Debug, Clone)]
pub struct ChartConfig {
    /// Total SVG width in pixels.
    pub width: f64,
    /// Total SVG height in pixels.
    pub height: f64,
    /// Top margin in pixels.
    pub margin_top: f64,
    /// Right margin in pixels.
    pub margin_right: f64,
    /// Bottom margin in pixels.
    pub margin_bottom: f64,
    /// Left margin in pixels.
    pub margin_left: f64,
    /// Chart title.
    pub title: String,
    /// X-axis label.
    pub x_label: String,
    /// Y-axis label.
    pub y_label: String,
    /// Background color (CSS).
    pub background: String,
    /// Whether to draw grid lines.
    pub grid: bool,
    /// Base font size in pixels.
    pub font_size: f64,
}

impl Default for ChartConfig {
    fn default() -> Self {
        Self {
            width: 800.0,
            height: 500.0,
            margin_top: 50.0,
            margin_right: 30.0,
            margin_bottom: 60.0,
            margin_left: 70.0,
            title: String::new(),
            x_label: String::new(),
            y_label: String::new(),
            background: "#ffffff".into(),
            grid: true,
            font_size: 14.0,
        }
    }
}

impl ChartConfig {
    fn plot_w(&self) -> f64 {
        self.width - self.margin_left - self.margin_right
    }
    fn plot_h(&self) -> f64 {
        self.height - self.margin_top - self.margin_bottom
    }
}

fn nice_ticks(min: f64, max: f64, target_count: usize) -> Vec<f64> {
    if (max - min).abs() < 1e-15 {
        return vec![min];
    }
    let range = max - min;
    let rough_step = range / target_count as f64;
    let mag = 10f64.powf(rough_step.log10().floor());
    let frac = rough_step / mag;
    let nice = if frac <= 1.5 {
        1.0
    } else if frac <= 3.5 {
        2.0
    } else if frac <= 7.5 {
        5.0
    } else {
        10.0
    };
    let step = nice * mag;
    let lo = (min / step).floor() * step;
    let mut ticks = Vec::new();
    let mut v = lo;
    while v <= max + step * 0.01 {
        if v >= min - step * 0.01 {
            ticks.push(v);
        }
        v += step;
    }
    ticks
}

fn format_tick(v: f64) -> String {
    if v.abs() >= 1e6 || (v != 0.0 && v.abs() < 0.01) {
        format!("{:.2e}", v)
    } else if v == v.floor() {
        format!("{:.0}", v)
    } else {
        format!("{:.2}", v)
    }
}

fn data_range(data: &[f64]) -> (f64, f64) {
    let min = data.iter().copied().fold(f64::INFINITY, f64::min);
    let max = data.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    if (max - min).abs() < 1e-15 {
        (min - 1.0, max + 1.0)
    } else {
        let pad = (max - min) * 0.05;
        (min - pad, max + pad)
    }
}

fn svg_header(cfg: &ChartConfig) -> String {
    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {} {}\" width=\"{}\" height=\"{}\">\n\
         <rect width=\"100%\" height=\"100%\" fill=\"{}\"/>\n",
        cfg.width, cfg.height, cfg.width, cfg.height, cfg.background,
    )
}

fn svg_title(cfg: &ChartConfig) -> String {
    if cfg.title.is_empty() {
        return String::new();
    }
    format!(
        "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"{}\" font-weight=\"bold\">{}</text>\n",
        cfg.width / 2.0,
        cfg.margin_top / 2.0 + 5.0,
        cfg.font_size + 2.0,
        escape_xml(&cfg.title),
    )
}

fn svg_axes(cfg: &ChartConfig, x_min: f64, x_max: f64, y_min: f64, y_max: f64) -> String {
    let pw = cfg.plot_w();
    let ph = cfg.plot_h();
    let ml = cfg.margin_left;
    let mt = cfg.margin_top;
    let mut s = String::new();

    s.push_str(&format!(
        "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#333\" stroke-width=\"1.5\"/>\n",
        ml,
        mt,
        ml,
        mt + ph,
    ));
    s.push_str(&format!(
        "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#333\" stroke-width=\"1.5\"/>\n",
        ml,
        mt + ph,
        ml + pw,
        mt + ph,
    ));

    let xt = nice_ticks(x_min, x_max, 6);
    for &v in &xt {
        let x = ml + (v - x_min) / (x_max - x_min) * pw;
        s.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"{}\">{}</text>\n",
            x,
            mt + ph + 20.0,
            cfg.font_size - 2.0,
            format_tick(v),
        ));
        if cfg.grid {
            s.push_str(&format!(
                "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#ddd\" stroke-width=\"0.5\"/>\n",
                x, mt, x, mt + ph,
            ));
        }
    }

    let yt = nice_ticks(y_min, y_max, 5);
    for &v in &yt {
        let y = mt + ph - (v - y_min) / (y_max - y_min) * ph;
        s.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" text-anchor=\"end\" font-size=\"{}\">{}</text>\n",
            ml - 8.0,
            y + 4.0,
            cfg.font_size - 2.0,
            format_tick(v),
        ));
        if cfg.grid {
            s.push_str(&format!(
                "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#ddd\" stroke-width=\"0.5\"/>\n",
                ml, y, ml + pw, y,
            ));
        }
    }

    if !cfg.x_label.is_empty() {
        s.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"{}\">{}</text>\n",
            ml + pw / 2.0,
            cfg.height - 10.0,
            cfg.font_size,
            escape_xml(&cfg.x_label),
        ));
    }
    if !cfg.y_label.is_empty() {
        s.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"{}\" transform=\"rotate(-90,{},{})\">{}</text>\n",
            15.0, mt + ph / 2.0, cfg.font_size, 15.0, mt + ph / 2.0, escape_xml(&cfg.y_label),
        ));
    }

    s
}

/// Named data series for line charts.
#[derive(Debug, Clone)]
pub struct Series {
    /// Series display name (used in legend).
    pub name: String,
    /// X-coordinates.
    pub x: Vec<f64>,
    /// Y-coordinates.
    pub y: Vec<f64>,
}

/// Generates a multi-series SVG line chart.
pub fn line_chart(series: &[Series], cfg: &ChartConfig) -> String {
    let all_x: Vec<f64> = series.iter().flat_map(|s| s.x.iter().copied()).collect();
    let all_y: Vec<f64> = series.iter().flat_map(|s| s.y.iter().copied()).collect();
    if all_x.is_empty() {
        return String::from("<svg/>");
    }

    let (x_min, x_max) = data_range(&all_x);
    let (y_min, y_max) = data_range(&all_y);
    let pw = cfg.plot_w();
    let ph = cfg.plot_h();
    let ml = cfg.margin_left;
    let mt = cfg.margin_top;

    let mut svg = svg_header(cfg);
    svg.push_str(&svg_title(cfg));
    svg.push_str(&svg_axes(cfg, x_min, x_max, y_min, y_max));

    for (si, s) in series.iter().enumerate() {
        let color = palette(si);
        let mut path = String::new();
        for (i, (&xi, &yi)) in s.x.iter().zip(s.y.iter()).enumerate() {
            let px = ml + (xi - x_min) / (x_max - x_min) * pw;
            let py = mt + ph - (yi - y_min) / (y_max - y_min) * ph;
            if i == 0 {
                path.push_str(&format!("M{:.2},{:.2}", px, py));
            } else {
                path.push_str(&format!(" L{:.2},{:.2}", px, py));
            }
        }
        svg.push_str(&format!(
            "<path d=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"2\"/>\n",
            path, color,
        ));
    }

    if series.len() > 1 {
        for (si, s) in series.iter().enumerate() {
            let lx = ml + pw - 120.0;
            let ly = mt + 20.0 + si as f64 * 20.0;
            svg.push_str(&format!(
                "<rect x=\"{}\" y=\"{}\" width=\"12\" height=\"12\" fill=\"{}\"/>\n",
                lx,
                ly - 10.0,
                palette(si),
            ));
            svg.push_str(&format!(
                "<text x=\"{}\" y=\"{}\" font-size=\"{}\">{}</text>\n",
                lx + 18.0,
                ly,
                cfg.font_size - 2.0,
                escape_xml(&s.name),
            ));
        }
    }

    svg.push_str("</svg>");
    svg
}

/// Generates an SVG scatter plot.
pub fn scatter_plot(x: &[f64], y: &[f64], cfg: &ChartConfig) -> String {
    if x.is_empty() {
        return String::from("<svg/>");
    }
    let (x_min, x_max) = data_range(x);
    let (y_min, y_max) = data_range(y);
    let pw = cfg.plot_w();
    let ph = cfg.plot_h();
    let ml = cfg.margin_left;
    let mt = cfg.margin_top;

    let mut svg = svg_header(cfg);
    svg.push_str(&svg_title(cfg));
    svg.push_str(&svg_axes(cfg, x_min, x_max, y_min, y_max));

    for (&xi, &yi) in x.iter().zip(y.iter()) {
        let px = ml + (xi - x_min) / (x_max - x_min) * pw;
        let py = mt + ph - (yi - y_min) / (y_max - y_min) * ph;
        svg.push_str(&format!(
            "<circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"3\" fill=\"{}\" opacity=\"0.7\"/>\n",
            px, py, PALETTE[0],
        ));
    }

    svg.push_str("</svg>");
    svg
}

/// Generates an SVG bar chart from labels and values.
pub fn bar_chart(labels: &[&str], values: &[f64], cfg: &ChartConfig) -> String {
    if labels.is_empty() {
        return String::from("<svg/>");
    }
    let n = labels.len();
    let pw = cfg.plot_w();
    let ph = cfg.plot_h();
    let ml = cfg.margin_left;
    let mt = cfg.margin_top;

    let y_max = values.iter().copied().fold(0.0f64, f64::max) * 1.1;
    let y_min = 0.0f64;
    let bar_w = pw / n as f64 * 0.7;
    let gap = pw / n as f64 * 0.3;

    let mut svg = svg_header(cfg);
    svg.push_str(&svg_title(cfg));
    svg.push_str(&svg_axes(cfg, 0.0, n as f64, y_min, y_max));

    for (i, (&label, &val)) in labels.iter().zip(values.iter()).enumerate() {
        let x = ml + i as f64 * (bar_w + gap) + gap / 2.0;
        let h = if y_max > 0.0 { val / y_max * ph } else { 0.0 };
        let y = mt + ph - h;
        svg.push_str(&format!(
            "<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" fill=\"{}\" rx=\"2\"/>\n",
            x, y, bar_w, h, palette(i),
        ));
        svg.push_str(&format!(
            "<text x=\"{:.2}\" y=\"{}\" text-anchor=\"middle\" font-size=\"{}\">{}</text>\n",
            x + bar_w / 2.0,
            mt + ph + 18.0,
            cfg.font_size - 3.0,
            escape_xml(label),
        ));
    }

    svg.push_str("</svg>");
    svg
}

/// Generates an SVG histogram by binning the input data.
pub fn histogram(data: &[f64], bins: usize, cfg: &ChartConfig) -> String {
    if data.is_empty() || bins == 0 {
        return String::from("<svg/>");
    }
    let (d_min, d_max) = data_range(data);
    let bin_w = (d_max - d_min) / bins as f64;

    let mut counts = vec![0usize; bins];
    for &v in data {
        let idx = ((v - d_min) / bin_w).floor() as usize;
        let idx = idx.min(bins - 1);
        counts[idx] += 1;
    }

    let max_count = *counts.iter().max().unwrap_or(&1);
    let pw = cfg.plot_w();
    let ph = cfg.plot_h();
    let ml = cfg.margin_left;
    let mt = cfg.margin_top;
    let bar_px = pw / bins as f64;

    let mut svg = svg_header(cfg);
    svg.push_str(&svg_title(cfg));
    svg.push_str(&svg_axes(cfg, d_min, d_max, 0.0, max_count as f64));

    for (i, &c) in counts.iter().enumerate() {
        let x = ml + i as f64 * bar_px;
        let h = if max_count > 0 {
            c as f64 / max_count as f64 * ph
        } else {
            0.0
        };
        let y = mt + ph - h;
        svg.push_str(&format!(
            "<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" fill=\"{}\" stroke=\"#fff\" stroke-width=\"0.5\"/>\n",
            x, y, bar_px, h, PALETTE[0],
        ));
    }

    svg.push_str("</svg>");
    svg
}

/// Generates an SVG heatmap from a 2D matrix.
pub fn heatmap(matrix: &[Vec<f64>], cfg: &ChartConfig) -> String {
    if matrix.is_empty() {
        return String::from("<svg/>");
    }
    let rows = matrix.len();
    let cols = matrix[0].len();
    let pw = cfg.plot_w();
    let ph = cfg.plot_h();
    let ml = cfg.margin_left;
    let mt = cfg.margin_top;
    let cell_w = pw / cols as f64;
    let cell_h = ph / rows as f64;

    let all: Vec<f64> = matrix.iter().flat_map(|r| r.iter().copied()).collect();
    let v_min = all.iter().copied().fold(f64::INFINITY, f64::min);
    let v_max = all.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let range = if (v_max - v_min).abs() < 1e-15 {
        1.0
    } else {
        v_max - v_min
    };

    let mut svg = svg_header(cfg);
    svg.push_str(&svg_title(cfg));

    for (r, row) in matrix.iter().enumerate() {
        for (c, &val) in row.iter().enumerate() {
            let t = ((val - v_min) / range).clamp(0.0, 1.0);
            let red = (255.0 * t) as u8;
            let blue = (255.0 * (1.0 - t)) as u8;
            let x = ml + c as f64 * cell_w;
            let y = mt + r as f64 * cell_h;
            svg.push_str(&format!(
                "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"rgb({},0,{})\"/>\n",
                x, y, cell_w, cell_h, red, blue,
            ));
        }
    }

    svg.push_str("</svg>");
    svg
}
