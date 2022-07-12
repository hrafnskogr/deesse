use std::arch::asm;


#[inline(always)]
pub fn syscall(syscall: usize) -> usize
{
    unsafe
    {
        let mut ret: usize;

        asm!(
            "mov    r10, rcx",
            "syscall",
            in("eax") syscall,

            lateout("rax") ret,
        );

        ret
    }
}

#[inline(always)]
pub fn syscall1(syscall: usize, arg1: usize) -> usize
{
    unsafe
    {
        let mut ret: usize;

        asm!(
            "mov    r10, rcx",
            "syscall",
            in("eax") syscall,
            in("rcx") arg1,

            lateout("rax") ret,
        );

        ret
    }
}

#[inline(always)]
pub fn syscall2(syscall: usize, arg1: usize, arg2: usize) -> usize
{
    unsafe
    {
        let mut ret: usize;

        asm!(
            "mov    r10, rcx",
            "syscall",
            in("eax") syscall,
            in("rcx") arg1,
            in("rdx") arg2,

            lateout("rax") ret,
        );

        ret
    }
}

#[inline(always)]
pub fn syscall3(syscall: usize, arg1: usize, arg2: usize, arg3: usize) -> usize
{
    unsafe
    {
        let mut ret: usize;

        asm!(
            "mov    r10, rcx",
            "syscall",
            in("eax") syscall,
            in("rcx") arg1,
            in("rdx") arg2,
            in("r8") arg3,

            lateout("rax") ret,
        );

        ret
    }
}

#[inline(always)]
pub fn syscall4(syscall: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> usize
{
    unsafe
    {
        let mut ret: usize;

        asm!(
            "mov    r10, rcx",
            "syscall",
            in("eax") syscall,
            in("rcx") arg1,
            in("rdx") arg2,
            in("r8") arg3,
            in("r9") arg4,

            lateout("rax") ret,
        );

        ret
    }
}
