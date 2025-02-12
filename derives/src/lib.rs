use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Error, Field, Ident, TypePath};

/// Derive RequestT, IntoFuture, Builder, etc on this request params struct
///
/// You must specify API return type with ``#[conogram(result = *type*)]``
#[proc_macro_derive(Request, attributes(conogram))]
pub fn derive_request(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let params_struct_ident = &input.ident;
    let Data::Struct(params_struct) = input.data else {
        return quote_spanned! {
            input.span() => "Must be used on a request params struct"
        }
        .into();
    };

    let mut stream = TokenStream2::new();

    let params_struct_ident_str = params_struct_ident.to_string();
    let request_struct_ident = format_ident!(
        "{}Request",
        params_struct_ident_str
            .strip_suffix("Params")
            .unwrap_or(&params_struct_ident_str)
    );

    let request_struct_ident_str = request_struct_ident.to_string();
    let mut request_name = request_struct_ident_str
        .strip_suffix("Request")
        .unwrap_or(&request_struct_ident_str)
        .to_string();

    request_name.replace_range(
        ..1,
        &request_name
            .chars()
            .next()
            .unwrap()
            .to_ascii_lowercase()
            .to_string(),
    );

    let mut params_struct_doc_comment = Vec::new();
    let mut result_type = None;
    for attr in input.attrs {
        if attr.path().is_ident("conogram") {
            if let Err(err) = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("result") {
                    let type_ = meta.value()?;
                    result_type = Some(type_.parse::<TypePath>()?);
                } else {
                    return Err(Error::new(
                        meta.path.span(),
                        "Unknown attribute".to_string(),
                    ));
                }

                Ok(())
            }) {
                return err.into_compile_error().into();
            }
        }
        if attr.path().is_ident("doc") {
            params_struct_doc_comment.push(attr);
        }
    }

    let Some(result_type) = result_type else {
        return quote_spanned! {input.ident.span() => compile_error!("You must specify API return type with ``#[conogram(result = *type*)]``");}.into();
    };

    let request_struct_doc_comment = params_struct_doc_comment
        .into_iter()
        .map(|attr| attr.into_token_stream())
        .collect::<TokenStream2>();
    // Impl RequestT trait
    stream.extend(quote! {
        #[derive(Clone)]
        #request_struct_doc_comment
        pub struct #request_struct_ident<'a> {
            api: &'a crate::api::Api,
            params: #params_struct_ident,
        }

        impl crate::request::RequestT for #request_struct_ident<'_> {
            type ParamsType = #params_struct_ident;
            type ReturnType = #result_type;

            fn get_name() -> &'static str {
                #request_name
            }

            fn get_api_ref(&self) -> &crate::api::Api {
                self.api
            }

            fn get_params_ref(&self) -> &Self::ParamsType {
                &self.params
            }
        }
    });

    #[derive(Clone)]
    struct RequestField {
        _inner: Field,
        is_multipart: bool,
        name: Ident,
        optional: bool,

        method_type: TokenStream2,
        assing_impl: TokenStream2,
        doc_comment: TokenStream2,
    }

    let mut fields = Vec::new();
    for f in params_struct.fields {
        let name = f.ident.clone().unwrap();
        let type_name = f.ty.to_token_stream().to_string().replace(" ", "");
        let mut doc_comment_attrs = Vec::new();
        let mut optional = type_name.starts_with("Option<");

        for attr in &f.attrs {
            if attr.path().is_ident("doc") {
                doc_comment_attrs.push(attr.clone());
            }
            if attr.path().is_ident("serde") {
                let _ = attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("skip_serializing_if") {
                        optional = true;
                    }
                    Ok(())
                });
            }
        }

        let (method_type, assing_impl) = if type_name.starts_with("Vec<") {
            let item = format_ident!(
                "{}",
                type_name
                    .strip_prefix("Vec<")
                    .unwrap()
                    .strip_suffix(">")
                    .unwrap()
            );
            (
                quote! {
                    impl IntoIterator<Item = impl Into<#item>>
                },
                quote! {
                    #name.into_iter().map(Into::into).collect()
                },
            )
        } else if type_name.starts_with("Option<") {
            let item = format_ident!(
                "{}",
                type_name
                    .strip_prefix("Option<")
                    .unwrap()
                    .strip_suffix(">")
                    .unwrap()
            );
            (
                quote! {
                    impl Into<#item>
                },
                quote! {
                    Some(#name.into())
                },
            )
        } else {
            let type_ = &f.ty;
            (
                quote! {
                    impl Into<#type_>
                },
                quote! {
                    #name.into()
                },
            )
        };

        let multipart_starts = [
            "InputSticker",
            "InputMedia",
            "InputFile",
            "LocalFile",
            "Option<LocalFile>",
            "Option<InputFile>",
            "Vec<InputMedia>",
        ];
        // False-positives
        let multipart_blacklist = ["InputStickerFormat"];
        let is_multipart = multipart_starts.iter().any(|n| type_name.starts_with(n))
            && multipart_blacklist.iter().all(|n| !type_name.eq(n));

        fields.push(RequestField {
            _inner: f,

            name,
            optional,
            method_type,
            assing_impl,
            is_multipart,

            doc_comment: doc_comment_attrs
                .iter()
                .map(|attr| attr.to_token_stream())
                .collect(),
        });
    }

    let multipart_fields = fields.iter().filter(|f| f.is_multipart).collect::<Vec<_>>();
    let mut mutipart_field_names = multipart_fields
        .iter()
        .map(|f| f.name.clone())
        .collect::<Vec<_>>();
    mutipart_field_names.sort();

    // GetFiles impl
    if !multipart_fields.is_empty() {
        let getfiles_body = mutipart_field_names
            .iter()
            .map(|name| {
                quote! {
                    form = crate::entities::misc::input_file::GetFiles::form(&self.#name, form).await?;
                    // vec.extend(crate::entities::misc::input_file::GetFiles::get_files(&self.#name));
                }
            })
            .collect::<TokenStream2>();

        stream.extend(quote! {
            impl crate::entities::misc::input_file::GetFiles for #params_struct_ident {
                async fn form(&self, form: reqwest::multipart::Form) -> Result<reqwest::multipart::Form, std::io::Error> {
                    // let mut vec = Vec::with_capacity(6);
                    let mut form = form;
                    #getfiles_body
                    Ok(form)
                    // vec
                }
            }
        });
    }

    // TargetChatId impl for rate limit tracker
    if let Some(f) = fields.iter().find(|f| f.name.to_string() == "chat_id") {
        let type_name = f._inner.ty.to_token_stream().to_string().replace(" ", "");

        let body = match type_name.as_ref() {
            "ChatId" => quote! {
                Some(self.chat_id.clone())
            },
            "Option<ChatId>" => quote! {
                self.chat_id.clone()
            },
            "i64" => quote! {
                Some(self.chat_id.into())
            },
            "Option<i64>" => quote! {
                self.chat_id.map(Into::into)
            },
            _ => {
                let msg = format!("Unknown `chat_id` field type: `{type_name}`");
                return quote! {
                    compile_error!(#msg)
                }
                .into();
            }
        };

        stream.extend(quote! {
            impl crate::request::TargetChatId for #params_struct_ident {
                fn get_target_chat_id(&self) -> Option<crate::entities::misc::chat_id::ChatId> {
                    #body
                }
            }
        });
    } else {
        stream.extend(quote! {
            impl crate::request::TargetChatId for #params_struct_ident {
                fn get_target_chat_id(&self) -> Option<crate::entities::misc::chat_id::ChatId> {
                    None
                }
            }
        });
    }

    let (send_ident, send_ref_ident) = if multipart_fields.is_empty() {
        (format_ident!("send"), format_ident!("send_ref"))
    } else {
        (
            format_ident!("send_multipart"),
            format_ident!("send_multipart_ref"),
        )
    };

    stream.extend(quote! {
        impl<'a> std::future::IntoFuture for #request_struct_ident<'a> {
            type Output = Result<#result_type, crate::errors::ConogramError>;

            type IntoFuture =
                std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + Sync + 'a>>;

            fn into_future(self) -> Self::IntoFuture {
                Box::pin(crate::request::RequestT::#send_ident(self))
            }
        }

        impl<'a> std::future::IntoFuture for &'a #request_struct_ident<'_> {
            type Output = Result<#result_type, crate::errors::ConogramError>;

            type IntoFuture =
                std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + Sync + 'a>>;

            fn into_future(self) -> Self::IntoFuture {
                Box::pin(crate::request::RequestT::#send_ref_ident(self))
            }
        }
    });

    let required_fields = fields
        .iter()
        .filter(|&f| !f.optional)
        .cloned()
        .collect::<Vec<_>>();
    let optional_fields = fields
        .iter()
        .filter(|&f| f.optional)
        .cloned()
        .collect::<Vec<_>>();

    let constructor_method_params = required_fields
        .iter()
        .map(|f| {
            let name = &f.name;
            let method_type = &f.method_type;
            quote! {
                #name: #method_type,
            }
        })
        .collect::<TokenStream2>();

    let constructor_invoke_params = required_fields
        .iter()
        .map(|f| {
            let name = &f.name;
            let invocation = &f.assing_impl;
            quote! {
                #name: #invocation,
            }
        })
        .collect::<TokenStream2>();

    let default_ = optional_fields
        .iter()
        .map(|f| {
            let name = &f.name;
            quote! {
                #name: Default::default(),
            }
        })
        .collect::<TokenStream2>();

    let mut setter_impls = TokenStream2::new();
    for field in fields {
        let setter_doc_comment = field.doc_comment;
        let name = field.name;
        let method_type = field.method_type;
        let invocation = field.assing_impl;

        setter_impls.extend(quote! {
            #setter_doc_comment
            #[must_use]
            pub fn #name(mut self, #name: #method_type) -> Self {
                self.params.#name = #invocation;
                self
            }
        });
    }

    stream.extend(quote! {
        impl<'a> #request_struct_ident<'a> {
            pub fn new(
                api: &'a crate::api::Api,
                #constructor_method_params
            ) -> Self {
                Self {
                    api,
                    params: #params_struct_ident {
                        #constructor_invoke_params
                        #default_
                    },
                }
            }

            #setter_impls
        }
    });

    {
        let args = required_fields
            .iter()
            .map(|f| {
                let name = &f.name;
                let method_type = &f.method_type;
                quote! {
                    #name: #method_type,
                }
            })
            .collect::<TokenStream2>();

        let args_assign = required_fields
            .iter()
            .map(|f| {
                let name = &f.name;
                quote! {
                    #name,
                }
            })
            .collect::<TokenStream2>();

        let request_struct_ident_snake = request_struct_ident_to_snake_case(&request_struct_ident);

        stream.extend(quote! {
            impl crate::api::Api {
                #request_struct_doc_comment
                pub fn #request_struct_ident_snake(
                    &self,
                    #args
                ) -> #request_struct_ident {
                    #request_struct_ident::new(self, #args_assign)
                }
            }
        });
    }

    stream.into()
}

fn request_struct_ident_to_snake_case(ident: &Ident) -> Ident {
    let str = ident.to_string();
    let str = str.strip_suffix("Request").unwrap_or(&str);

    let mut out = String::with_capacity(str.len());

    for (i, char) in str.chars().enumerate() {
        if char as u32 >= b'A'.into() && char as u32 <= b'Z'.into() {
            if i > 0 {
                out.push('_');
            }
            for lower in char.to_lowercase() {
                out.push(lower);
            }
        } else {
            out.push(char);
        }
    }

    format_ident!("{}", out)
}
