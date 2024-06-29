use proc_macro::TokenStream;
use syn::DeriveInput;

#[cfg(test)]
mod tests {

}

fn impl_save(ast:DeriveInput) -> TokenStream{
    let ident = ast.ident;
    
    let token = quote::quote! {
        
        impl crate::utils::config::Save for #ident{
            fn save(&self,path:&std::path::Path) -> anyhow::Result<()> {
                let content = serde_json::to_string(self).unwrap();
                std::fs::write(path,content)?;
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
        
        impl crate::utils::config::Load<'_> for #ident{
            fn load(path: &std::path::Path) -> anyhow::Result<Box<Self>> {
                let content = std::fs::read_to_string(path)?;
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