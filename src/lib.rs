#[macro_use]
pub mod asmcalls;
pub mod ntcalls;
pub mod err;

use nt_utils::pe::*;
use nt_utils::peb::*;

use crate::ntcalls::*;

#[macro_export]
macro_rules! ds
{
    ($func: literal, $($args:expr),*) =>
    {
        let mut num = 0;
        
        $(
            let a = $args;
            num += 1;
        )*

        println!("{} arguments", num);
        /*print!("print: {}", $func);
        
        $(
            print!(" {}", $args);
        )*

        print!("\n");*/

        println!("not implemented");
    }
}


pub fn init()
{
    let nt = match NTApi::new()
    {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };

}


