use nt_utils::{peb::*, pe::*};
use sha256::digest;
use std::collections::HashMap;

use crate::err::*;


#[derive(Debug, Clone)]
pub struct NTApi
{
    syscalls:   HashMap<String, Syscall>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Syscall
{
    rva: usize,
    name: String,
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
        let mut nt_api = NTApi { syscalls: HashMap::new() };
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

        self.populate_zw_syscalls(ntdll);

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
        let mut buf: Vec<Syscall> = Vec::new();

        for (ord, idx) in &ntdll
        {
            let fname = ntdll.fname_from_index(idx);

            if fname.starts_with("Zw")
            {
                let sysc = String::from(fname.replacen("Zw", "Nt", 1));
                buf.push(Syscall::new(sysc,
                                      ntdll.rva_from_ord(ord),
                                      0));
            }
        }

        buf.sort_unstable();
        buf.iter_mut()
           .enumerate()
           .for_each(|(x, y)| {
                y.num = x;
           });

        for sc in buf
        {
            let sysc_name = format!("{}{}", crate::SALT, sc.name);
            let sysc_h = digest(sysc_name);
            self.syscalls.insert(sysc_h, sc.clone());
        }
    }

    pub fn get_syscall(&self, hfname: &str) -> usize
    {
        match self.syscalls.get(hfname)
        {
            Some(sysc) => sysc.num,
            _ => panic!("Could not find the system call"),
        }
    }
}


