use proc_macro2::*;
use syn::spanned::Spanned;

pub fn error<T: Spanned>(span: &T, s: &str) -> TokenStream {
    let (start, end) = (span.span(), span.span());
    let mut v = Vec::new();
    v.push(respan(Literal::string(&format!("bin_codec: {}", s)), Span::call_site()));
    let group = v.into_iter().collect();

    let mut r = Vec::<TokenTree>::new();
    r.push(respan(Ident::new("compile_error", start), start));
    r.push(respan(Punct::new('!', Spacing::Alone), Span::call_site()));
    r.push(respan(Group::new(Delimiter::Brace, group), end));

    r.into_iter().collect()
}

fn respan<T: Into<TokenTree>>(t: T, span: Span) -> TokenTree {
    let mut t = t.into();
    t.set_span(span);
    t
}