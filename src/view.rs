//! Component Dioxus chịu trách nhiệm render game.

use dioxus::prelude::*;

#[component]
fn GameView() -> Element {
    // Nếu vẫn lỗi, chuyển sang dùng use_shared_state hoặc use_signal
    // let world = use_shared_state::<World>(cx).unwrap();
    // hoặc
    // let world = use_signal(|| World::default());
    // Tạm thời chỉ render UI mẫu
    rsx! {
        div {
            p { "Score: 0" }
        }
    }
}
