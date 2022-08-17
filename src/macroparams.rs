use syn::parse::{Parse, ParseBuffer};
use syn::{Ident, Token};

pub struct MacroParams
{
    pub syscall: String,
    pub params: Vec<String>,
}

impl Default for MacroParams
{
    fn default() -> Self
    {
        MacroParams
        {
                syscall: String::from(""),
                params: Vec::new(),
        }
    }
}

impl Parse for MacroParams
{
    fn parse(input: &ParseBuffer) -> syn::Result<Self>
    {
        let mparams = Punctuated::<Ident, Token![,]>::parse_terminated(input).unwrap();

        let ret_struct = MacroParams::default();

        println!("{:?}", mparams);
    }
}