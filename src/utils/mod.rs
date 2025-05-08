#[cfg(target_family = "unix")]
pub(crate) fn is_elevated() -> bool {
    unsafe { libc::geteuid() == 0 }
}

#[cfg(not(target_family = "unix"))]
pub(crate) fn is_elevated() -> bool {
    // Give users the benefit of the doubt on non-Unix systems.
    true
}
