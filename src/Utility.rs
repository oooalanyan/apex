use std::ffi::c_void;

pub fn read_memory<T>(base_address: usize, size: usize) -> Option<T> {
    let mut data = std::mem::MaybeUninit::<T>::uninit();
    let data_ptr = data.as_mut_ptr() as *mut c_void;

    if unsafe { nt_read_virtual_memory(base_address, data_ptr, size) } {
        Some(unsafe { data.assume_init() })
    } else {
        None
    }
}

pub fn write_memory<T>(base_address: usize, data: &T) -> bool {
    let data_ptr = data as *const T as *const c_void;
    unsafe { nt_write_virtual_memory(base_address, data_ptr, std::mem::size_of::<T>()) }
}

unsafe fn nt_read_virtual_memory(
    base_address: usize,
    buffer: *mut c_void,
    size: usize,
) -> bool {
    let mut bytes_read: usize = 0;
    let status = ntdll::NtReadVirtualMemory(
        GetCurrentProcess(),
        base_address as *const c_void,
        buffer,
        size,
        &mut bytes_read,
    );
    status as u32 == 0 && bytes_read == size
}

unsafe fn nt_write_virtual_memory(
    base_address: usize,
    buffer: *const c_void,
    size: usize,
) -> bool {
    let mut bytes_written: usize = 0;
    let status = ntdll::NtWriteVirtualMemory(
        GetCurrentProcess(),
        base_address as *mut c_void,
        buffer,
        size,
        &mut bytes_written,
    );
    status as u32 == 0 && bytes_written == size
}

extern "system" {
    fn GetCurrentProcess() -> *mut c_void;
}

mod ntdll {
    use std::ffi::c_void;

    extern "system" {
        pub fn NtReadVirtualMemory(
            ProcessHandle: *mut c_void,
            BaseAddress: *const c_void,
            Buffer: *mut c_void,
            BufferSize: usize,
            NumberOfBytesRead: *mut usize,
        ) -> isize;

        pub fn NtWriteVirtualMemory(
            ProcessHandle: *mut c_void,
            BaseAddress: *mut c_void,
            Buffer: *const c_void,
            BufferSize: usize,
            NumberOfBytesWritten: *mut usize,
        ) -> isize;
    }
}