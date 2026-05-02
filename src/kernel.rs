#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
use core::arch::asm;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_main() -> ! {
    crate::print!("\n\nHello {}\n", "World!");

    loop {
        unsafe {
            asm!("wfi");
        }
    }
}
