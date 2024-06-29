use proc_macro::TokenStream;
use syn::DeriveInput;

#[cfg(test)]
mod tests {

}

fn impl_save(ast:DeriveInput) -> TokenStream{
    let ident = ast.ident;
    
    let token = quote::quote! {
        impl Save for #ident{
            fn save(&self,path:&Path) -> Result<()> {
                let content = serde_json::to_string(self).unwrap();
                fs::write(path,content)?;
                Ok(())
            }
        }
    };

    token.into()
}

#[proc_macro_derive(Save)]
pub fn a(input:TokenStream) -> TokenStream{
    let ast = syn::parse(input).unwrap();

    impl_save(ast)
}

fn impl_load(ast:DeriveInput) -> TokenStream{
    let ident = ast.ident;
    let token = quote::quote! {
        impl Load<'_> for #ident{
            fn load(path: &Path) -> Result<Box<Self>> {
                let content = fs::read_to_string(path)?;
                let config = serde_json::from_str::<Self>(&content)?;
                
                Ok(Box::new(config))
            }
        }
    };
    token.into()
}

#[proc_macro_derive(Load)]
pub fn b(input:TokenStream) -> TokenStream{
    let ast = syn::parse(input).unwrap();

    impl_load(ast)
}