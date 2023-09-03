use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[derive(deluxe::ParseMetaItem)]
struct ClassData(String, String);

pub fn class(attr: TokenStream, item: TokenStream) -> TokenStream {
    // parse
    let structure = parse_macro_input!(item as DeriveInput);
    let ClassData(namespace, class) = match deluxe::parse(attr) {
        Ok(info) => info,
        Err(err) => return err.to_compile_error().into(),
    };

    // prepare tokens
    let ident = &structure.ident;
    let (impl_generics, type_generics, where_clause) = structure.generics.split_for_impl();

    let ctx = super::utils::context();

    quote!(
        #structure

        impl #impl_generics #ctx::Il2CppClassData for #ident #type_generics #where_clause {
            const NAMESPACE: &'static str = #namespace;
            const CLASS: &'static str = #class;

            fn get_class<'a>() -> &'a #ctx::Il2CppClass {
                static CLASS_TYPE: #ctx::LazyLock<&'static mut #ctx::Il2CppClass> = #ctx::LazyLock::new(|| {
                    #ctx::Il2CppClass::from_name(#namespace, #class)
                        .expect(&format!("Failed to find class {}.{}", #namespace, #class))
                });
                &CLASS_TYPE
            }

            fn get_class_mut<'a>() -> &'a mut #ctx::Il2CppClass {
                Self::get_class().clone()
            }
        }
    )
    .into()
}
