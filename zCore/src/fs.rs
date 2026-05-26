cfg_if! {
    if #[cfg(feature = "linux")] {
        use alloc::sync::Arc;
        use rcore_fs::vfs::FileSystem;

        #[cfg(feature = "libos")]
        pub fn rootfs() -> Arc<dyn FileSystem> {
            let rootfs = if let Ok(dir) = std::env::var("CARGO_MANIFEST_DIR") {
                std::path::Path::new(&dir).parent().unwrap().to_path_buf()
            } else {
                std::env::current_dir().unwrap()
            };
            rcore_fs_hostfs::HostFS::new(rootfs.join("rootfs").join("libos"))
        }

        #[cfg(not(feature = "libos"))]
        pub fn rootfs() -> Arc<dyn FileSystem> {
            use rcore_fs::dev::Device;

            let device: Arc<dyn Device> = {
                #[cfg(feature = "mock-disk")]
                {
                    let block = linux_object::fs::mock_block();
                    Arc::new(block)
                }
                #[cfg(not(feature = "mock-disk"))]
                {
                    use linux_object::fs::rcore_fs_wrapper::*;
                    let block = kernel_hal::drivers::all_block().first_unwrap();
                    Arc::new(BlockCache::new(Block::new(block), 0x100))
                }
            };
            info!("Opening the rootfs...");
            rcore_fs_sfs::SimpleFileSystem::open(device).expect("failed to open device SimpleFS")
        }
    }
}
