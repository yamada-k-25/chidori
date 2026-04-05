#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::naked_asm;

unsafe extern "C" {
    static __bss: u8;
    static __bss_end: u8;
    static __stack_top: u8;
}

fn bss() -> *mut u8 {
    core::ptr::addr_of!(__bss) as *mut u8
}

fn bss_end() -> *mut u8 {
    core::ptr::addr_of!(__bss_end) as *mut u8
}

fn memset(mut p: *mut u8, value: u8, mut size: u32) {
    while size > 0 {
        unsafe {
            p.write_volatile(value);
            p = p.add(1);
        }
        size -= 1;
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_main() -> ! {
    memset(bss(), 0, bss_end() as u32 - bss() as u32);

    loop {}
}

#[unsafe(link_section = ".text.boot")]
#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe extern "C" fn boot() -> ! {
    naked_asm!(
        "la sp, {stack_top}",
        "j kernel_main",
        stack_top = sym __stack_top
    )
}

