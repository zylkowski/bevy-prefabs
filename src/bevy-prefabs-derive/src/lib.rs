extern crate proc_macro;
use proc_macro::TokenStream;
extern crate bevy;

#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

#[proc_macro_derive(Prefab)]
pub fn prefab_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    
    let name = input.ident;
    let name_string = name.to_string();
    let expanded = quote! {

        #[typetag::serde(name = #name_string)]
        impl Prefab for #name where #name: Clone {
            fn add_to_entity(&self,entity_commands: &mut bevy::ecs::system::EntityCommands){
                entity_commands.insert(self.clone());
            }
        }
    };

    expanded.into()
}
