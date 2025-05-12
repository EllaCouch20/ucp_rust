fn main() {
    #[cfg(not(target_arch="wasm32"))]
    {
        main::desktop_main()
    }
}
