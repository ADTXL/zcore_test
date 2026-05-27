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
    /// If handle is 0, reads directly without resource validation.
    pub async fn sys_debug_read(
        &self,
        handle: HandleValue,
        mut buf: UserOutPtr<u8>,
        mut len: UserInOutPtr<u32>,
    ) -> ZxResult {
        debug!(
            "debug.read: handle={:#x}, buf=({:?}), len={:?}",
            handle, buf, len
        );

        // If handle is not 0, validate it as a ROOT resource
        if handle != 0 {
            let proc = self.thread.proc();
            proc.get_object::<Resource>(handle)?
                .validate(ResourceKind::ROOT)?;
        }

        let readlen = len.read()?;
        if readlen == 0 {
            return Ok(());
        }

        let mut vec = vec![0u8; readlen as usize];

        for i in 0..readlen as usize {
            let mut byte = [0u8; 1];
            let n = kernel_hal::console::console_read(&mut byte).await;
            if n != 1 {
                return Err(ZxError::IO);
            }
            vec[i] = match byte[0] {
                b'\r' => b'\n',
                c => c,
            };
        }

        buf.write_array(&vec)?;
        len.write(readlen)?;
        Ok(())
    }
}
