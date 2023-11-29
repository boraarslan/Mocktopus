use proc_macro::TokenStream;
use watt::WasmMacro;

static MACRO: WasmMacro = WasmMacro::new(WASM);
static WASM: &[u8] = include_bytes!("mocktopus_macros.wasm");

#[proc_macro_attribute]
pub fn mockable(args: TokenStream, token_stream: TokenStream) -> TokenStream {
    MACRO.proc_macro_attribute("mockable", args, token_stream)
}

#[proc_macro_attribute]
pub fn not_mockable(_: TokenStream, token_stream: TokenStream) -> TokenStream {
    token_stream
}
