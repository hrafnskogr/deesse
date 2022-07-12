use pe_utils::{pe_image::*, mem_utils};


fn init_syscalls()
{
    let ntdll: PEImage = PEImage::new(get_ntdll_base());

}

fn get_ntdll_base() -> usize
{
    0
}



