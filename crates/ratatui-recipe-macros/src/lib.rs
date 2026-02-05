use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{Data, DataEnum, DeriveInput, Type, parse_macro_input};

#[proc_macro_derive(Pages)]
pub fn page(input: proc_macro::TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match pages_derive(&input) {
        Ok(tokens) => tokens,
        Err(tokens) => tokens,
    }
}

fn pages_derive(input: &DeriveInput) -> Result<TokenStream, TokenStream> {
    let r#enum = get_enum(input)?;
    let variants = get_pages_variants(r#enum)?;

    let page_id_tokens = generate_page_id(&variants);
    let page_state_impl = generate_page_state_impl(&input.ident, &variants);

    Ok(quote! {
        #page_id_tokens

        #page_state_impl
    }
    .into())
}

fn get_enum(input: &DeriveInput) -> Result<&syn::DataEnum, proc_macro::TokenStream> {
    match &input.data {
        Data::Enum(data_enum) => Ok(data_enum),
        _ => Err(quote! {
            compile_error!("#[derive(ratatui_recipe::pages)] can only be used on enums. Check out the ratapp documentation for more information.");
        }
        .into()),
    }
}

fn get_pages_variants(input: &DataEnum) -> Result<Vec<(&Ident, &Type)>, proc_macro::TokenStream> {
    let mut result = Vec::new();

    for variant in &input.variants {
        let name = &variant.ident;
        let ty = match &variant.fields {
            syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => &fields.unnamed[0].ty,
            _ => {
                return Err(quote! {
                    compile_error!("#[derive(ratatui_recipe::pages)] can only be used on enums with single unnamed field variants (i.e. `Variant(YourpageType)`). Check out the ratapp documentation for more information.");
                }.into());
            }
        };
        result.push((name, ty));
    }

    Ok(result)
}

// TODO: Base `pub` on app's `page` enum visibility.
fn generate_page_id(variants: &[(&Ident, &Type)]) -> proc_macro2::TokenStream {
    let ids = variants.iter().map(|(name, _)| name);

    quote! {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub enum pageID {
            #(#ids),*
        }
    }
}

fn generate_page_state_impl(
    enum_name: &Ident,
    variants: &[(&Ident, &Type)],
) -> proc_macro2::TokenStream {
    let where_bounds = variants.iter().map(|(_, ty)| {
        quote! {
            #ty : ratatui_recipe::StatefulPage<pageID, S>
        }
    });

    let match_new = variants.iter().map(|(name, ty)| {
        quote! {
            pageID::#name => #enum_name::#name(#ty::default()),
        }
    });

    let match_draw = variants.iter().map(|(name, _)| {
        quote! {
            #enum_name::#name(page) => StatefulPage::draw(page, frame, state),
        }
    });

    let match_on_event = variants.iter().map(|(name, _)| {
        quote! {
            #enum_name::#name(page) => StatefulPage::on_event(page, event, router, state).await,
        }
    });

    let match_on_enter = variants.iter().map(|(name, _)| {
        quote! {
            #enum_name::#name(page) => StatefulPage::on_enter(page, router, state).await,
        }
    });

    let match_on_exit = variants.iter().map(|(name, _)| {
        quote! {
            #enum_name::#name(page) => StatefulPage::on_exit(page, router, state).await,
        }
    });

    let match_on_pause = variants.iter().map(|(name, _)| {
        quote! {
            #enum_name::#name(page) => StatefulPage::on_pause(page, router, state).await,
        }
    });

    let match_on_resume = variants.iter().map(|(name, _)| {
        quote! {
            #enum_name::#name(page) => StatefulPage::on_resume(page, router, state).await,
        }
    });

    let match_task = variants.iter().map(|(name, _)| {
        quote! {
            #enum_name::#name(page) => StatefulPage::task(page, router, state).await,
        }
    });

    let page_state_impl = quote! {
        impl<S> ratatui_recipe::PageState<S> for #enum_name
        where
            #( #where_bounds, )*
        {
            type ID = pageID;

            fn new(id: Self::ID) -> Self {
                match id {
                    #(#match_new)*
                }
            }

            fn draw(&mut self, frame: &mut ratatui::Frame, state: &S) {
                use ratatui_recipe::StatefulPage;

                match self {
                    #(#match_draw)*
                }
            }

            async fn on_event(&mut self, event: ratatui::crossterm::event::Event, router: ratatui_recipe::Router<Self::ID>, state: &mut S) {
                use ratatui_recipe::StatefulPage;

                match self {
                    #(#match_on_event)*
                }
            }

            async fn on_enter(&mut self, router: ratatui_recipe::Router<Self::ID>, state: &mut S) {
                use ratatui_recipe::StatefulPage;

                match self {
                    #(#match_on_enter)*
                }
            }

            async fn on_exit(&mut self, router: ratatui_recipe::Router<Self::ID>, state: &mut S) {
                use ratatui_recipe::StatefulPage;

                match self {
                    #(#match_on_exit)*
                }
            }

            async fn on_pause(&mut self, router: ratatui_recipe::Router<Self::ID>, state: &mut S) {
                use ratatui_recipe::StatefulPage;

                match self {
                    #(#match_on_pause)*
                }
            }

            async fn on_resume(&mut self, router: ratatui_recipe::Router<Self::ID>, state: &mut S) {
                use ratatui_recipe::StatefulPage;

                match self {
                    #(#match_on_resume)*
                }
            }

            async fn task(&mut self, router: ratatui_recipe::Router<Self::ID>, state: &mut S) {
                use ratatui_recipe::StatefulPage;

                match self {
                    #(#match_task)*
                }
            }
        }
    };

    page_state_impl
}
