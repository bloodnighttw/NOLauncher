use proc_macro::TokenStream;
use syn::{DeriveInput, Meta};

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

fn impl_config(ast:DeriveInput) -> TokenStream{
    let ident = ast.ident;
    let attr = ast.attrs.iter().filter(
        |x| x.path().segments.len() == 1 && x.path().segments[0].ident == "save_path"
    ).nth(0).expect("required #[config_path(SavePath)] (SavePath is Enum) to use this derive!");
    
    let attr = match &attr.meta {
        Meta::List(a) => a.tokens.clone(), 
        _=> panic!("error while parsing argument!")
    };

    let token = quote::quote! {
        impl crate::utils::config::Storage<'_> for #ident{
            fn save_by_app(&self, app: &tauri::AppHandle) -> anyhow::Result<()> {
                let path = #attr.to_path(app)?;
                self.save(&path)?;
                Ok(())
            }

            fn load_by_app(app: &tauri::AppHandle) -> anyhow::Result<()> {
                let path = #attr.to_path(app)?;
                Self::load(&path)?;
                Ok(())
            }
        }
    };

    token.into()
}

#[proc_macro_derive(Storage, attributes(save_path))]
pub fn config_path(item: TokenStream) -> TokenStream {
    let ast:DeriveInput = syn::parse(item).unwrap();
    let implement = impl_config(ast);
    implement
}