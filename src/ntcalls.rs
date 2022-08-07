use std::cmp::{PartialOrd, Ordering};
use nt_utils::{peb::*, pe::*, err::PEErr};
use crate::err::*;


#[derive(Debug)]
pub struct NTApi
{
    //seed:         Vec<u8>,
    syscalls:        Vec<Syscall>,
}

#[derive(Debug, Clone)]
pub struct Syscall
{
    name: String,
    rva: usize,
    num: usize,
}

impl Syscall
{
    pub fn new(name: String, rva: usize, num: usize) -> Syscall
    {
        Syscall { name, rva, num }
    }
}

impl NTApi
{
    pub fn new() -> Result<NTApi, DSErr>
    {
        let mut nt_api = NTApi { syscalls: Vec::new() };
        nt_api.load()?;

        Ok(nt_api)
    }

    fn load(&mut self) -> Result<() , DSErr>
    {
        let ntdll: PEImage = match NTApi::get_ntdll()
        {
            Ok(module) => PEImage::new(module.dll_base),
            Err(e) => return Err(e),
        };

        //println!("{:#x?}", ntdll);

        self.populate_zw_syscalls(ntdll);
        self.sort_syscalls();
        self.assign_syscall_number();

        println!("{:#x?}", self);

        Ok(())
    }

    fn get_ntdll() -> Result<Module, DSErr>
    {
        let peb: Peb = Peb::new();
        let ldr: Ldr = peb.get_ldr();

        match ldr.in_load_order_module_list.find_module("ntdll.dll")
        {
            Ok(module) => Ok(module),
            Err(_) => Err( DSErr { message: String::from("Failed to find ntdll.dll") } ),
        }
    }

    fn populate_zw_syscalls(&mut self, ntdll: PEImage)
    {
        for ord in &ntdll
        {
            let fname = ntdll.fname_from_ord(ord);

            if fname.starts_with("Zw")
            {
                self.syscalls.push(Syscall::new(String::from(fname.replacen("Zw", "Nt", 1)),
                                   ntdll.faddr_from_ord(ord) - ntdll.base_addr,
                                   0));
                //self.syscalls.push(Syscall::new(String::from(fname), ntdll.faddr_from_ord(ord), 0));

            }
        }
    }

    // Simple bubble sort
    fn sort_syscalls(&mut self)
    {
        let len = self.syscalls.len();
        let mut sorted = true;

        while sorted
        {
            sorted = false;
            for i in 0..len
            {
                if i < (len - 1)
                {
                    if self.syscalls[i].rva > self.syscalls[i+1].rva
                    {
                        self.syscalls.swap(i, i+1);
                        sorted = true;
                    }
                }
            }
        }
    }

    fn assign_syscall_number(&mut self)
    {
        for i in 0..self.syscalls.len()
        {
            self.syscalls[i].num = i; 
        }
    }
}

