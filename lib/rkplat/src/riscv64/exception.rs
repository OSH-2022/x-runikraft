use super::reg;

#[no_mangle]
unsafe extern "C" fn __rkplat_exception_handle(cause: usize, regs: &reg::RegGenExcept) {
    panic!("exception {}\nregs={:?}",cause, regs);
}
