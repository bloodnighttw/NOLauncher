use proc_macro::TokenStream;
use syn::{DeriveInput, ItemStruct, Meta};

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
    ).nth(0).expect("required #[save_path(SavePath)] (SavePath is Enum) to use this derive!");
    
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

            fn load_by_app(app: &tauri::AppHandle) -> anyhow::Result<Box<Self>> {
                let path = #attr.to_path(app)?;
                let temp = Self::load(&path)?;
                Ok(temp)
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

fn impl_time(ast:ItemStruct) -> TokenStream{
    let ident = ast.ident;
    let duration_filed = ast.fields.iter()
        .find(
            |x| -> bool {
                let has_attr = x.attrs.iter().find(
                    |i| i.path().segments.len() == 1 && i.path().segments[0].ident == "dur"
                );
                has_attr != None
            }
        ).expect("you should have attr #[dur] in your std::time::Duration field");
    
    let ident2 = duration_filed.ident.clone().unwrap();
    
    let token = quote::quote! {
        impl crate::utils::data::TimeSensitiveTrait for #ident{
            fn get_duration(&self) -> std::time::Duration {
                self.#ident2
            }
        }
    };
    
    token.into()
}

#[proc_macro_derive(TimeSensitive, attributes(dur))]
pub fn time_sensitive(item:TokenStream) -> TokenStream{
    let ast:ItemStruct = syn::parse(item).unwrap();
    let implement = impl_time(ast);
    implement
}