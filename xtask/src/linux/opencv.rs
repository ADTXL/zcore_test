use super::join_path_env;
use crate::{commands::fetch_online, Arch, REPOS};
use os_xtask_utils::{dir, CommandExt, Ext, Git, Make};
use std::{fs, path::Path};

impl super::LinuxRootfs {
    pub fn put_ffmpeg(&self) {
        // 递归 rootfs
        let _musl = self.put_musl_libs();
        // 拉 ffmpeg
        let ffmpeg = REPOS.join("ffmpeg");
        if !ffmpeg.is_dir() {
            fetch_online!(ffmpeg, |tmp| {
                Git::clone("https://github.com/FFmpeg/FFmpeg.git")
                    .dir(tmp)
                    .branch("release/5.0")
                    .single_branch()
                    .depth(1)
                    .done()
            });
        }
        // 拷贝到目标路径
        let build = self.0.target().join("ffmpeg");
        dircpy::copy_dir(ffmpeg, &build).unwrap();
        // 构建
        // TODO: aarch64 ffmpeg build
        todo!("ffmpeg build for aarch64")
    }

    pub fn put_opencv(&self) {
        // 递归 rootfs
        let musl = self.put_musl_libs();
        // 拉 opencv
        let opencv = REPOS.join("opencv");
        if !opencv.is_dir() {
            fetch_online!(opencv, |tmp| {
                Git::clone("https://github.com/opencv/opencv.git")
                    .dir(tmp)
                    .single_branch()
                    .depth(1)
                    .done()
            });
        }
        let source = opencv.canonicalize().unwrap();
        let target = self.0.target().join("opencv");
        // 如果 Makefile 未生成，重新执行 cmake
        let cmake_needed = !target.join("Makefile").is_file();
        // 如果执行了 cmake 或安装目录不存在，需要 make
        let install_needed = cmake_needed || !target.join("install").is_dir();
        // 工具链
        let path_with_musl_gcc = join_path_env(&[musl.join("bin")]);
        //
        if cmake_needed {
            dir::clear(&target).unwrap();
            // ffmpeg 路径
            let ffmpeg = self.0.target().join("ffmpeg").join("install").join("lib");
            // 创建平台相关 cmake
            let platform_cmake = self.0.target().join("musl-gcc.toolchain.cmake");
            fs::write(&platform_cmake, self.opencv_cmake(&ffmpeg)).unwrap();
            // 执行
            let mut cmake = Ext::new("cmake");
            if ffmpeg.is_dir() {
                cmake.env(
                    "PKG_CONFIG_LIBDIR",
                    ffmpeg.join("pkgconfig").canonicalize().unwrap(),
                );
            }
            cmake
                .current_dir(&target)
                .arg(format!(
                    "-DCMAKE_TOOLCHAIN_FILE={}",
                    platform_cmake.canonicalize().unwrap().display()
                ))
                .arg("-DWITH_FFMPEG=ON")
                .arg("-DCMAKE_BUILD_TYPE=Release")
                .arg(format!(
                    "-DCMAKE_INSTALL_PREFIX={}",
                    target.canonicalize().unwrap().join("install").display(),
                ))
                .arg(source)
                .env("PATH", &path_with_musl_gcc)
                .invoke();
        }
        //
        if install_needed {
            Make::install()
                .current_dir(&target)
                .j(num_cpus::get().min(8)) // 不能用太多线程，以免爆内存
                .env("PATH", path_with_musl_gcc)
                .invoke();
        }
        // 拷贝
        self.put_libs(musl, target.join("install"));
    }

    /// 构造一个用于 opencv 构建的 cmake 文件。
    fn opencv_cmake(&self, ffmpeg: impl AsRef<Path>) -> String {
        // 不会写 cmake
        if !matches!(self.0, Arch::Aarch64) {
            todo!();
        }
        const HEAD: &str = "\
set(CMAKE_SYSTEM_NAME      \"Linux\")
set(CMAKE_SYSTEM_PROCESSOR \"aarch64\")

set(CMAKE_C_COMPILER   aarch64-linux-musl-gcc)
set(CMAKE_CXX_COMPILER aarch64-linux-musl-g++)

set(CMAKE_C_FLAGS   \"\" CACHE STRING \"\")
set(CMAKE_CXX_FLAGS \"\" CACHE STRING \"\")

set(CMAKE_C_FLAGS   \"${CMAKE_C_FLAGS}   ${CMAKE_PASS_TEST_FLAGS}\")
set(CMAKE_CXX_FLAGS \"${CMAKE_CXX_FLAGS} ${CMAKE_PASS_TEST_FLAGS}\")";

        let ffmpeg = ffmpeg.as_ref();
        if ffmpeg.is_dir() {
            format!(
                "\
{HEAD}

set(CMAKE_LD_FFMPEG_FLAGS  \"-Wl,-rpath-link,{}\")
set(CMAKE_EXE_LINKER_FLAGS \"${{CMAKE_EXE_LINKER_FLAGS}} ${{CMAKE_LD_FFMPEG_FLAGS}}\")",
                ffmpeg.canonicalize().unwrap().display()
            )
        } else {
            HEAD.into()
        }
    }
}
