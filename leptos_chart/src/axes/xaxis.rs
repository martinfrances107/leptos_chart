use crate::core::REM;
use leptos::{component, tracing, view, IntoView};
use theta_chart::coord::{Axes, Rec};
#[allow(non_snake_case)]
#[component]
pub fn XAxis(region: Rec, axes: Axes) -> impl IntoView {
    let vector = region.get_vector();
    let mut mark_origin_y = REM;
    let mut baseline = "text-before-edge";
    let mut style = "";
    let mut text_anchor = "middle";

    if vector.get_y() < 0. {
        mark_origin_y *= -1.;
        baseline = "text-after-edge";
    }

    if axes.style == "time-month".to_string() {
        style = "writing-mode: tb;";
        baseline = "";
        text_anchor = "";
    }

    view! {
        // Draw region of x-axis
        {
            #[cfg(feature = "debug")]
            {
                let path = format!("M {},{} l {},{} l {},{} l {},{} Z", 0, 0, vector.get_x(), 0, 0,vector.get_y(), -vector.get_x(), 0);
                view! {
                    <circle id="originX" cx="0" cy="0" r="3" />
                    <line x1="0" y1="0" x2=vector.get_x() y2=vector.get_y() style="stroke:#ff000033;stroke-width:1" />
                    <path id="regionX" d=path  fill="#ff000033" />
                }
            }
        }
        // Draw x-axis
        <g class="stick" dominant-baseline={baseline} text-anchor=text_anchor stroke="currentColor">
            <line x1="0" y1="0" x2=vector.get_x() y2="0" />
            <line x1="0" y1="0" x2="0" y2={mark_origin_y} />
                {
                    axes.sticks.into_iter().map(|stick| {
                        let dx = stick.value * vector.get_x();
                        view! {
                            <line x1=dx y1="0" x2=dx y2={mark_origin_y/2.} />
                            <text
                                y={mark_origin_y}
                                x={dx}
                                stroke="currentColor"
                                fill="currentColor"
                                style=style
                                fill="currentColor"
                                stroke="none"
                            >
                                {stick.label}
                            </text>
                        }
                    })
                    .collect::<Vec<_>>()
                }
        </g>
    }
}
