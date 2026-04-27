#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
use core::arch::asm;

#[repr(C)]
#[derive(Clone, Copy)]
struct SbiRet {
	error: isize,
	value: usize,
}

impl SbiRet {
	fn into_result(self) -> Result<usize, isize> {
		if self.error == 0 {
			Ok(self.value)
		} else {
			Err(self.error)
		}
	}
}

fn sbi_call(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize, fid: usize, eid: usize) -> SbiRet {
    let mut error = arg0;
    let mut value = arg1;
    
    unsafe {
        asm!(
            "ecall",
            inlateout("a0") error,
            inlateout("a1") value,
            in("a2") arg2,
            in("a3") arg3,
            in("a4") arg4,
            in("a5") arg5,
            in("a6") fid,
            in("a7") eid,
        );
    }
	SbiRet { error: error as isize, value: value }	
}

fn put_char(ch: usize) {
    sbi_call(ch, 0, 0, 0, 0, 0, 0, 1);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_main() -> ! {
    let s = "\nHello World\n";

    for ch in s.chars() {
        put_char(ch as usize);
    }

    loop {
        unsafe {
            asm!("wfi");
        }
    }
}
