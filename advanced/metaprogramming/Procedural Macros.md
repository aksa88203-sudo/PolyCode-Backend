# Procedural Macros in Rust

## Overview

Procedural macros in Rust provide powerful metaprogramming capabilities that go beyond declarative macros. This guide covers procedural macro creation, TokenStream manipulation, and advanced macro techniques.

---

## Procedural Macro Fundamentals

### Basic Procedural Macro Structure

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro]
pub fn say_hello(input: TokenStream) -> TokenStream {
    // Parse the input tokens
    let input_ast = parse_macro_input!(input as ItemFn);
    
    // Generate the output tokens
    let expanded = quote! {
        fn hello_world() {
            println!("Hello, world!");
        }
        
        #input_ast
        
        hello_world();
    };
    
    expanded.into()
}
```

### Macro Attributes

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, LitStr};

#[proc_macro_attribute]
pub fn trace_function(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attrs as AttributeArgs);
    let input_fn = parse_macro_input!(item as ItemFn);
    
    // Extract the trace level from attributes
    let level = args.iter().find_map(|arg| {
        if let syn::Meta::NameValue(syn::MetaNameValue {
            path: syn::PathSegment { ident, .. },
            lit: syn::Lit::Str(lit_str),
            ..
        }) = arg {
            if ident == "level" {
                Some(lit_str.value())
            } else {
                None
            }
        } else {
            None
        }
    }).unwrap_or("info".to_string());
    
    let fn_name = &input_fn.sig.ident;
    let fn_body = &input_fn.block;
    
    let expanded = quote! {
        fn #fn_name() #fn_body {
            println!("[{}] Entering function: {}", #level, stringify!(#fn_name));
            
            let start = std::time::Instant::now();
            
            {
                let result = {
                    fn #fn_name() #fn_body
                    result
                };
                
                let duration = start.elapsed();
                println!("[{}] Exiting function: {} (took {:?})", #level, stringify!(#fn_name), duration);
                result
            }
        }
    };
    
    expanded.into()
}
```

---

## TokenStream Manipulation

### Custom Token Parsing

```rust
use proc_macro::TokenStream;
use syn::{parse_macro_input, Expr, LitStr};
use quote::quote;

#[proc_macro]
pub fn json_object(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as Expr);
    
    let json_value = match parsed {
        Expr::Struct(expr_struct) => {
            let struct_name = &expr_struct.path.segments.last().unwrap().ident;
            let fields: Vec<_> = expr_struct.fields.iter().map(|field| {
                let field_name = &field.member;
                let field_value = &field.expr;
                quote!(#field_name: #field_value)
            }).collect();
            
            quote!({
                let mut map = std::collections::HashMap::new();
                #(map.insert(stringify!(#fields.0), #fields.1);)*
                map
            })
        },
        _ => {
            quote!(compile_error!("Expected struct expression"))
        }
    };
    
    json_value.into()
}
```

### Code Generation Patterns

```rust
#[proc_macro]
pub fn create_struct(input: TokenStream) -> TokenStream {
    let struct_definition = parse_macro_input!(input as syn::ItemStruct);
    let struct_name = &struct_definition.ident;
    let fields = &struct_definition.fields;
    
    // Generate getter methods
    let getters: Vec<_> = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        let getter_name = quote::format_ident!("get_{}", field_name);
        
        quote! {
            pub fn #getter_name(&self) -> &#field_type {
                &self.#field_name
            }
        }
    }).collect();
    
    // Generate setter methods
    let setters: Vec<_> = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        let setter_name = quote::format_ident!("set_{}", field_name);
        
        quote! {
            pub fn #setter_name(&mut self, value: #field_type) {
                self.#field_name = value;
            }
        }
    }).collect();
    
    // Generate constructor
    let constructor_fields: Vec<_> = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote!(#field_name: #field_name)
    }).collect();
    
    let expanded = quote! {
        pub struct #struct_name {
            #fields
        }
        
        impl #struct_name {
            pub fn new(#constructor_fields) -> Self {
                Self { #fields }
            }
            
            #(#getters)*
            
            #(#setters)*
        }
    };
    
    expanded.into()
}
```

---

## Advanced Macro Techniques

### Recursive Macros

```rust
#[proc_macro]
pub fn count_items(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as Expr);
    
    let count = count_expr_items(&parsed);
    
    quote! {
        const COUNT: usize = #count;
        println!("Total items: {}", COUNT);
    }.into()
}

fn count_expr_items(expr: &syn::Expr) -> usize {
    match expr {
        syn::Expr::Tuple(expr_tuple) => expr_tuple.elems.len(),
        syn::Expr::Array(expr_array) => expr_array.elems.len(),
        syn::Expr::Struct(expr_struct) => expr_struct.fields.len(),
        syn::Expr::Block(expr_block) => {
            expr_block.stmts.iter().map(|stmt| {
                if let syn::Stmt::Expr(stmt_expr) = stmt {
                    count_expr_items(&stmt_expr.expr)
                } else {
                    0
                }
            }).sum()
        },
        _ => 1,
    }
}
```

