use std::ffi::c_void;
use std::os::raw::{c_int, c_uint, c_ulong, c_ulonglong};
use windows::Win32::Foundation::BOOL;
use windows::Win32::System::Threading::{CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32};
use windows::Win32::System::Memory::{ReadProcessMemory, WriteProcessMemory};
use windows::Win32::System::Diagnostics::Debug::GetModuleBaseNameA;

fn get_base() -> Result<u64, ()> {
    let mut pe32: PROCESSENTRY32 = PROCESSENTRY32::default();
    pe32.dwSize = std::mem::size_of::<PROCESSENTRY32>() as c_uint;

    let snapshot = unsafe { CreateToolhelp32Snapshot(0x2, 0) };
    if snapshot == 0 {
        return Err(());
    }

    if unsafe { Process32First(snapshot, &mut pe32) }.as_bool() {
        loop {
            if unsafe { Process32Next(snapshot, &mut pe32) }.as_bool() {
                break;
            }
        }
    }

    unsafe { GetModuleBaseNameA(pe32.th32ProcessID, std::ptr::null_mut(), std::ptr::null_mut(), 0) };

    Ok(pe32.th32ProcessID as u64)
}

fn rpm64(offset: u64) -> Result<u64, ()> {
    let base = get_base()?;
    let mut value: c_ulonglong = 0;
    let mut bytes_read: c_ulong = 0;

    if unsafe {
        ReadProcessMemory(
            base,
            offset as *const c_void,
            &mut value as *mut c_ulonglong as *mut c_void,
            std::mem::size_of::<c_ulonglong>(),
            &mut bytes_read,
        )
    }
    .as_bool()
    {
        Ok(value)
    } else {
        Err(())
    }
}

fn rpm_string(offset: u64) -> Result<String, ()> {
    let base = get_base()?;
    let mut value = Vec::with_capacity(256);
    let mut bytes_read: c_ulong = 0;

    if unsafe {
        ReadProcessMemory(
            base,
            offset as *const c_void,
            value.as_mut_ptr() as *mut c_void,
            value.capacity(),
            &mut bytes_read,
        )
    }
    .as_bool()
    {
        unsafe { value.set_len(bytes_read as usize) };
        Ok(String::from_utf8_lossy(&value).to_string())
    } else {
        Err(())
    }
}

fn wpm_bool(offset: u64, value: bool) -> Result<(), ()> {
    let base = get_base()?;
    let value_as_int = value as c_int;

    if unsafe {
        WriteProcessMemory(
            base,
            offset as *mut c_void,
            &value_as_int as *const c_int as *const c_void,
            std::mem::size_of::<c_int>(),
            std::ptr::null_mut(),
        )
    }
    .as_bool()
    {
        Ok(())
    } else {
        Err(())
    }
}

fn wpm_float(offset: u64, value: f32) -> Result<(), ()> {
    let base = get_base()?;

    if unsafe {
        WriteProcessMemory(
            base,
            offset as *mut c_void,
            &value as *const f32 as *const c_void,
            std::mem::size_of::<f32>(),
            std::ptr::null_mut(),
        )
    }
    .as_bool()
    {
        Ok(())
    } else {
        Err(())
    }
}

fn wpm_int(offset: u64, value: i32) -> Result<(), ()> {
    let base = get_base()?;

    if unsafe {
        WriteProcessMemory(
            base,
            offset as *mut c_void,
            &value as *const i32 as *const c_void,
            std::mem::size_of::<i32>(),
            std::ptr::null_mut(),
        )
    }
    .as_bool()
    {
        Ok(())
    } else {
        Err(())
    }
}