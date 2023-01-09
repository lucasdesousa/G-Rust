extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use quote::Tokens;
use syn::{Body, Field, Ident, Ty, VariantData};

#[proc_macro_derive(PacketVariable)]
pub fn packet_variable_derive(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_packet_variable(&ast);
    gen.parse().unwrap()
}

fn impl_packet_variable(ast: &syn::DeriveInput) -> Tokens {
    let name = &ast.ident;

    match &ast.body {
        Body::Struct(VariantData::Struct(fields)) => {
            impl_struct_derive(name, fields)
        },
        Body::Enum(data) => {
            todo!()
        }
        _ => {
            panic!("Packet Variable arrive not supported for this type");
        }
    }
}

fn impl_struct_derive(name: &Ident, fields: &Vec<Field>) -> Tokens {
    let from_idents = fields.iter().map(| f | &f.ident);
    let to_idents = from_idents.clone();
    let types = fields.iter().map(| f | &f.ty);
    println!("{types:?}");
    let types_clone = types.clone();
    quote! {
        impl PacketVariable for #name {
            fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
                let mut packet = HPacket::from_header_id_and_bytes(0, bytes);
                (
                    Self {
                        #(
                            #from_idents: packet.read()
                        ),*
                    },
                    packet.read_index - 6
                )
            }

            fn to_packet(&self) -> Vec<u8> {
                let mut packet = HPacket::from_header_id(0);
                #(
                    packet.append(self.#to_idents.clone());
                )*
                packet.get_bytes()[6..].to_vec()
            }

            fn can_read(bytes: Vec<u8>) -> bool {
                Self::read_size(bytes) != 0
            }

            // TODO fix read_size
            fn read_size(bytes: Vec<u8>) -> usize {
                let mut size = 0;
                #(
                    {
                        println!("#types");
                        size += 1;
                    }
                )*
                1
            }
        }
    }
}