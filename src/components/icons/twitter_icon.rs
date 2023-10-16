use leptos::*;

#[component]
pub fn TwitterIcon(
    #[prop(default = 40)] size: u32,
    #[prop(default = "fill-black")] class: &'static str,
) -> impl IntoView {
    view! {
        <svg
            width=size
            height=size
            viewBox="0 0 512 512"
            class=class
            xmlns="http://www.w3.org/2000/svg"
        >
            <path d="M160.768 460.8c193.126 0 298.803-160.154 298.803-298.803 0-4.506 0-9.011-.205-13.517 20.48-14.746 38.298-33.382 52.43-54.477-18.842 8.397-39.118 13.927-60.417 16.589 21.709-12.902 38.298-33.587 46.285-58.163-20.275 12.083-42.803 20.685-66.765 25.395-19.251-20.48-46.49-33.178-76.595-33.178-57.958 0-105.062 47.104-105.062 105.063 0 8.192 1.024 16.179 2.662 23.961-87.245-4.3-164.66-46.284-216.474-109.772-9.01 15.564-14.13 33.587-14.13 52.838 0 36.454 18.636 68.608 46.694 87.45a106.224 106.224 0 01-47.514-13.108v1.434c0 50.79 36.25 93.389 84.173 103.014-8.807 2.458-18.023 3.687-27.648 3.687-6.759 0-13.312-.615-19.661-1.843 13.312 41.779 52.224 72.09 98.1 72.908-36.046 28.263-81.306 45.056-130.458 45.056-8.397 0-16.794-.41-24.986-1.433C46.285 443.392 101.58 460.8 160.768 460.8"></path>
        </svg>
    }
}