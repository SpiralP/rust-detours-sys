use detours_sys::*;
use std::{mem, os::raw::c_void};
use winapi::um::processthreadsapi::GetCurrentThread;

#[test]
fn it_works() {
    assert_eq!(target(), "not hacked");

    let mut original = target as *mut c_void;

    // add detour
    unsafe {
        DetourTransactionBegin();
        DetourUpdateThread(GetCurrentThread());
        DetourAttach(&mut original, hook as *mut c_void);
        assert!(DetourTransactionCommit() == 0);

        // "original" has been modified to point to the original code
        let original = mem::transmute::<_, extern "C" fn() -> &'static str>(original);
        assert_eq!(original(), "not hacked");
    }
    assert_eq!(target(), "hacked");

    // remove detour
    unsafe {
        DetourTransactionBegin();
        DetourUpdateThread(GetCurrentThread());
        DetourDetach(&mut original, hook as *mut c_void);
        assert!(DetourTransactionCommit() == 0);

        // "original" has been restored to its original address
        let original = mem::transmute::<_, extern "C" fn() -> &'static str>(original);
        assert_eq!(original(), "not hacked");
    }
    assert_eq!(target(), "not hacked");
}

extern "C" fn target() -> &'static str {
    "not hacked"
}

extern "C" fn hook() -> &'static str {
    "hacked"
}
