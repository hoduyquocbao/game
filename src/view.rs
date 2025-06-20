//! Component Dioxus chịu trách nhiệm render game.

use dioxus::prelude::*;

#[component]
pub fn GameView() -> Element {
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

/// Component hiển thị màn hình GameOver, lấy state và score từ props hoặc context
#[component]
pub fn GameOverView(score: u32) -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh; background: #111; color: #fff; font-size: 2rem;",
            h1 { style: "margin-bottom: 1rem;", "GAME OVER" }
            p { style: "margin-bottom: 2rem;", "Final Score: {score}" }
            p { style: "font-size: 1.2rem; color: #aaa;", "Press 'R' to Restart" }
        }
    }
}
