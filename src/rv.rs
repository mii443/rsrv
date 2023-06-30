use std::{ffi::c_void, slice};

pub const RV_EIALIGN: u32 = 1;
pub const RV_EIFAULT: u32 = 2;
pub const RV_EILL: u32 = 3;
pub const RV_EBP: u32 = 4;
pub const RV_ELALIGN: u32 = 5;
pub const RV_ELFAULT: u32 = 6;
pub const RV_ESALIGN: u32 = 7;
pub const RV_ESFAULT: u32 = 8;
pub const RV_EECALL: u32 = 9;

pub const RV_OK: u32 = 0;
pub const RV_BAD: u32 = 1;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CSRS {
    pub mhartid: u32,
    pub mstatus: u32,
    pub mstatush: u32,
    pub mscratch: u32,
    pub mepc: u32,
    pub mcause: u32,
    pub mtval: u32,
    pub mip: u32,
    pub mtinst: u32,
    pub mtval2: u32,
    pub mtvec: u32,
    pub mie: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RV {
    rv_load_cb: extern "C" fn(*mut c_void, u32, u8) -> u32,
    rv_store_cb: extern "C" fn(*mut c_void, u32, *const u8) -> u32,
    r: [u32; 32],
    pc: u32,
    next_pc: u32,
    user: *mut c_void,
    csrs: CSRS,
}

extern "C" fn rv_load_cb(user: *mut c_void, addr: u32, data: *mut u8) -> u32 {
    if (addr - 0x80000000) > 0x10000 {
        return RV_BAD;
    }

    unsafe {
        *data = *(user.offset((addr - 0x80000000) as isize) as *mut u8);
    }

    RV_OK
}

extern "C" fn rv_store_cb(user: *mut c_void, addr: u32, data: u8) -> u32 {
    if (addr - 0x80000000) > 0x10000 {
        return RV_BAD;
    }

    unsafe {
        *(user.offset((addr - 0x80000000) as isize) as *mut u8) = data;
    }

    RV_OK
}

extern "C" {
    fn rv_init(
        cpu: *mut RV,
        user: *mut c_void,
        rv_load_cb: extern "C" fn(*mut c_void, u32, *mut u8) -> u32,
        rv_store_cb: extern "C" fn(*mut c_void, u32, u8) -> u32,
    );

    fn rv_step(cpu: *mut RV) -> u32;
}

impl RV {
    pub fn new(mem_size: usize, program: Vec<u32>) -> *mut RV {
        unsafe {
            let mem = libc::malloc(mem_size as libc::size_t) as *mut c_void;

            libc::memcpy(
                mem,
                program.as_ptr() as *mut c_void,
                std::mem::size_of_val(&program) as libc::size_t,
            );

            let rv = libc::malloc(std::mem::size_of::<RV>() as libc::size_t) as *mut RV;

            rv_init(rv, mem, rv_load_cb, rv_store_cb);

            rv
        }
    }

    pub fn step(cpu: *mut RV) -> u32 {
        unsafe { rv_step(cpu) }
    }

    pub fn get_r(cpu: *mut RV) -> [u32; 32] {
        let cpu = unsafe { *cpu };
        cpu.r
    }

    pub fn get_pc(cpu: *mut RV) -> u32 {
        let cpu = unsafe { *cpu };
        cpu.pc
    }

    pub fn get_next_pc(cpu: *mut RV) -> u32 {
        let cpu = unsafe { *cpu };
        cpu.next_pc
    }

    pub fn get_csrs(cpu: *mut RV) -> CSRS {
        let cpu = unsafe { *cpu };
        cpu.csrs
    }
}