### Conditional Compilation Macros

```rust
#[proc_macro]
pub fn debug_print(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as syn::Expr);
    
    let expanded = if cfg!(debug_assertions) {
        quote! {
            println!("[DEBUG] {}", #parsed);
        }
    } else {
        quote! {
            // Debug printing disabled in release mode
        }
    };
    
    expanded.into()
}
```

### Macro Validation

```rust
use syn::{parse_macro_input, DeriveInput, Data, Fields, Variant};
use quote::quote;

#[proc_macro_derive(Validate)]
pub fn validate_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    
    let validation_impl = match &input.data {
        Data::Struct(data_struct) => {
            generate_struct_validation(&input.ident, &data_struct.fields)
        },
        Data::Enum(data_enum) => {
            generate_enum_validation(&input.ident, &data_enum.variants)
        },
        Data::Union(_) => {
            quote!(compile_error!("Union validation not supported"))
        },
    };
    
    let expanded = quote! {
        impl #struct_name {
            #validation_impl
        }
    };
    
    expanded.into()
}

fn generate_struct_validation(struct_name: &syn::Ident, fields: &syn::Fields) -> proc_macro2::TokenStream {
    let validation_methods: Vec<_> = fields.iter().enumerate().map(|(i, field)| {
        let field_name = field.ident.as_ref().unwrap();
        let method_name = quote::format_ident!("validate_field_{}", i);
        
        match &field.ty {
            syn::Type::Path(type_path) if type_path.path.is_ident("String") => {
                quote! {
                    fn #method_name(&self) -> Result<(), String> {
                        if self.#field_name.is_empty() {
                            Err(format!("Field '{}' cannot be empty", stringify!(#field_name)))
                        } else {
                            Ok(())
                        }
                    }
                }
            },
            syn::Type::Path(type_path) if type_path.path.is_ident("u32") => {
                quote! {
                    fn #method_name(&self) -> Result<(), String> {
                        if self.#field_name == 0 {
                            Err(format!("Field '{}' cannot be zero", stringify!(#field_name)))
                        } else {
                            Ok(())
                        }
                    }
                }
            },
            _ => {
                quote! {
                    fn #method_name(&self) -> Result<(), String> {
                        Ok(()) // No validation for this field type
                    }
                }
            },
        }
    }).collect();
    
    let validate_all: Vec<_> = fields.iter().enumerate().map(|(i, field)| {
        let method_name = quote::format_ident!("validate_field_{}", i);
        quote! {
            self.#method_name()?;
        }
    }).collect();
    
    quote! {
        pub fn validate(&self) -> Result<(), String> {
            #(#validate_all)*
            Ok(())
        }
        
        #(#validation_methods)*
    }
}
```

---

## Macro Testing and Debugging

### Macro Testing Utilities

```rust
#[proc_macro]
pub fn assert_expand(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as syn::Expr);
    
    let expanded = quote! {
        {
            let expected = #parsed;
            let actual = macro_expand!(#parsed);
            
            assert_eq!(actual.to_string(), expected.to_string(), 
                       "Macro expansion did not match expected result");
        }
    };
    
    expanded.into()
}
```

### Debug Macros

```rust
#[proc_macro]
pub fn debug_macro(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as syn::Expr);
    
    let debug_output = quote! {
        eprintln!("[MACRO_DEBUG] Input tokens: {}", stringify!(#parsed));
        eprintln!("[MACRO_DEBUG] Parsed AST: {:?}", #parsed);
        
        let expanded = #parsed;
        eprintln!("[MACRO_DEBUG] Expanded: {}", stringify!(#expanded));
        
        expanded
    };
    
    debug_output.into()
}
```

---

## Key Takeaways

- **Procedural macros** provide powerful code generation capabilities
- **TokenStream** manipulation allows fine-grained control
- **quote!** macro simplifies token generation
- **syn** crate provides robust AST parsing
- **Recursive macros** can handle complex patterns
- **Derive macros** automate trait implementations
- **Testing macros** help validate macro behavior

---

## Procedural Macro Best Practices

| Practice | Description | Implementation |
|----------|-------------|----------------|
| **Error handling** | Provide clear error messages | Use compile_error! |
| **Documentation** | Document macro behavior | Include doc comments |
| **Testing** | Test macro expansions | Use assert_expand! |
| **Validation** | Validate input tokens | Use syn for parsing |
| **Performance** | Optimize macro expansion | Avoid unnecessary allocations |
| **Modularity** | Keep macros focused | Split complex macros |
| **Debugging** | Provide debug output | Use debug macros |
