// components/erks/macros/src/lib.rs

use heck::ToSnakeCase;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
  Data, DeriveInput, Field, Fields, Ident, LitStr, Type, parse_macro_input,
};

#[proc_macro_derive(Error, attributes(severity, category, url, help))]
pub fn derive_error_ctors(input: TokenStream) -> TokenStream {
  let DeriveInput {
    ident: enum_name,
    data,
    ..
  } = parse_macro_input!(input as DeriveInput);

  let variants = if let Data::Enum(e) = data {
    e.variants
  } else {
    return syn::Error::new_spanned(
      enum_name,
      "ErrorDerive can only be derived on enums",
    )
    .to_compile_error()
    .into();
  };

  let mut methods: Vec<TokenStream2> = Vec::new();
  let mut sev_match_arms: Vec<TokenStream2> = Vec::new();
  let mut cat_match_arms: Vec<TokenStream2> = Vec::new();
  let mut code_match_arms: Vec<TokenStream2> = Vec::new();
  let mut url_match_arms: Vec<TokenStream2> = Vec::new();
  let mut help_match_arms: Vec<TokenStream2> = Vec::new();

  for var in &variants {
    let v_ident = &var.ident;
    let snake = format_ident!("{}", v_ident.to_string().to_snake_case());

    // Generate constructor methods
    if let Fields::Named(fields_named) = &var.fields {
      let fields = &fields_named.named;

      // Generate convenience constructor for variants with source and path
      if has_source_and_path(fields) {
        let path_type = get_path_field_type(fields);
        let convenience_ctor = quote! {
          pub fn #snake<P: Into<#path_type>>(source: std::io::Error, path: P) -> Self {
            Self::#v_ident {
              source,
              path: path.into(),
              context: String::new(),
            }
          }
        };
        methods.push(convenience_ctor);

        // Also generate a version with context
        let snake_with_context = format_ident!("{}_with_context", snake);
        let context_ctor = quote! {
          pub fn #snake_with_context<P: Into<#path_type>, C: Into<String>>(
            source: std::io::Error,
            path: P,
            context: C
          ) -> Self {
            Self::#v_ident {
              source,
              path: path.into(),
              context: context.into(),
            }
          }
        };
        methods.push(context_ctor);
      }
    }

    // --- parse #[severity(Low|Medium|High|Critical)] as an Ident ---
    let sev_attr = var
      .attrs
      .iter()
      .find(|a| a.path().is_ident("severity"))
      .unwrap_or_else(|| panic!("Variant `{v_ident}` missing #[severity(..)]"));
    let domain_sev_ident: Ident = sev_attr
      .parse_args()
      .expect("expected an identifier in #[severity(...)]");

    // Convert domain severity to miette severity
    let miette_sev = match domain_sev_ident.to_string().as_str() {
      "Low" => quote! { miette::Severity::Advice },
      "Medium" => quote! { miette::Severity::Warning },
      "High" => quote! { miette::Severity::Error },
      "Critical" => quote! { miette::Severity::Error },
      _ => panic!("Unknown severity: {domain_sev_ident}"),
    };

    sev_match_arms.push(quote! {
        #enum_name::#v_ident { .. } => Some(#miette_sev),
    });

    // TODO: Fix the implementation here for stitching in category
    // --- parse #[category(Generic|Filesystem|Data|Http)] as an Ident ---
    let cat_attr = var
      .attrs
      .iter()
      .find(|a| a.path().is_ident("category"))
      .unwrap_or_else(|| panic!("Variant `{v_ident}` missing #[category(..)]"));
    let domain_cat_ident: Ident = cat_attr
      .parse_args()
      .expect("expected an identifier in #[category(...)]");

    // Convert domain category to your Category enum
    let category_variant = match domain_cat_ident.to_string().as_str() {
      "Generic" => quote! { crate::Category::Generic },
      "Filesystem" => quote! { crate::Category::Filesystem },
      "Data" => quote! { crate::Category::Data },
      "Http" => quote! { crate::Category::Http },
      _ => panic!("Unknown category: {domain_cat_ident}"),
    };

    cat_match_arms.push(quote! {
        #enum_name::#v_ident { .. } => #category_variant,
    });

    // --- parse optional #[url("…")] ---
    if let Some(url_attr) = var.attrs.iter().find(|a| a.path().is_ident("url"))
    {
      let url: LitStr = url_attr
        .parse_args()
        .expect("expected string literal in #[url(...)]");
      url_match_arms.push(quote! {
          #enum_name::#v_ident { .. } => Some(Box::new(#url)),
      });
    }

    // --- parse optional #[help("…")] ---
    if let Some(help_attr) =
      var.attrs.iter().find(|a| a.path().is_ident("help"))
    {
      let help: LitStr = help_attr
        .parse_args()
        .expect("expected string literal in #[help(...)]");
      help_match_arms.push(quote! {
          #enum_name::#v_ident { .. } => Some(Box::new(#help)),
      });
    }

    // --- derive `code()` from the snake name ---
    let code_str = format!("erks::{snake}");
    code_match_arms.push(quote! {
        #enum_name::#v_ident { .. } => Some(Box::new(#code_str)),
    });
  }

  // now generate the `Diagnostic` impl
  let diagnostic_impl = quote! {
      impl miette::Diagnostic for #enum_name {
          fn severity(&self) -> Option<miette::Severity> {
              match self {
                  #(#sev_match_arms)*
              }
          }

          fn code<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
              match self {
                  #(#code_match_arms)*
              }
          }

          fn url<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
              match self {
                  #(#url_match_arms)*
                  _ => None,
              }
          }

          fn help<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
              match self {
                  #(#help_match_arms)*
                  _ => None,
              }
          }
      }
  };

  // combine constructors + Diagnostic impl
  let expanded = quote! {
      impl #enum_name {
          #(#methods)*
      }

      #diagnostic_impl
  };

  expanded.into()
}

// Helper function to check if variant has both source and path fields
fn has_source_and_path(
  fields: &syn::punctuated::Punctuated<Field, syn::token::Comma>,
) -> bool {
  let mut has_source = false;
  let mut has_path = false;

  for field in fields {
    if let Some(name) = &field.ident {
      if name == "source" {
        has_source = true;
      } else if name == "path" {
        has_path = true;
      }
    }
  }

  has_source && has_path
}

// Helper function to get the type of the path field
fn get_path_field_type(
  fields: &syn::punctuated::Punctuated<Field, syn::token::Comma>,
) -> &Type {
  for field in fields {
    if let Some(name) = &field.ident {
      if name == "path" {
        return &field.ty;
      }
    }
  }
  panic!("Path field not found");
}
