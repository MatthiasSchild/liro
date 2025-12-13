use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Ident, ItemFn, Token,
    parse::{Parse, ParseStream},
};
struct Args {
    name: Ident,
    impl_block: Vec<ItemFn>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let _comma: Token![,] = input.parse()?;

        let content;
        syn::parenthesized!(content in input);
        let mut impls = Vec::new();
        while !content.is_empty() {
            impls.push(content.parse::<ItemFn>()?);
        }

        Ok(Args {
            name,
            impl_block: impls,
        })
    }
}

#[proc_macro]
pub fn make_repo(input: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(input as Args);
    let ident = args.name;
    let impl_block = args.impl_block;
    let ident_name = ident.to_string();
    let repo_ident_name = format!("{}Repo", ident_name);
    let repo_ident = Ident::new(&repo_ident_name, ident.span());
    let repo_ident_impl_name = format!("{}RepoImpl", ident_name);
    let repo_ident_impl = Ident::new(&repo_ident_impl_name, ident.span());

    let signatures = impl_block.iter().map(|f| {
        let sig = &f.sig;
        quote! { #sig; }
    });

    let expanded = quote!(
        #[cfg_attr(test, ::mockall::automock)]
        #[::async_trait::async_trait]
        pub trait #repo_ident: Send + Sync {
            #( #signatures )*
        }

        pub struct #repo_ident_impl {
            pub db: ::sea_orm::DatabaseConnection
        }

        impl #repo_ident_impl {
            pub fn new(db: ::sea_orm::DatabaseConnection) -> Self {
                Self { db }
            }
        }

        #[::async_trait::async_trait]
        impl #repo_ident for #repo_ident_impl {
            #( #impl_block )*
        }
    );

    expanded.into()
}
