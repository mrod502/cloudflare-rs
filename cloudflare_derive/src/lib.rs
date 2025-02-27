extern crate proc_macro;
use std::any::Any;

use proc_macro::{Ident, TokenStream, TokenTree};

#[proc_macro_derive(DnsCommon)]
pub fn dns_common(input: TokenStream) -> TokenStream {
    let mut s = vec![];
    for tok in input.clone().into_iter() {
        match tok {
            TokenTree::Ident(v) => {
                if (s.len() > 0) {
                    s.pop();
                    s.push(v.to_string());
                    break;
                }
                if v.to_string() == "struct" {
                    s.push(v.to_string());
                }
            }
            _ => {}
        }
    }
    let struct_name = s.pop().unwrap();
    let impl_open = format!("impl DnsCommon for {}", struct_name);
    let impl_body = "{
    fn get_id(self) -> Option<String>{
        self.id
    }
    fn set_id(&mut self, v: Option<String>){
        self.id = v;
    }
    fn get_zone_id(self) -> Option<String> {self.zone_id}
    fn set_zone_id(&mut self, v: Option<String>) {
        self.zone_id = v;
    }
    fn get_zone_name(self) -> Option<String> {self.zone_name}
    fn set_zone_name(&mut self, v: Option<String>){
        self.zone_name = v;
    }
    fn get_name(self) -> Option<String> {self.name}
    fn set_name(&mut self, v: Option<String>){
        self.name = v;
    }
}
"
    .to_string();
    let implement = impl_open + &impl_body;
    implement.parse().unwrap()
}
