use darling::{ast::Data, FromDeriveInput, FromField};
use quote::quote;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(deref))]
struct AutoDerefInfo {
    ident: syn::Ident,
    generics: syn::Generics,
    data: Data<(), AutoDerefFieldsInfo>,
    // meta: AutoDerefMeta,
    mutable: bool,
    field: Option<syn::Ident>,
}

// #[derive(Debug, Default, FromMeta)]
// #[darling(default)]
// struct AutoDerefMeta {
//     mutable: bool,
//     #[darling(default)]
//     field: Option<syn::Ident>,
// }

#[derive(Debug, FromField)]
// #[darling(attributes(deref))]
struct AutoDerefFieldsInfo {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    // attrs: Vec<syn::Attribute>,
}

pub(crate) fn process_auto_deref(input: syn::DeriveInput) -> proc_macro2::TokenStream {
    let AutoDerefInfo {
        ident,
        generics,
        data: Data::Struct(fields),
        mutable,
        field,
    } = AutoDerefInfo::from_derive_input(&input).unwrap()
    else {
        panic!("AutoDeref can only be derived for structs");
    };

    let field = match field {
        Some(field) => field,
        None => {
            // if no field is specified, use the first field
            fields
                .iter()
                .next()
                .expect("struct should have at least one field")
                .ident
                .clone()
                .expect("field should have an identifier")
        }
    };

    // 遍历fields获取与filed相同的field的类型
    let field_ty = fields
        .iter()
        .find(|f| f.ident == Some(field.clone()))
        .expect("field should exist")
        .ty
        .clone();

    // println!("meta field: {:#?}", field);
    println!("field type: {:#?}", field_ty);

    let mut qs = vec![quote! {
        impl #generics std::ops::Deref for #ident #generics {
            type Target = #field_ty;

            fn deref(&self) -> &Self::Target {
                &self.#field
            }
        }
    }];

    if mutable {
        qs.push(quote! {
            impl #generics std::ops::DerefMut for #ident #generics {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.#field
                }
            }
        });
    }

    quote! {#(#qs)*}
}

#[allow(unused)]
pub(crate) fn process_auto_debug(input: syn::DeriveInput) -> proc_macro2::TokenStream {
    quote::quote! {}
}
