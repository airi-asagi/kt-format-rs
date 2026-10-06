use proc_macro2::{TokenStream, TokenTree};
use quote::quote;
use regex::{Captures, Regex};
use std::sync::{LazyLock, Mutex};
use syn::parse::discouraged::Speculative;
use syn::parse::{Parse, ParseStream};
use syn::{LitStr, Path, Token};

#[macro_export]
macro_rules! def {
    ($(#[$attr:meta])* $callback:ident! $(in $($path:ident)::+)?) => {
        paste::paste! {
            $(#[$attr])* #[proc_macro]
            pub fn $callback(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
                ::syn::parse_macro_input!(input as $crate::FormatArgs)
                    .to_tokens(stringify!($(::$($path)::+::)?$callback))
                    .into()
            }
        }
    };
}

pub struct FormatArgs {
    pre_args: TokenStream,
    dollars: u8,
    src: LitStr,
    post_args: TokenStream,
}
impl FormatArgs {
    pub fn to_tokens(self, callback: &str) -> TokenStream {
        let Self {
            pre_args,
            src,
            post_args,
            ..
        } = self;

        let callback: Path = syn::parse_str(callback).unwrap();

        let mut inner = src.value();

        inner = inner.replace("{", "{{");
        inner = inner.replace("}", "}}");

        let inner =
            re(self.dollars).replace_all(&inner, |caps: &Captures| format!("{{{}}}", &caps[1]));

        let src = LitStr::new(&inner, src.span());

        quote!(#callback!(#pre_args #src #post_args))
    }
}
impl Parse for FormatArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork();
        let mut pre_args = TokenStream::new();

        while !fork.is_empty() {
            if !(fork.peek(Token![$]) || fork.peek(syn::LitStr))
                && let Ok(token) = fork.parse::<TokenTree>()
            {
                pre_args.extend(std::iter::once(token));
            } else {
                break;
            }
        }
        input.advance_to(&fork);

        let mut dollars = 0;
        while input.peek(Token![$]) {
            dollars += 1;
            input.parse::<Token![$]>()?;
        }

        Ok(Self {
            pre_args,
            dollars: std::cmp::max(1, dollars),
            src: input.parse()?,
            post_args: input.parse()?,
        })
    }
}

static REGEX_CACHE: LazyLock<Mutex<[Option<Regex>; 256]>> =
    LazyLock::new(|| Mutex::new(std::array::from_fn(|_| None)));
fn re(n: u8) -> Regex {
    let mut cache = REGEX_CACHE.lock().unwrap();
    if let Some(re) = &cache[n as usize] {
        re.clone()
    } else {
        let re = Regex::new(&format!(r"\${{{}}}\{{\{{([^{{}}]*?)\}}\}}", n)).unwrap();
        cache[n as usize] = Some(re.clone());
        re
    }
}
