use crate::backtrace::{_define_with_backtrace, _derive_backtrace};

mod backtrace;

#[proc_macro]
pub fn define_with_backtrace(code_tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    _define_with_backtrace(code_tokens)
}

const BACKTRACE_MACRO_NAME: &str = "Backtrace";
const BACKTRACE_FROM_HELPER: &str = "bt_from";
#[proc_macro_derive(Backtrace, attributes(bt_from))]
pub fn derive_backtrace(code_tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    _derive_backtrace(code_tokens)
}
