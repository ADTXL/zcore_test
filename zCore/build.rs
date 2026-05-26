fn main() {
    #[cfg(feature = "link-user-img")]
    println!("cargo:rustc-env=USER_IMG=prebuilt/zircon/arm64/bringup_fuchsia.zbi");
}
