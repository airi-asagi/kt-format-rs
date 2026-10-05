use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use regex::{Captures, Regex};
use std::sync::{LazyLock, Mutex};
use syn::parse::{Parse, ParseStream};
use syn::{Expr, LitStr, Token, parse_macro_input};

macro_rules! def {
    ($(#[$attr:meta])* $callback:ident) => {
        $(#[$attr])*
        #[proc_macro]
        pub fn $callback(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            parse_macro_input!(input as FString)
                .translate().to_tokens(stringify!($callback)).into()
        }
    };
}

def! {
    /// Rust's braces escape syntaxes works the same way as Python's, and depending on the string you're trying to represent, it can significantly reduce readability.
    /// ```
    /// assert_eq!(format!("{{{{{}", 1), r"{{1");
    /// ```
    ///
    /// This crate aims to improve your code's readability by introducing a Kotlin-like escape to `format!` and several related macros.
    /// ```
    /// use kt_format::kt
    ///
    /// assert_eq!(kt::format!("{{${}", 1), r"{{1");
    /// assert_eq!(kt::format!($$"{{$${}", 1), r"{{1");
    /// assert_eq!(kt::format!($$"${${$${}", 1), r"${${1");
    /// assert_eq!(format!("${{${{{}", 1), r"${${1");
    /// ```
    ///
    /// For example, within this crate's source code, regular expressions are generated using code like the following:
    /// ```
    /// Regex::new(&std::format!(r"\${{{}}}\{{\{{([^{{}}]*?)\}}\}}", n)).unwrap()
    /// ```
    /// Although this is actually impossible due to proc_macro constraints, if I could use this crate here, I could write it as follows.
    /// ```
    /// Regex::new(&kt::format!($$r"\${$${}}\{\{([^{}]*?)\}\}", 3)).unwrap()
    /// ```
    format
}
def! { print }
def! { println }
def! { eprintln }

struct FString {
    dollars: u8,
    src: LitStr,
    args: Vec<(Token![,], Option<(Ident, Token![=])>, Expr)>,
}
impl FString {
    fn translate(mut self) -> Self {
        let mut inner = self.src.value();

        inner = inner.replace("{", "{{");
        inner = inner.replace("}", "}}");

        let inner = re(self.dollars)
            .replace_all(&inner, |caps: &Captures| std::format!("{{{}}}", &caps[1]));

        self.src = LitStr::new(&inner, self.src.span());
        self
    }

    fn to_tokens(self, callback: &str) -> TokenStream {
        let Self { src, args, .. } = self;
        let callback = Ident::new(callback, Span::call_site());
        let args: Vec<_> = args
            .into_iter()
            .map(|(comma, id_eq, expr)| {
                let id_eq = id_eq.map(|(ident, eq)| quote!(#ident #eq));
                quote!(#comma #id_eq #expr)
            })
            .collect();
        quote!(#callback!(#src #(#args)*))
    }
}
impl Parse for FString {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut dollars = 0;
        while input.peek(Token![$]) {
            dollars += 1;
            input.parse::<Token![$]>()?;
        }
        let dollars = std::cmp::max(1, dollars);

        let src = input.parse::<LitStr>()?;

        let mut args = Vec::new();
        while input.peek(Token![,]) {
            let c = input.parse::<Token![,]>()?;
            let id_eq = if input.peek2(Token![=]) {
                Some((input.parse()?, input.parse()?))
            } else {
                None
            };
            match input.parse::<Expr>().ok() {
                None => break,
                Some(expr) => args.push((c, id_eq, expr)),
            }
        }

        input.parse::<syn::parse::Nothing>()?;

        Ok(Self { dollars, src, args })
    }
}

static REGEX_CACHE: LazyLock<Mutex<[Option<Regex>; 256]>> =
    LazyLock::new(|| Mutex::new(std::array::from_fn(|_| None)));
fn re(n: u8) -> Regex {
    let mut cache = REGEX_CACHE.lock().unwrap();
    if let Some(re) = &cache[n as usize] {
        re.clone()
    } else {
        let re = Regex::new(&std::format!(r"\${{{}}}\{{\{{([^{{}}]*?)\}}\}}", n)).unwrap();
        cache[n as usize] = Some(re.clone());
        re
    }
}
