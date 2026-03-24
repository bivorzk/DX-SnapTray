use dioxus::prelude::*;

// r=20 → circumference = 2π×20 ≈ 125.66
const C: f32 = 125.664;

#[component]
pub fn CountdownRing(remaining: u8, total: u8) -> Element {
    let frac = remaining as f32 / total as f32;
    let offset = C * (1.0 - frac);
    // colour shifts: green→orange→red as time runs out
    let color = if frac > 0.5 {
        "#FF6B35"
    } else if frac > 0.2 {
        "#FFC857"
    } else {
        "#ef4444"
    };

    rsx! {
        svg { view_box: "0 0 50 50", width: "68", height: "68",
            // track
            circle {
                cx: "25",
                cy: "25",
                r: "20",
                fill: "none",
                stroke: "#e9ecef",
                stroke_width: "4",
            }
            // progress arc
            circle {
                cx: "25",
                cy: "25",
                r: "20",
                fill: "none",
                stroke: "{color}",
                stroke_width: "4",
                stroke_dasharray: "{C}",
                stroke_dashoffset: "{offset:.2}",
                stroke_linecap: "round",
                transform: "rotate(-90 25 25)",
                class: "ring-arc",
            }
            // seconds label inside ring
            text {
                x: "25",
                y: "29",
                text_anchor: "middle",
                dominant_baseline: "middle",
                font_size: "13",
                font_weight: "800",
                fill: "{color}",
                "{remaining}"
            }
        }
    }
}
