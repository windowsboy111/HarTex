/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2023 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

use proc_macro::Diagnostic;
use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::TokenTree;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::AttrStyle;
use syn::Data;
use syn::DataEnum;
use syn::DataUnion;
use syn::DeriveInput;
use syn::Visibility;

const BOOLEAN_PARAMETERS: [&str; 1] = ["interaction_only"];
const LITERAL_PARAMETERS: [&str; 3] = ["command_type", "name", "minimum_level"];
const VALID_ATTR_PARAMETER_NAMES: [&str; 4] =
    ["command_type", "interaction_only", "name", "minimum_level"];

pub(crate) struct MetadataMacroInput;

impl Parse for MetadataMacroInput {
    fn parse(_: ParseStream) -> syn::Result<Self> {
        todo!()
    }
}

#[allow(clippy::needless_pass_by_ref_mut)]
#[allow(clippy::too_many_lines)]
#[deprecated(since = "0.3.0")]
pub fn expand_command_metadata_derivation(input: &mut DeriveInput) -> Option<TokenStream2> {
    // check if item is public
    match input.vis.clone() {
        Visibility::Public(_) => {}
        visibility => {
            visibility
                .span()
                .unwrap()
                .error("trait can only be derived on pub items")
                .emit();

            return None;
        }
    }

    // check if item is a struct
    match input.data.clone() {
        Data::Struct(_) => {}
        Data::Enum(DataEnum { enum_token, .. }) => {
            enum_token
                .span()
                .unwrap()
                .error("trait can only be derived on structs")
                .emit();

            return None;
        }
        Data::Union(DataUnion { union_token, .. }) => {
            union_token
                .span()
                .unwrap()
                .error("trait can only be derived on structs")
                .emit();

            return None;
        }
    }

    // check for any attributes following derive
    if input.attrs.is_empty() {
        Span::call_site()
            .unwrap()
            .error("expected `metadata` attributes after derive")
            .emit();

        return None;
    }

    // split attribute vector into two
    let mut wrong_paths = input.attrs.clone();
    let correct_attrs = wrong_paths
        .extract_if(|attr| attr.style == AttrStyle::Outer && attr.path().is_ident("metadata"))
        .collect::<Vec<_>>();

    #[allow(unused_must_use)]
    if !wrong_paths.is_empty() {
        wrong_paths
            .into_iter()
            .map(|attr| attr.path().span().unwrap())
            .map(|span| span.error("expected `metadata` attribute"))
            .map(Diagnostic::emit);

        return None;
    }

    let mut functions = TokenStream2::new();
    let mut previous_attr_name = String::new();
    for attr in correct_attrs {
        if attr.parse_args::<TokenStream2>().unwrap().is_empty() {
            attr.path()
                .span()
                .unwrap()
                .error("unexpected end of attribute")
                .emit();

            return None;
        }

        let mut iter = attr
            .parse_args::<TokenStream2>()
            .unwrap()
            .into_iter()
            .peekable();

        let Some(tree_first) = iter.next() else {
            attr.span()
                .unwrap()
                .error("expected parameter; found none")
                .emit();

            return None;
        };

        let TokenTree::Ident(ident) = tree_first.clone() else {
            tree_first
                .span()
                .unwrap()
                .error(format!("expected identifier; found `{tree_first}`"))
                .emit();

            return None;
        };

        if ident == previous_attr_name {
            ident
                .span()
                .unwrap()
                .error(format!("duplicate attribute: `{ident}`"))
                .emit();

            return None;
        }

        if !(VALID_ATTR_PARAMETER_NAMES.contains(&ident.to_string().as_str())) {
            ident
                .span()
                .unwrap()
                .error(format!("unexpected parameter name: `{ident}`"))
                .emit();

            return None;
        }

        let Some(tree_next) = iter.next() else {
            tree_first
                .span()
                .unwrap()
                .error("unexpected end of parameter")
                .emit();

            return None;
        };

        let TokenTree::Punct(punct) = tree_next.clone() else {
            tree_next
                .span()
                .unwrap()
                .error(format!("expected punctuation; found `{tree_next}` instead"))
                .emit();

            return None;
        };

        if punct.as_char() != '=' {
            punct
                .span()
                .unwrap()
                .error(format!("expected `=`; found `{punct}` instead"))
                .emit();

            return None;
        }

        let Some(tree_next) = iter.next() else {
            tree_next
                .span()
                .unwrap()
                .error("unexpected end of parameter")
                .emit();

            return None;
        };

        if LITERAL_PARAMETERS.contains(&ident.to_string().as_str()) {
            let TokenTree::Literal(literal) = tree_next.clone() else {
                tree_next
                    .span()
                    .unwrap()
                    .error(format!("expected literal; found `{tree_next}`"))
                    .emit();

                return None;
            };

            match ident.to_string().as_str() {
                "command_type" => {
                    let Ok(command_type) = literal.to_string().parse::<u8>() else {
                        literal
                            .span()
                            .unwrap()
                            .error(format!(
                                "expected integer literal; found literal `{literal}`"
                            ))
                            .emit();

                        return None;
                    };

                    if !(1..=3).contains(&command_type) {
                        literal
                            .span()
                            .unwrap()
                            .error(format!("invalid command type: `{literal}`"))
                            .emit();

                        return None;
                    }

                    let expanded = quote::quote! {
                        fn command_type(&self) -> u8 {
                            #tree_next
                        }
                    };
                    functions.extend(expanded);
                }
                "name" => {
                    let expanded = quote::quote! {
                        fn name(&self) -> String {
                            String::from(#tree_next)
                        }
                    };
                    functions.extend(expanded);
                }
                "minimum_level" => {
                    let expanded = quote::quote! {
                        fn minimum_level(&self) -> u16 {
                            #tree_next
                        }
                    };
                    functions.extend(expanded);
                }
                _ => unreachable!(),
            }
        } else if BOOLEAN_PARAMETERS.contains(&ident.to_string().as_str()) {
            let TokenTree::Ident(ident_bool) = tree_next.clone() else {
                tree_next
                    .span()
                    .unwrap()
                    .error(format!("expected identifier; found `{tree_next}`"))
                    .emit();

                return None;
            };

            match ident.to_string().as_str() {
                "interaction_only" => {
                    let Ok(_) = ident_bool.to_string().parse::<bool>() else {
                        ident_bool
                            .span()
                            .unwrap()
                            .error(format!("expected boolean; found `{ident_bool}`"))
                            .emit();

                        return None;
                    };

                    let expanded = quote::quote! {
                        fn interaction_only(&self) -> bool {
                            #tree_next
                        }
                    };
                    functions.extend(expanded);
                }
                _ => unreachable!(),
            }
        } else {
            unreachable!()
        };

        if let Some(extra) = iter.next() {
            extra
                .span()
                .unwrap()
                .error(format!("unexpected token: `{extra}`"))
                .emit();

            return None;
        }

        previous_attr_name = ident.to_string();
    }

    let core_use = quote::quote! {
        extern crate hartex_discord_commands_core as _commands_core;
    };
    let ident = input.ident.clone();
    let expanded = quote::quote! {
        #core_use

        #[automatically_derived]
        impl _commands_core::traits::CommandMetadata for #ident {
            #functions
        }
    };

    Some(quote::quote! {
       #expanded
    })
}

pub fn implement_metadata(_: &mut MetadataMacroInput) -> Option<TokenStream2> {
    None
}
