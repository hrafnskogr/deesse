mod asmcalls;

fn main()
{
    let res: usize = asmcalls::syscall(0x26);
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
