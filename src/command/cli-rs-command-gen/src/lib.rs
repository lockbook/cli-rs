use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;

const MAX_COMMANDS: u16 = 6;

#[proc_macro]
pub fn command(input: TokenStream) -> TokenStream {
    let n: u16 = input.to_string().parse().unwrap();
    if n > MAX_COMMANDS {
        panic!("Max command size is {MAX_COMMANDS}");
    }

    let command_name = Ident::new(&format!("Command{n}"), Span::call_site());
    let callback_name = Ident::new(&format!("Callback{n}"), Span::call_site());

    // T1, T2, ... TN
    let generics: &Vec<Ident> = &(1..=n)
        .into_iter()
        .map(|num| Ident::new(&format!("T{num}"), Span::call_site()))
        .collect();

    let ins: &Vec<Ident> = &(1..=n)
        .into_iter()
        .map(|num| Ident::new(&format!("in{num}"), Span::call_site()))
        .collect();

    // T1: Input, T2: Input, ... TN: Input
    let generic_with_types = quote!(#(#generics: Input),*);

    let where_generic_with_types = if n >= 1 {
        quote! {
            where
                #generic_with_types
        }
    } else {
        quote!()
    };

    // pub in1: T1,\npub in2: T2, ...
    let struct_definition = quote! {
        #(pub #ins: #generics,)*
    };

    let symbol_vec = quote! {
        vec![#(&mut self.#ins),*]
    };

    let handler_call = quote! {
        handler(#(&self.#ins),*)
    };

    let input_fn = if n == MAX_COMMANDS {
        quote! {}
    } else {
        let next_command = Ident::new(&format!("Command{}", n + 1), Span::call_site());
        let next_generic = Ident::new(&format!("T{}", n + 1), Span::call_site());
        let next_in = Ident::new(&format!("in{}", n + 1), Span::call_site());
        let ins_copy = &ins;

        let in_transfer = quote! {
            #(#ins: self.#ins_copy,)*
        };

        quote! {
            pub fn input<#next_generic: Input>(self, #next_in: #next_generic) -> #next_command<'a, #(#generics,)* #next_generic> {
                #next_command {
                    docs: self.docs,
                    handler: None,

                    #in_transfer

                    #next_in,

                    subcommands: self.subcommands,
                }
            }
        }
    };

    let command_0_fns = if n == 0 {
        quote! {
            pub fn name(name: &str) -> Self {
                Self {
                    docs: DocInfo {
                        name: name.to_string(),
                        ..Default::default()
                    },
                    subcommands: vec![],
                    handler: None,
                }
            }

            pub fn with_completions(self) -> Self {
                let name = self.docs.name.clone();

                self.subcommand(
                    Self::name("completions")
                        .description("generate completions for a given shell")
                        .input(Arg::<CompletionMode>::name("shell").completor(|prompt| {
                            Ok(["bash".to_string(), "zsh".to_string(), "fish".to_string()]
                                .into_iter()
                                .filter(|sh| sh.starts_with(prompt))
                                .collect())
                        }))
                        .handler(move |shell| {
                            shell.get().print_completion(&name);
                            Ok(())
                        }),
                )
            }

            pub fn description(mut self, description: &str) -> Self {
                self.docs.description = Some(description.to_string());
                self
            }
        }
    } else {
        quote! {}
    };

    quote! {

        type #callback_name<'a, #(#generics),* > = Box<dyn FnMut(#(&#generics),*) -> CliResult<()> + 'a>;
        pub struct #command_name<'a, #generic_with_types> {
            pub docs: DocInfo,

            pub subcommands: Vec<Box<dyn Cmd + 'a>>,
            pub handler: Option<#callback_name<'a, #( #generics),*>>,

            #struct_definition
        }

        impl<'a, #( #generics ),*> ParserInfo for #command_name<'a, #( #generics ),*>
            #where_generic_with_types
        {
            fn docs(&self) -> &DocInfo {
                &self.docs
            }

            fn symbols(&mut self) -> Vec<&mut dyn Input> {
                #symbol_vec
            }

            fn subcommand_docs(&self) -> Vec<DocInfo> {
                self.subcommands.iter().map(|s| s.docs().clone()).collect()
            }

            fn call_handler(&mut self) -> CliResult<()> {
                if let Some(handler) = &mut self.handler {
                    #handler_call
                } else {
                    Err(CliError::from(format!(
                        "No handler hooked up to {}",
                        self.docs.cmd_path()
                    )))
                }
            }

            fn push_parent(&mut self, parents: &[String]) {
                self.docs.parents.extend_from_slice(parents);
            }

            fn complete_subcommand(&mut self, sub_idx: usize, tokens: &[String]) -> Result<(), CliError> {
                self.subcommands[sub_idx].complete_args(tokens)
            }

            fn parse_subcommand(&mut self, sub_idx: usize, tokens: &[String]) -> Result<(), CliError> {
                self.subcommands[sub_idx].parse_args(tokens)
            }
        }

        impl<'a, #generic_with_types> #command_name<'a, #( #generics ),*> {

            #command_0_fns

            #input_fn

            pub fn handler<F>(mut self, handler: F) -> Self
            where
                F: FnMut(#(&#generics),*) -> CliResult<()> + 'a,
            {
                self.handler = Some(Box::new(handler));
                self
            }

            pub fn subcommand<C: Cmd + 'a>(mut self, mut sub: C) -> Self {
                sub.push_parent(&self.docs.parents);
                sub.push_parent(&[self.docs.name.clone()]);
                self.subcommands.push(Box::new(sub));
                self
            }
        }
    }.into()
}
