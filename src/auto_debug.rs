use darling::{ast::Data, FromDeriveInput, FromField};
use quote::quote;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(debug))]
struct AutoDebugInfo {
    ident: syn::Ident,
    generics: syn::Generics,
    data: Data<(), AutoDebugFieldsInfo>,
}

#[derive(Debug, FromField)]
#[darling(attributes(debug))]
struct AutoDebugFieldsInfo {
    ident: Option<syn::Ident>,
    // ty: syn::Type,
    #[darling(default)]
    skip: bool,
}

pub(crate) fn process_auto_debug(input: syn::DeriveInput) -> proc_macro2::TokenStream {
    let AutoDebugInfo {
        ident,
        generics,
        data: Data::Struct(fields),
    } = AutoDebugInfo::from_derive_input(&input).unwrap()
    else {
        panic!("AutoDebug can only be derived for structs");
    };

    println!("all fields {:#?}", fields);

    let formats = fields.iter().filter(|f| !f.skip).map(|f| {
        let ident = f.ident.clone().unwrap();
        // let ty = &f.ty;
        quote! {
            .field(stringify!(#ident), &self.#ident)
        }
    });

    quote::quote! {
        impl #generics std::fmt::Debug for #ident #generics {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!(#ident))
                #(#formats)*
                .finish()
            }
        }
    }
}
