use dioxus::prelude::*;

#[component]
pub fn Logo() -> Element {
    rsx! {
        div { class: "flex justify-center mb-4",
            svg { view_box: "0 0 500 140", width: "100%", height: "80",
                // Tray body
                rect {
                    x: "25",
                    y: "55",
                    width: "90",
                    height: "50",
                    rx: "6",
                    fill: "#FF6B35",
                }
                rect {
                    x: "30",
                    y: "60",
                    width: "35",
                    height: "40",
                    rx: "3",
                    fill: "#FFE5DC",
                }
                rect {
                    x: "70",
                    y: "60",
                    width: "20",
                    height: "18",
                    rx: "3",
                    fill: "#FFE5DC",
                }
                rect {
                    x: "95",
                    y: "60",
                    width: "20",
                    height: "18",
                    rx: "3",
                    fill: "#FFE5DC",
                }
                rect {
                    x: "70",
                    y: "82",
                    width: "45",
                    height: "18",
                    rx: "3",
                    fill: "#FFE5DC",
                }
                // Snap bolt
                path {
                    d: "M80 25 L65 52 L75 52 L60 80 L85 50 L75 50 L90 25 Z",
                    fill: "#FFC857",
                    stroke: "#FF6B35",
                    stroke_width: "2",
                    stroke_linejoin: "round",
                }
                // Wordmark
                text {
                    x: "150",
                    y: "85",
                    font_family: "system-ui, -apple-system, sans-serif",
                    font_size: "56",
                    font_weight: "bold",
                    fill: "#FF6B35",
                    letter_spacing: "-1",
                    "SnapTray"
                }
                text {
                    x: "150",
                    y: "110",
                    font_family: "system-ui, -apple-system, sans-serif",
                    font_size: "20",
                    fill: "#6C757D",
                    letter_spacing: "2",
                    "SCHOOL CAFETERIA ORDERING"
                }
            }
        }
    }
}
