use std::ffi::CStr;
use std::os::raw::c_char;
use winapi::shared::minwindef::DWORD64;
use winapi::um::winnt::FLOAT;
use lazy_static::lazy_static;
use winapi::shared::ntdef::NULL;

lazy_static! {
    static ref ENTITY_LIST_OFFSET: DWORD64 = 0x1F6CAB8;
    static ref GLOW_OFFSET: DWORD64 = 0x380;
    static ref GLOW_STATE_OFFSET: DWORD64 = 0x2F0;
    static ref GLOW_COLOR_R_OFFSET: DWORD64 = 0x1B0;
    static ref GLOW_COLOR_G_OFFSET: DWORD64 = 0x1B4;
    static ref GLOW_COLOR_B_OFFSET: DWORD64 = 0x1B8;
    static ref GLOW_TIME_START_OFFSET: DWORD64 = 0x2B0;
    static ref GLOW_TIME_END_OFFSET: DWORD64 = 0x2C8;
    static ref GLOW_DISTANCE_OFFSET: DWORD64 = 0x2DC;
    static ref ENTITY_HANDLE_OFFSET: DWORD64 = 0x500;
}

#[no_mangle]
pub unsafe extern "C" fn get_entity_by_id(ent: i32, base: DWORD64) -> DWORD64 {
    let entity_list = base + *ENTITY_LIST_OFFSET;
    let base_entity = rpm64(entity_list);
    if base_entity == 0 {
        return NULL;
    }
    rpm64(entity_list + (ent as DWORD64 * 0x20))
}

#[no_mangle]
pub unsafe extern "C" fn enable_highlight(entity: DWORD64, r: FLOAT, g: FLOAT, b: FLOAT) {
    wpm_bool(entity + *GLOW_OFFSET, true);
    wpm_int(entity + *GLOW_STATE_OFFSET, 1);
    wpm_float(entity + *GLOW_COLOR_R_OFFSET, r);
    wpm_float(entity + *GLOW_COLOR_G_OFFSET, g);
    wpm_float(entity + *GLOW_COLOR_B_OFFSET, b);

    for offset in (*GLOW_TIME_START_OFFSET..=*GLOW_TIME_END_OFFSET).step_by(4) {
        wpm_float(entity + offset, f32::MAX);
    }
    wpm_float(entity + *GLOW_DISTANCE_OFFSET, f32::MAX);
}

#[no_mangle]
pub unsafe extern "C" fn get_base() -> DWORD64 {
    // 获取基址的实现
    0
}

#[no_mangle]
pub unsafe extern "C" fn main() {
    let base = get_base();
    loop {
        for i in 0..100 {
            let entity = get_entity_by_id(i, base);
            if entity == 0 {
                continue;
            }
            let entity_handle = rpm64(entity + *ENTITY_HANDLE_OFFSET);
            let identifier = rpms(entity_handle);
            if strcmp(identifier.as_ptr(), "player\0") == 0 {
                enable_highlight(entity, 120.0, 0.0, 0.0);
            }
        }
    }
}

unsafe fn rpm64(addr: DWORD64) -> DWORD64 {
    // 读取内存的实现
    0
}

unsafe fn wpm_bool(addr: DWORD64, value: bool) {
    // 写入内存的实现
}

unsafe fn wpm_int(addr: DWORD64, value: i32) {
    // 写入内存的实现
}

unsafe fn wpm_float(addr: DWORD64, value: FLOAT) {
    // 写入内存的实现
}

unsafe fn rpms(addr: DWORD64) -> String {
    let cstr = CStr::from_ptr(addr as *const c_char);
    cstr.to_string_lossy().to_string()
}

unsafe fn strcmp(s1: *const c_char, s2: *const c_char) -> i32 {
    libc::strcmp(s1, s2)
}