use super::join_path_env;
use os_xtask_utils::{dir, CommandExt, Ext, Make};
use std::{
    collections::HashSet,
    ffi::{OsStr, OsString},
    fs,
};

impl super::LinuxRootfs {
    /// 将 libc-test 放入 rootfs。
    pub fn put_libc_test(&self) {
        // 递归 rootfs
        self.make(false);
        // 拷贝仓库
        let dir = self.path().join("libc-test");
        dir::rm(&dir).unwrap();
        dircpy::copy_dir("libc-test", &dir).unwrap();
        // 编译
        fs::copy(dir.join("config.mak.def"), dir.join("config.mak")).unwrap();
        Make::new()
            .j(usize::MAX)
            .env("ARCH", self.0.name())
            .env("CROSS_COMPILE", &format!("{}-linux-musl-", self.0.name()))
            .env(
                "PATH",
                join_path_env(&[self.0.linux_musl_cross().join("bin")]),
            )
            .current_dir(&dir)
            .invoke();
        // 删除 libc-test 不必要的文件
        let elf_path = OsString::from("src");
        let test_set = HashSet::from([
            OsString::from("api"),
            OsString::from("common"),
            OsString::from("math"),
            OsString::from("musl"),
            OsString::from("functional"),
            OsString::from("regression"),
        ]);

        fs::read_dir(&dir)
            .unwrap()
            .filter_map(Result::ok)
            .filter(|path| path.file_name() != elf_path)
            .for_each(|path| dir::rm(path.path()).unwrap());

        fs::read_dir(&dir.join(&elf_path))
            .unwrap()
            .filter_map(Result::ok)
            .filter(|path| !test_set.contains(&path.file_name()))
            .for_each(|path| dir::rm(path.path()).unwrap());

        for item in test_set {
            fs::read_dir(&dir.join(&elf_path).join(item))
                .unwrap()
                .filter_map(Result::ok)
                .filter(|path| !path.file_name().into_string().unwrap().ends_with(".exe"))
                .filter(|path| !path.file_name().into_string().unwrap().ends_with(".so"))
                .for_each(|path| dir::rm(path.path()).unwrap());
        }
    }

    /// 将其他测试放入 rootfs。
    pub fn put_other_test(&self) {
        // 递归 rootfs
        self.make(false);
        // build linux-syscall/test
        let bin = self.path().join("bin");
        let musl_cross = self
            .0
            .linux_musl_cross()
            .join("bin")
            .join(format!("{}-linux-musl-gcc", self.0.name()));
        fs::read_dir("linux-syscall/test")
            .unwrap()
            .filter_map(|res| res.ok())
            .map(|entry| entry.path())
            .filter(|path| path.extension().map_or(false, |ext| ext == OsStr::new("c")))
            .for_each(|c| {
                Ext::new(&musl_cross)
                    .arg(&c)
                    .arg("-o")
                    .arg(bin.join(c.file_stem().unwrap()))
                    .invoke()
            });
    }
}

