#[cfg(feature = "integration")]
mod integration {
    #[path = "basic/main.rs"]
    mod basic;
    #[path = "full_lifecycle/main.rs"]
    mod full_lifecyle;
}