

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(EnumRotor)]
pub fn derive_rotor(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let enum_ident = &input.ident;

    let rotor_engine_struct = quote! {
        #[derive(Debug, Clone)]
        pub struct RotorEngine {
            blocks: Vec<#enum_ident>,
            trigger: Option<usize>,
            final_trigger: bool,
        }
    };

    let rotor_engine_init = gen_rotor_engine_init(&input);
    let rotor_engine_rot_at = gen_rotor_engine_rot_at(&input);
    let rotor_engine_impl = quote! {
        impl RotorEngine {
            #rotor_engine_init
            #rotor_engine_rot_at
        }
    };

    let iterator_for_rotor_engine = gen_iterator_for_rotator_engine(&input);

    quote! {
        #rotor_engine_struct
        #rotor_engine_impl
        #iterator_for_rotor_engine
        
    }.into()
}

fn gen_rotor_engine_init(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    
    let enum_ident = &ast.ident;
    quote! {
        /// Creates an initial state for the machine of a given size.
        /// The size is the length of the permutation.
        /// # Example
        /// ```
        /// let rotor_engine = RotorEngine::init(4);
        /// ```
        pub fn init(size: usize) -> Self {
            let mut blocks = Vec::new();
            for _i in 0..size{
                blocks.push(#enum_ident::default());
            }
            Self { blocks, trigger: None, final_trigger: false }
        }
       
    }
}

fn gen_rotor_engine_rot_at(ast:  &syn::DeriveInput) -> proc_macro2::TokenStream {
    let enum_ident = &ast.ident;

    let mut match_elems_impl = quote!{};
    let (first_elem, last_elem) = match &ast.data {
        syn::Data::Enum(syn::DataEnum {variants, ..}) => {
            let mut last_elem = &variants[0].ident;
            for i in 0..variants.len() {
                if i != variants.len() - 1 {
                    let current_elem = &variants[i+1].ident;
                    let stmt = quote! {
                        #enum_ident::#last_elem => self.blocks[index] = #enum_ident::#current_elem,
                    };
                    match_elems_impl.extend(stmt);
                    last_elem = current_elem;                   
                }

            }
            (&variants[0].ident, &variants[variants.len() - 1].ident)
            
        }
        _ => panic!("only enums supported")
    };

    quote! {
        fn rot_at(&mut self, index: usize) {
            match self.blocks[index] {
                #match_elems_impl
                #enum_ident::#last_elem => {
                    
                    if index > 0 {
                        self.trigger = Some(index - 1);
                        self.blocks[index] = #enum_ident::#first_elem;
                    } else {
                        self.final_trigger = true;
                    }
                }
            }
        }
    }
}

fn gen_iterator_for_rotator_engine(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let enum_ident = &ast.ident;
    quote! {
        impl std::iter::Iterator for RotorEngine {
            type Item = Vec<#enum_ident>;
            fn next(&mut self) -> Option<Self::Item> {

                while let Some(idx) = self.trigger {
                    self.trigger = None;
                    self.rot_at(idx);
                }         
                let capture = self.blocks.clone();
                self.rot_at(self.blocks.len() - 1);

                if self.final_trigger {
                    return None;
                }

                Some(capture)
            }
        }         
    }
    
}
