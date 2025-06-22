#![allow(unused_unsafe)]

#[cfg(all(target_arch = "x86", target_os = "windows"))]
#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleA(lpModuleName: *const u8) -> *mut u8;
    fn GetProcAddress(hModule: *mut u8, lpProcName: *const u8) -> *mut u8;
}

#[cfg(all(target_arch = "x86", not(target_os = "windows")))]
extern "C" {
    // Placeholder for dynamic linking on non-Windows
    // In a real scenario, you'd use `libloading` or similar crates to dynamically load this function.
    // For demonstration, we assume it's statically linked or resolved elsewhere.
    #[no_mangle]
    pub fn PushAllRegistersAndIterateStack(
        stack: *mut Stack,
        stack_visitor: *mut StackVisitor,
        callback: IterateStackCallback,
    );
}

// Define types based on context
pub type Stack = u32;
pub type StackVisitor = u32;
pub type IterateStackCallback = extern "C" fn(
    stack: *mut Stack,
    stack_visitor: *mut StackVisitor,
    esp: *mut u32
);


#[cfg(all(target_arch = "x86", target_os = "windows"))]
pub unsafe fn push_all_registers_and_iterate_stack(
    stack: *mut Stack,
    stack_visitor: *mut StackVisitor,
    callback: IterateStackCallback,
) {
    let module_name = b"v8.dll\0"; // Replace with actual DLL name if different
    let proc_name = b"_PushAllRegistersAndIterateStack\0";

    let module_handle = GetModuleHandleA(module_name.as_ptr());
    if module_handle.is_null() {
        panic!("Failed to get module handle");
    }

    let proc_address = GetProcAddress(module_handle, proc_name.as_ptr());
    if proc_address.is_null() {
        panic!("Failed to get procedure address");
    }

    let func: extern "C" fn(*mut Stack, *mut StackVisitor, IterateStackCallback) =
        std::mem::transmute(proc_address);

    func(stack, stack_visitor, callback);
}

#[cfg(all(target_arch = "x86", not(target_os = "windows")))]
pub unsafe fn push_all_registers_and_iterate_stack(
    stack: *mut Stack,
    stack_visitor: *mut StackVisitor,
    callback: IterateStackCallback,
) {
    // Directly call the function since it's assumed to be linked.
    PushAllRegistersAndIterateStack(stack, stack_visitor, callback);
}

#[cfg(all(target_arch = "x86", target_os = "windows"))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_all_registers_and_iterate_stack() {
        unsafe {
            let stack = 0 as *mut Stack;
            let stack_visitor = 0 as *mut StackVisitor;

            extern "C" fn callback(
                _stack: *mut Stack,
                _stack_visitor: *mut StackVisitor,
                esp: *mut u32
            ) {
                println!("Callback called! esp: {:?}", esp);
            }
            push_all_registers_and_iterate_stack(stack, stack_visitor, callback);
        }
    }
}

#[cfg(all(target_arch = "x86", not(target_os = "windows")))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_all_registers_and_iterate_stack() {
        unsafe {
            let stack = 0 as *mut Stack;
            let stack_visitor = 0 as *mut StackVisitor;

            extern "C" fn callback(
                _stack: *mut Stack,
                _stack_visitor: *mut StackVisitor,
                esp: *mut u32
            ) {
                println!("Callback called! esp: {:?}", esp);
            }
            push_all_registers_and_iterate_stack(stack, stack_visitor, callback);
        }
    }
}