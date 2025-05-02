#![allow(unused_unsafe)]

#[cfg(target_arch = "powerpc64")]
#[cfg(target_endian = "big")]
#[cfg(target_os = "linux")]
#[link_section = ".text"]
#[naked]
pub unsafe extern "C" fn PushAllRegistersAndIterateStack(
    stack: *mut core::ffi::c_void,
    stack_visitor: *mut core::ffi::c_void,
    callback: extern "C" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void),
) {
    core::arch::asm!(
        ".text",
        ".align 2",
        ".globl PushAllRegistersAndIterateStack",
        ".type PushAllRegistersAndIterateStack, %function",
        ".hidden PushAllRegistersAndIterateStack",
        "PushAllRegistersAndIterateStack:",
        "  mflr 0",
        "  std 0, 16(1)",
        "  std 2, 24(1)",
        "  stdu 1, -256(1)",
        "  std 14, 112(1)",
        "  std 15, 120(1)",
        "  std 16, 128(1)",
        "  std 17, 136(1)",
        "  std 18, 144(1)",
        "  std 19, 152(1)",
        "  std 20, 160(1)",
        "  std 21, 168(1)",
        "  std 22, 176(1)",
        "  std 23, 184(1)",
        "  std 24, 192(1)",
        "  std 25, 200(1)",
        "  std 26, 208(1)",
        "  std 27, 216(1)",
        "  std 28, 224(1)",
        "  std 29, 232(1)",
        "  std 30, 240(1)",
        "  std 31, 248(1)",
        "  mr 6, 5",
        "  mr 5, 1",
        "  mr 12, 6",
        "  mtctr 6",
        "  bctrl",
        "  addi 1, 1, 256",
        "  ld 0, 16(1)",
        "  mtlr  0",
        "  ld 2, 24(1)",
        "  blr",
        options(noreturn)
    )
}

#[cfg(target_arch = "powerpc64")]
#[cfg(target_endian = "big")]
#[cfg(target_os = "aix")]
#[link_section = ".text[PR]"]
#[naked]
pub unsafe extern "C" fn PushAllRegistersAndIterateStack(
    stack: *mut core::ffi::c_void,
    stack_visitor: *mut core::ffi::c_void,
    callback: extern "C" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void),
) {
    core::arch::asm!(
        ".csect .text[PR]",
        ".align 2",
        ".globl .PushAllRegistersAndIterateStack, hidden",
        ".PushAllRegistersAndIterateStack:",
        "  mflr 0",
        "  std 0, 16(1)",
        "  std 2, 40(1)",
        "  stdu 1, -256(1)",
        "  std 14, 112(1)",
        "  std 15, 120(1)",
        "  std 16, 128(1)",
        "  std 17, 136(1)",
        "  std 18, 144(1)",
        "  std 19, 152(1)",
        "  std 20, 160(1)",
        "  std 21, 168(1)",
        "  std 22, 176(1)",
        "  std 23, 184(1)",
        "  std 24, 192(1)",
        "  std 25, 200(1)",
        "  std 26, 208(1)",
        "  std 27, 216(1)",
        "  std 28, 224(1)",
        "  std 29, 232(1)",
        "  std 30, 240(1)",
        "  std 31, 248(1)",
        "  mr 6, 5",
        "  ld 2,8(5)",
        "  ld 6,0(6)",
        "  mr 5, 1",
        "  mtctr 6",
        "  bctrl",
        "  addi 1, 1, 256",
        "  ld 0, 16(1)",
        "  mtlr  0",
        "  ld 2, 40(1)",
        "  blr",
        options(noreturn)
    )
}