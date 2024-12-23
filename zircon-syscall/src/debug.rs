use super::*;
use zircon_object::dev::*;

impl Syscall<'_> {
    /// Write debug info to the serial port.
    pub fn sys_debug_write(&self, buf: UserInPtr<u8>, len: usize) -> ZxResult {
        info!("debug.write: buf=({:?}; {:#x})", buf, len);
        kernel_hal::console::console_write_str(buf.as_str(len)?);
        Ok(())
    }

    /// Read debug info from the serial port.
    // pub async fn sys_debug_read(
    //     &self,
    //     handle: HandleValue,
    //     mut buf: UserOutPtr<u8>,
    //     buf_size: u32,
    //     mut actual: UserOutPtr<u32>,
    // ) -> ZxResult {
    //     info!(
    //         "debug.read: handle={:#x}, buf=({:?}; {:#x})",
    //         handle, buf, buf_size
    //     );
    //     let proc = self.thread.proc();
    //     proc.get_object::<Resource>(handle)?
    //         .validate(ResourceKind::ROOT)?;
    //     let mut vec = vec![0u8; buf_size as usize];
    //     let len = kernel_hal::console::console_read(&mut vec).await;
    //     buf.write_array(&vec[..len])?;
    //     actual.write(len as u32)?;
    //     Ok(())
    // }

    /// Read debug info from the serial port.
    pub async fn sys_debug_read(
        &self,
        handle: HandleValue,          // 资源句柄
        mut buf: UserOutPtr<u8>,      // 用户提供的输出缓冲区
        mut len: UserInOutPtr<u32>,   // 输入输出参数：期望读取/实际读取的长度
    ) -> ZxResult {
        info!(
            "debug.read: handle={:#x}, buf=({:?}), len={:?}",
            handle, buf, len
        );
        let proc = self.thread.proc();
        // 验证资源权限
        proc.get_object::<Resource>(handle)?
            .validate(ResourceKind::ROOT)?;

        // 从用户空间读取期望读取的长度
        let readlen = len.read()?;

        // 创建临时缓冲区用于存储读取的数据
        let mut vec = vec![0u8; readlen as usize];

        // 逐个字符读取并处理
        for i in 0..readlen as usize {
            let mut byte = [0u8; 1];
            // 异步读取单个字符
            let n = kernel_hal::console::console_read(&mut byte).await;

            // 处理读取错误（假设 n=0 表示错误）
            if n != 1 {
                return Err(ZxError::IO);
            }

            // 处理字符替换（\r -> \n）
            vec[i] = match byte[0] {
                b'\r' => b'\n',
                c => c,
            };
        }

        // 将处理后的数据写入用户缓冲区
        buf.write_array(&vec)?;

        // 将实际读取长度（与期望相同）写回用户空间
        len.write(readlen)?;

        Ok(())
        }
    
}
