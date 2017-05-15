mod error {



















































    // TODO(emilio, #453): Figure out what to do when this happens
    // legitimately, we could track the opaque stuff and disable the
    // assertion there I guess.









    // Don't bother creating an empty module.








    // Account the trailing zero.
    //
    // TODO: Here we ignore the type we just made up, probably
    // we should refactor how the variable type and ty id work.







    // These items don't need code generation, they only need to be
    // converted to rust types in fields, arguments, and such.

    // Try to catch the common pattern:
    //
    // typedef struct foo { ... } foo;
    //
    // here.
    //

    // If this is a known named type, disallow generating anything
    // for it too.

    // Its possible that we have better layout information than
    // the inner type does, so fall back to an opaque blob based
    // on our layout if converting the inner item fails.

    // FIXME(emilio): This is a workaround to avoid generating
    // incorrect type aliases because of types that we haven't
    // been able to resolve (because, eg, they depend on a
    // template parameter).
    //
    // It's kind of a shame not generating them even when they
    // could be referenced, but we already do the same for items
    // with invalid template parameters, and at least this way
    // they can be replaced, instead of generating plain invalid
    // code.



    // We prefer using `pub use` over `pub type` because of:
    // https://github.com/rust-lang/rust/issues/26264





    // For now, generate an empty struct, later we should generate function
    // pointers and whatnot.







    // NOTE: What follows is reverse-engineered from LLVM's
    // lib/AST/RecordLayoutBuilder.cpp
    //
    // FIXME(emilio): There are some differences between Microsoft and the
    // Itanium ABI, but we'll ignore those and stick to Itanium for now.
    //
    // Also, we need to handle packed bitfields and stuff.
    // TODO(emilio): Take into account C++'s wide bitfields, and
    // packing, sigh.

    // (name, mask, width, bitfield's type, bitfield's layout)


    // We've finished a physical field, so flush it and its bitfields.

    // TODO(emilio): dedup this.

    // Now reset the size and the rest of stuff.
    // unfilled_bits_in_last_unit = 0;





    // NB: The width here is completely, absolutely intentional.


    // Flush the last physical field and its bitfields.
























    // Although uses of instantiations don't need code generation, and are
    // just converted to rust types in fields, vars, etc, we take this
    // opportunity to generate tests for their layout here.










    // Don't output classes with template parameters that aren't types, and
    // also don't output template specializations, neither total or partial.


    // generate tuple struct if struct or union is a forward declaration,
    // skip for now if template parameters are needed.




    // FIXME: This requires extra logic if you have a big array in a
    // templated struct. The reason for this is that the magic:
    //     fn clone(&self) -> Self { *self }
    // doesn't work for templates.
    //
    // It's not hard to fix though.



    // Generate the vtable from the method list if appropriate.
    //
    // TODO: I don't know how this could play with virtual methods that are
    // not in the list of methods found by us, we'll see. Also, could the
    // order of the vtable pointers vary?
    //
    // FIXME: Once we generate proper vtables, we need to codegen the
    // vtable, but *not* generate a field for it in the case that
    // needs_explicit_vtable is false but has_vtable is true.
    //
    // Also, we need to generate the vtable in such a way it "inherits" from
    // the parent too.





    // Virtual bases are already taken into account by the vtable
    // pointer.
    //
    // FIXME(emilio): Is this always right?

    // NB: We won't include unsized types in our base chain because they
    // would contribute to our size given the dummy field we insert for
    // unsized types.








    // Try to catch a bitfield contination early.

    // Flush the current bitfield.




    // NB: In unstable rust we use proper `union` types.











    // TODO: Factor the following code out, please!






    // Flush the last bitfield if any.
    //
    // FIXME: Reduce duplication with the loop above.
    // FIXME: May need to pass current_bitfield_layout too.




    // Yeah, sorry about that.



    // C++ requires every struct to be addressable, so what C++ compilers do
    // is making the struct 1-byte sized.
    //
    // This is apparently not the case for C, see:
    // https://github.com/servo/rust-bindgen/issues/551
    //
    // Just get the layout, and assume C++ if not.
    //
    // NOTE: This check is conveniently here to avoid the dummy fields we
    // may add for unused template parameters.





    // Generate the inner types and all that stuff.
    //
    // TODO: In the future we might want to be smart, and use nested
    // modules, and whatnot.
    // assert_eq!(child_item.parent_id(), item.id());

    // NOTE: Some unexposed attributes (like alignment attributes) may
    // affect layout, so we're bad and pray to the gods for avoid sending
    // all the tests to shit when parsing things like max_align_t.



    // FIXME when [RFC 1358](https://github.com/rust-lang/rust/issues/33626) ready

    // FIXME when [issue #465](https://github.com/servo/rust-bindgen/issues/465) ready








    /* const */



    // NB: We can't use to_rust_ty here since for opaque types this tries to
    // use the specialization knowledge to generate a blob field.











    // FIXME

    // First of all, output the actual function.



    // Do not generate variadic methods, since rust does not allow
    // implementing them, and we don't do a good job at it anyway.





    // FIXME: use aster here.

    // If it's a constructor, we always return `Self`, and we inject the
    // "this" parameter, so there's no need to ask the user for it.
    //
    // Note that constructors in Clang are represented as functions with
    // return-type = void.




    // If it's a constructor, we need to insert an extra parameter with a
    // variable called `__bindgen_tmp` we're going to create.


























    // FIXME(emilio): These should probably use the path so it can
    // disambiguate between namespaces, just like is_opaque etc.



    // FIXME: Rust forbids repr with empty enums. Remove this condition when
    // this is allowed.
    //
    // TODO(emilio): Delegate this to the builders?




    // Only to avoid recomputing every time.
    // May be the same as "variant" if it's because the
    // enum is unnamed and we still haven't seen the
    // value.




    // A map where we keep a value -> variant relation.

    // Used to mangle the constants we generate in the unnamed-enum case.


    // NB: We defer the creation of constified variants, in case we find
    // another variant with the same value (which is the common thing to
    // do).







    // If it's an unnamed enum, or constification is enforced,
    // we also generate a constant so it can be properly
    // accessed.

































    // TODO: we should do something smart with nullptr, or maybe *const
    // c_void is enough?

    // FIXME: This doesn't generate the proper alignment, but we
    // can't do better right now. We should be able to use
    // i128/u128 when they're available.

    // We can't rely on the sizeof(Option<NonZero<_>>) ==
    // sizeof(NonZero<_>) optimization with opaque blobs (because
    // they aren't NonZero), so don't *ever* use an or_opaque
    // variant here.



    /* is_const = */

    // Regardless if we can properly represent the inner type, we
    // should always generate a proper pointer here, so use
    // infallible conversion of the inner type.

    // Avoid the first function pointer level, since it's already
    // represented in Rust.





    // This can happen if we generated an opaque type for a partial
    // template specialization, and we've hit an instantiation of
    // that partial specialization.

    // TODO: If the decl type is a template class/struct
    // declaration's member template declaration, it could rely on
    // generic template parameters from its outer template
    // class/struct. When we emit bindings for it, it could require
    // *more* type arguments than we have here, and we will need to
    // reconstruct them somehow. We don't have any means of doing
    // that reconstruction at this time.

    // Only pass type arguments for the type parameters that
    // the decl uses.




    // TODO: we might want to consider ignoring the reference return value.








    // TODO: Maybe warn here if there's a type/argument mismatch, or
    // something?







    // Handle overloaded functions by giving each overload its own unique
    // suffix.







    // Collect the actual used argument names




    // Public,


























    // TODO(emilio): The fmt::Debug impl could be way nicer with
    // std::intrinsics::type_name, but...


























    // XXX: I suck at aster.

    // Take into account the skip(1)
    // XXX Extra clone courtesy of the borrow checker.



    // FIXME: We could use the inner item to check this is really a
    // primitive type but, who the heck overrides these anyway?







    // From the C90 standard[1]:
    //
    //     A declaration of a parameter as "array of type" shall be
    //     adjusted to "qualified pointer to type", where the type
    //     qualifiers (if any) are those specified within the [ and ] of
    //     the array type derivation.
    //
    // [1]: http://c0x.coding-guidelines.com/6.7.5.3.html



    use std::error;
    use std::fmt;
    /// Errors that can occur during code generation.
    #[derive(Eq, PartialEq, Debug, Clone)]
    pub enum Error {

        /// Tried to generate an opaque blob for a type that did not have a layout.
        NoLayoutForOpaqueBlob,

        /// Tried to instantiate an opaque template definition, or a template
        /// definition that is too difficult for us to understand (like a partial
        /// template specialization).
        InstantiationOfOpaqueType,
    }
    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f , "{}" , error :: Error :: description ( self ))
        }
    }
    impl error::Error for Error {
        fn cause(&self) -> Option<&error::Error> { None }
        fn description(&self) -> &'static str {
            match *self {
                Error::NoLayoutForOpaqueBlob => {
                    "Tried to generate an opaque blob, but had no layout"
                }
                Error::InstantiationOfOpaqueType => {
                    "Instantiation of opaque template type or partial template specialization"
                }
            }
        }
    }
    /// A `Result` of `T` or an error of `bindgen::codegen::error::Error`.
    pub type Result<T> = ::std::result::Result<T, Error>;
}
mod helpers {
    //! Helpers for code generation that don't need macro expansion.
    use aster;
    use ir::layout::Layout;
    use syntax::ast;
    use syntax::ptr::P;
    pub mod attributes {
        use aster;
        use syntax::ast;
        pub fn allow(which_ones: &[&str]) -> ast::Attribute {
            aster::AstBuilder::new().attr().list("allow").words(which_ones).build()
        }
        pub fn repr(which: &str) -> ast::Attribute {
            aster::AstBuilder::new().attr().list("repr").words(&[which]).build()
        }
        pub fn repr_list(which_ones: &[&str]) -> ast::Attribute {
            aster::AstBuilder::new().attr().list("repr").words(which_ones).build()
        }
        pub fn derives(which_ones: &[&str]) -> ast::Attribute {
            aster::AstBuilder::new().attr().list("derive").words(which_ones).build()
        }
        pub fn inline() -> ast::Attribute {
            aster::AstBuilder::new().attr().word("inline")
        }
        pub fn doc(comment: &str) -> ast::Attribute {
            aster::AstBuilder::new().attr().doc(comment)
        }
        pub fn link_name(name: &str) -> ast::Attribute {
            aster::AstBuilder::new().attr().name_value("link_name").str(name)
        }
    }
    /// Generates a proper type for a field or type with a given `Layout`, that is,
    /// a type with the correct size and alignment restrictions.
    pub struct BlobTyBuilder {
        layout: Layout,
    }
    impl BlobTyBuilder {
        pub fn new(layout: Layout) -> Self { BlobTyBuilder{layout: layout,} }
        pub fn build(self) -> P<ast::Ty> {
            let opaque = self.layout.opaque();
            let ty_name =
                match opaque.known_rust_type_for_array() {
                    Some(ty) => ty,
                    None => {
                        warn!("Found unknown alignment on code generation!");
                        "u8"
                    }
                };
            let data_len = opaque.array_size().unwrap_or(self.layout.size);
            let inner_ty =
                aster::AstBuilder::new().ty().path().id(ty_name).build();
            if data_len == 1 {
                inner_ty
            } else {
                aster::ty::TyBuilder::new().array(data_len).build(inner_ty)
            }
        }
    }
    pub mod ast_ty {
        use aster;
        use ir::context::BindgenContext;
        use ir::function::FunctionSig;
        use ir::ty::FloatKind;
        use syntax::ast;
        use syntax::ptr::P;
        pub fn raw_type(ctx: &BindgenContext, name: &str) -> P<ast::Ty> {
            let ident = ctx.rust_ident_raw(&name);
            match ctx.options().ctypes_prefix {
                Some(ref prefix) => {
                    let prefix = ctx.rust_ident_raw(prefix);
                    {
                        let ext_cx = &*ctx.ext_cx();
                        ::quasi::parse_ty_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                          {
                                                                                              #[allow(unused_imports)]
                                                                                              use quasi::IntoWrappedIterator;
                                                                                              #[allow(unused_imports)]
                                                                                              use quasi::IntoWrappedRepeat;
                                                                                              let _sp =
                                                                                                  ext_cx.call_site();
                                                                                              let mut tt =
                                                                                                  ::std::vec::Vec::new();
                                                                                              tt.extend(::quasi::ToTokens::to_tokens(&prefix,
                                                                                                                                     ext_cx).into_iter());
                                                                                              tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                              ::syntax::parse::token::ModSep));
                                                                                              tt.extend(::quasi::ToTokens::to_tokens(&ident,
                                                                                                                                     ext_cx).into_iter());
                                                                                              tt
                                                                                          }))
                    }
                }
                None => {
                    let ext_cx = &*ctx.ext_cx();
                    ::quasi::parse_ty_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                      {
                                                                                          #[allow(unused_imports)]
                                                                                          use quasi::IntoWrappedIterator;
                                                                                          #[allow(unused_imports)]
                                                                                          use quasi::IntoWrappedRepeat;
                                                                                          let _sp =
                                                                                              ext_cx.call_site();
                                                                                          let mut tt =
                                                                                              ::std::vec::Vec::new();
                                                                                          tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                          ::syntax::parse::token::ModSep));
                                                                                          tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                          ::syntax::parse::token::Ident(ext_cx.ident_of("std"))));
                                                                                          tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                          ::syntax::parse::token::ModSep));
                                                                                          tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                          ::syntax::parse::token::Ident(ext_cx.ident_of("os"))));
                                                                                          tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                          ::syntax::parse::token::ModSep));
                                                                                          tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                          ::syntax::parse::token::Ident(ext_cx.ident_of("raw"))));
                                                                                          tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                          ::syntax::parse::token::ModSep));
                                                                                          tt.extend(::quasi::ToTokens::to_tokens(&ident,
                                                                                                                                 ext_cx).into_iter());
                                                                                          tt
                                                                                      }))
                }
            }
        }
        pub fn float_kind_rust_type(ctx: &BindgenContext, fk: FloatKind)
         -> P<ast::Ty> {
            match (fk, ctx.options().convert_floats) {
                (FloatKind::Float, true) => aster::ty::TyBuilder::new().f32(),
                (FloatKind::Double, true) | (FloatKind::LongDouble, true) =>
                aster::ty::TyBuilder::new().f64(),
                (FloatKind::Float, false) => raw_type(ctx, "c_float"),
                (FloatKind::Double, false) | (FloatKind::LongDouble, false) =>
                raw_type(ctx, "c_double"),
                (FloatKind::Float128, _) => {
                    aster::ty::TyBuilder::new().array(16).u8()
                }
            }
        }
        pub fn int_expr(val: i64) -> P<ast::Expr> {
            use std::i64;
            let expr = aster::AstBuilder::new().expr();
            if val == i64::MIN {
                expr.neg().uint(1u64 << 63)
            } else { expr.int(val) }
        }
        pub fn bool_expr(val: bool) -> P<ast::Expr> {
            aster::AstBuilder::new().expr().bool(val)
        }
        pub fn byte_array_expr(bytes: &[u8]) -> P<ast::Expr> {
            let mut vec = Vec::with_capacity(bytes.len() + 1);
            for byte in bytes { vec.push(int_expr(*byte as i64)); }
            vec.push(int_expr(0));
            let kind = ast::ExprKind::Array(vec);
            aster::AstBuilder::new().expr().build_expr_kind(kind)
        }
        pub fn cstr_expr(mut string: String) -> P<ast::Expr> {
            string.push('\u{0}');
            aster::AstBuilder::new().expr().build_lit(aster::AstBuilder::new().lit().byte_str(string))
        }
        pub fn float_expr(ctx: &BindgenContext, f: f64)
         -> Result<P<ast::Expr>, ()> {
            use aster::symbol::ToSymbol;
            if f.is_finite() {
                let mut string = f.to_string();
                if !string.contains('.') { string.push('.'); }
                let kind =
                    ast::LitKind::FloatUnsuffixed(string.as_str().to_symbol());
                return Ok(aster::AstBuilder::new().expr().lit().build_lit(kind))
            }
            let prefix = ctx.trait_prefix();
            if f.is_nan() {
                return Ok({
                              let ext_cx = &*ctx.ext_cx();
                              ::quasi::parse_expr_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                                  {
                                                                                                      #[allow(unused_imports)]
                                                                                                      use quasi::IntoWrappedIterator;
                                                                                                      #[allow(unused_imports)]
                                                                                                      use quasi::IntoWrappedRepeat;
                                                                                                      let _sp =
                                                                                                          ext_cx.call_site();
                                                                                                      let mut tt =
                                                                                                          ::std::vec::Vec::new();
                                                                                                      tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                      ::syntax::parse::token::ModSep));
                                                                                                      tt.extend(::quasi::ToTokens::to_tokens(&prefix,
                                                                                                                                             ext_cx).into_iter());
                                                                                                      tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                      ::syntax::parse::token::ModSep));
                                                                                                      tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                      ::syntax::parse::token::Ident(ext_cx.ident_of("f64"))));
                                                                                                      tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                      ::syntax::parse::token::ModSep));
                                                                                                      tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                      ::syntax::parse::token::Ident(ext_cx.ident_of("NAN"))));
                                                                                                      tt
                                                                                                  }))
                          });
            }
            if f.is_infinite() {
                return Ok(if f.is_sign_positive() {
                              {
                                  let ext_cx = &*ctx.ext_cx();
                                  ::quasi::parse_expr_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                                      {
                                                                                                          #[allow(unused_imports)]
                                                                                                          use quasi::IntoWrappedIterator;
                                                                                                          #[allow(unused_imports)]
                                                                                                          use quasi::IntoWrappedRepeat;
                                                                                                          let _sp =
                                                                                                              ext_cx.call_site();
                                                                                                          let mut tt =
                                                                                                              ::std::vec::Vec::new();
                                                                                                          tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                          ::syntax::parse::token::ModSep));
                                                                                                          tt.extend(::quasi::ToTokens::to_tokens(&prefix,
                                                                                                                                                 ext_cx).into_iter());
                                                                                                          tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                          ::syntax::parse::token::ModSep));
                                                                                                          tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                          ::syntax::parse::token::Ident(ext_cx.ident_of("f64"))));
                                                                                                          tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                          ::syntax::parse::token::ModSep));
                                                                                                          tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                          ::syntax::parse::token::Ident(ext_cx.ident_of("INFINITY"))));
                                                                                                          tt
                                                                                                      }))
                              }
                          } else {
                              {
                                  let ext_cx = &*ctx.ext_cx();
                                  ::quasi::parse_expr_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                                      {
                                                                                                          #[allow(unused_imports)]
                                                                                                          use quasi::IntoWrappedIterator;
                                                                                                          #[allow(unused_imports)]
                                                                                                          use quasi::IntoWrappedRepeat;
                                                                                                          let _sp =
                                                                                                              ext_cx.call_site();
                                                                                                          let mut tt =
                                                                                                              ::std::vec::Vec::new();
                                                                                                          tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                          ::syntax::parse::token::ModSep));
                                                                                                          tt.extend(::quasi::ToTokens::to_tokens(&prefix,
                                                                                                                                                 ext_cx).into_iter());
                                                                                                          tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                          ::syntax::parse::token::ModSep));
                                                                                                          tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                          ::syntax::parse::token::Ident(ext_cx.ident_of("f64"))));
                                                                                                          tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                          ::syntax::parse::token::ModSep));
                                                                                                          tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                          ::syntax::parse::token::Ident(ext_cx.ident_of("NEG_INFINITY"))));
                                                                                                          tt
                                                                                                      }))
                              }
                          });
            }
            warn!("Unknown non-finite float number: {:?}" , f);
            return Err(());
        }
        pub fn arguments_from_signature(signature: &FunctionSig,
                                        ctx: &BindgenContext)
         -> Vec<P<ast::Expr>> {
            let mut unnamed_arguments = 0;
            signature.argument_types().iter().map(|&(ref name, _ty)|
                                                      {
                                                          let arg_name =
                                                              match *name {
                                                                  Some(ref name)
                                                                  =>
                                                                  ctx.rust_mangle(name).into_owned(),
                                                                  None => {
                                                                      unnamed_arguments
                                                                          +=
                                                                          1;
                                                                      format!("arg{}"
                                                                              ,
                                                                              unnamed_arguments)
                                                                  }
                                                              };
                                                          aster::expr::ExprBuilder::new().id(arg_name)
                                                      }).collect::<Vec<_>>()
        }
    }
}
mod struct_layout {
    //! Helpers for code generation that need struct layout
    use super::helpers::BlobTyBuilder;
    use aster::struct_field::StructFieldBuilder;
    use ir::comp::CompInfo;
    use ir::context::BindgenContext;
    use ir::layout::Layout;
    use ir::ty::{Type, TypeKind};
    use std::cmp;
    use std::mem;
    use syntax::ast;
    /// Trace the layout of struct.
    pub struct StructLayoutTracker<'a, 'ctx: 'a> {
        ctx: &'a BindgenContext<'ctx>,
        comp: &'a CompInfo,
        latest_offset: usize,
        padding_count: usize,
        latest_field_layout: Option<Layout>,
        max_field_align: usize,
        last_field_was_bitfield: bool,
    }
    /// Returns a size aligned to a given value.
    pub fn align_to(size: usize, align: usize) -> usize {
        if align == 0 { return size; }
        let rem = size % align;
        if rem == 0 { return size; }
        size + align - rem
    }
    /// Returns the amount of bytes from a given amount of bytes, rounding up.
    pub fn bytes_from_bits(n: usize) -> usize {
        if n % 8 == 0 { return n / 8; }
        n / 8 + 1
    }
    /// Returns the lower power of two byte count that can hold at most n bits.
    pub fn bytes_from_bits_pow2(mut n: usize) -> usize {
        if n == 0 { return 0; }
        if n <= 8 { return 1; }
        if !n.is_power_of_two() { n = n.next_power_of_two(); }
        n / 8
    }
    #[test]
    fn test_align_to() {
        assert_eq!(align_to ( 1 , 1 ) , 1);
        assert_eq!(align_to ( 1 , 2 ) , 2);
        assert_eq!(align_to ( 1 , 4 ) , 4);
        assert_eq!(align_to ( 5 , 1 ) , 5);
        assert_eq!(align_to ( 17 , 4 ) , 20);
    }
    #[test]
    fn test_bytes_from_bits_pow2() {
        assert_eq!(bytes_from_bits_pow2 ( 0 ) , 0);
        for i in 1..9 { assert_eq!(bytes_from_bits_pow2 ( i ) , 1); }
        for i in 9..17 { assert_eq!(bytes_from_bits_pow2 ( i ) , 2); }
        for i in 17..33 { assert_eq!(bytes_from_bits_pow2 ( i ) , 4); }
    }
    #[test]
    fn test_bytes_from_bits() {
        assert_eq!(bytes_from_bits ( 0 ) , 0);
        for i in 1..9 { assert_eq!(bytes_from_bits ( i ) , 1); }
        for i in 9..17 { assert_eq!(bytes_from_bits ( i ) , 2); }
        for i in 17..25 { assert_eq!(bytes_from_bits ( i ) , 3); }
    }
    impl <'a, 'ctx> StructLayoutTracker<'a, 'ctx> {
        pub fn new(ctx: &'a BindgenContext<'ctx>, comp: &'a CompInfo)
         -> Self {
            StructLayoutTracker{ctx: ctx,
                                comp: comp,
                                latest_offset: 0,
                                padding_count: 0,
                                latest_field_layout: None,
                                max_field_align: 0,
                                last_field_was_bitfield: false,}
        }
        pub fn saw_vtable(&mut self) {
            let ptr_size = mem::size_of::<*mut ()>();
            self.latest_offset += ptr_size;
            self.latest_field_layout = Some(Layout::new(ptr_size, ptr_size));
            self.max_field_align = ptr_size;
        }
        pub fn saw_base(&mut self, base_ty: &Type) {
            if let Some(layout) = base_ty.layout(self.ctx) {
                self.align_to_latest_field(layout);
                self.latest_offset +=
                    self.padding_bytes(layout) + layout.size;
                self.latest_field_layout = Some(layout);
                self.max_field_align =
                    cmp::max(self.max_field_align, layout.align);
            }
        }
        pub fn saw_bitfield_batch(&mut self, layout: Layout) {
            self.align_to_latest_field(layout);
            self.latest_offset += layout.size;
            debug!("Offset: <bitfield>: {} -> {}" , self . latest_offset -
                   layout . size , self . latest_offset);
            self.latest_field_layout = Some(layout);
            self.last_field_was_bitfield = true;
        }
        pub fn saw_union(&mut self, layout: Layout) {
            self.align_to_latest_field(layout);
            self.latest_offset += self.padding_bytes(layout) + layout.size;
            self.latest_field_layout = Some(layout);
            self.max_field_align =
                cmp::max(self.max_field_align, layout.align);
        }
        /// Add a padding field if necessary for a given new field _before_ adding
        /// that field.
        pub fn pad_field(&mut self, field_name: &str, field_ty: &Type,
                         field_offset: Option<usize>)
         -> Option<ast::StructField> {
            let mut field_layout =
                match field_ty.layout(self.ctx) {
                    Some(l) => l,
                    None => return None,
                };
            if let TypeKind::Array(inner, len) =
                   *field_ty.canonical_type(self.ctx).kind() {
                if let Some(layout) =
                       self.ctx.resolve_type(inner).layout(self.ctx) {
                    if layout.align > mem::size_of::<*mut ()>() {
                        field_layout.size =
                            align_to(layout.size, layout.align) * len;
                        field_layout.align = mem::size_of::<*mut ()>();
                    }
                }
            }
            let will_merge_with_bitfield =
                self.align_to_latest_field(field_layout);
            let padding_layout =
                if self.comp.packed() {
                    None
                } else {
                    let padding_bytes =
                        match field_offset {
                            Some(offset) if offset / 8 > self.latest_offset =>
                            {
                                offset / 8 - self.latest_offset
                            }
                            _ if
                            will_merge_with_bitfield ||
                                field_layout.align == 0 => 0,
                            _ => self.padding_bytes(field_layout),
                        };
                    let need_padding =
                        padding_bytes >= field_layout.align ||
                            field_layout.align > mem::size_of::<*mut ()>();
                    self.latest_offset += padding_bytes;
                    debug!("Offset: <padding>: {} -> {}" , self .
                           latest_offset - padding_bytes , self .
                           latest_offset);
                    debug!("align field {} to {}/{} with {} padding bytes {:?}"
                           , field_name , self . latest_offset , field_offset
                           . unwrap_or ( 0 ) / 8 , padding_bytes ,
                           field_layout);
                    if need_padding && padding_bytes != 0 {
                        Some(Layout::new(padding_bytes,
                                         cmp::min(field_layout.align,
                                                  mem::size_of::<*mut ()>())))
                    } else { None }
                };
            self.latest_offset += field_layout.size;
            self.latest_field_layout = Some(field_layout);
            self.max_field_align =
                cmp::max(self.max_field_align, field_layout.align);
            self.last_field_was_bitfield = false;
            debug!("Offset: {}: {} -> {}" , field_name , self . latest_offset
                   - field_layout . size , self . latest_offset);
            padding_layout.map(|layout| self.padding_field(layout))
        }
        pub fn pad_struct(&mut self, name: &str, layout: Layout)
         -> Option<ast::StructField> {
            if layout.size < self.latest_offset {
                error!("Calculated wrong layout for {}, too more {} bytes" ,
                       name , self . latest_offset - layout . size);
                return None;
            }
            let padding_bytes = layout.size - self.latest_offset;
            if padding_bytes > 0 &&
                   (padding_bytes >= layout.align ||
                        (self.last_field_was_bitfield &&
                             padding_bytes >=
                                 self.latest_field_layout.unwrap().align) ||
                        layout.align > mem::size_of::<*mut ()>()) {
                let layout =
                    if self.comp.packed() {
                        Layout::new(padding_bytes, 1)
                    } else if self.last_field_was_bitfield ||
                                  layout.align > mem::size_of::<*mut ()>() {
                        Layout::for_size(padding_bytes)
                    } else { Layout::new(padding_bytes, layout.align) };
                debug!("pad bytes to struct {}, {:?}" , name , layout);
                Some(self.padding_field(layout))
            } else { None }
        }
        pub fn align_struct(&self, layout: Layout)
         -> Option<ast::StructField> {
            if self.max_field_align < layout.align &&
                   layout.align <= mem::size_of::<*mut ()>() {
                let ty =
                    BlobTyBuilder::new(Layout::new(0, layout.align)).build();
                Some(StructFieldBuilder::named("__bindgen_align").pub_().build_ty(ty))
            } else { None }
        }
        fn padding_bytes(&self, layout: Layout) -> usize {
            align_to(self.latest_offset, layout.align) - self.latest_offset
        }
        fn padding_field(&mut self, layout: Layout) -> ast::StructField {
            let ty = BlobTyBuilder::new(layout).build();
            let padding_count = self.padding_count;
            self.padding_count += 1;
            let padding_field_name =
                format!("__bindgen_padding_{}" , padding_count);
            self.max_field_align =
                cmp::max(self.max_field_align, layout.align);
            StructFieldBuilder::named(padding_field_name).pub_().build_ty(ty)
        }
        /// Returns whether the new field is known to merge with a bitfield.
        ///
        /// This is just to avoid doing the same check also in pad_field.
        fn align_to_latest_field(&mut self, new_field_layout: Layout)
         -> bool {
            if self.comp.packed() { return false; }
            let layout =
                match self.latest_field_layout {
                    Some(l) => l,
                    None => return false,
                };
            debug!("align_to_bitfield? {}: {:?} {:?}" , self .
                   last_field_was_bitfield , layout , new_field_layout);
            if self.last_field_was_bitfield &&
                   new_field_layout.align <= layout.size % layout.align &&
                   new_field_layout.size <= layout.size % layout.align {
                debug!("Will merge with bitfield");
                return true;
            }
            self.latest_offset += self.padding_bytes(layout);
            return false;
        }
    }
}
use self::helpers::{BlobTyBuilder, attributes};
use self::struct_layout::{StructLayoutTracker, bytes_from_bits_pow2};
use self::struct_layout::{align_to, bytes_from_bits};
use aster;
use ir::annotations::FieldAccessorKind;
use ir::comp::{Base, CompInfo, CompKind, Field, Method, MethodKind};
use ir::context::{BindgenContext, ItemId};
use ir::derive::{CanDeriveCopy, CanDeriveDebug, CanDeriveDefault};
use ir::dot;
use ir::enum_ty::{Enum, EnumVariant, EnumVariantValue};
use ir::function::{Function, FunctionSig};
use ir::int::IntKind;
use ir::item::{Item, ItemAncestors, ItemCanonicalName, ItemCanonicalPath,
               ItemSet};
use ir::item_kind::ItemKind;
use ir::layout::Layout;
use ir::module::Module;
use ir::objc::{ObjCInterface, ObjCMethod};
use ir::template::{AsNamed, TemplateInstantiation};
use ir::ty::{TemplateDeclaration, Type, TypeKind};
use ir::var::Var;
use std::borrow::Cow;
use std::cell::Cell;
use std::cmp;
use std::collections::{HashSet, VecDeque};
use std::collections::hash_map::{Entry, HashMap};
use std::fmt::Write;
use std::mem;
use std::ops;
use syntax::abi::Abi;
use syntax::ast;
use syntax::codemap::{Span, respan};
use syntax::ptr::P;
fn root_import_depth(ctx: &BindgenContext, item: &Item) -> usize {
    if !ctx.options().enable_cxx_namespaces { return 0; }
    item.ancestors(ctx).filter(|id|
                                   ctx.resolve_item(*id).is_module()).fold(1,
                                                                           |i,
                                                                            _|
                                                                               i
                                                                                   +
                                                                                   1)
}
fn top_level_path(ctx: &BindgenContext, item: &Item) -> Vec<ast::Ident> {
    let mut path = vec!(ctx . rust_ident_raw ( "self" ));
    if ctx.options().enable_cxx_namespaces {
        let super_ = ctx.rust_ident_raw("super");
        for _ in 0..root_import_depth(ctx, item) {
            path.push(super_.clone());
        }
    }
    path
}
fn root_import(ctx: &BindgenContext, module: &Item) -> P<ast::Item> {
    assert!(ctx . options (  ) . enable_cxx_namespaces ,
            "Somebody messed it up");
    assert!(module . is_module (  ));
    let mut path = top_level_path(ctx, module);
    let root = ctx.root_module().canonical_name(ctx);
    let root_ident = ctx.rust_ident(&root);
    path.push(root_ident);
    let use_root =
        aster::AstBuilder::new().item().use_().ids(path).build().build();
    {
        let ext_cx = &*ctx.ext_cx();
        ::quasi::parse_item_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                            {
                                                                                #[allow(unused_imports)]
                                                                                use quasi::IntoWrappedIterator;
                                                                                #[allow(unused_imports)]
                                                                                use quasi::IntoWrappedRepeat;
                                                                                let _sp =
                                                                                    ext_cx.call_site();
                                                                                let mut tt =
                                                                                    ::std::vec::Vec::new();
                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                ::syntax::parse::token::Pound));
                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                ::syntax::parse::token::OpenDelim(::syntax::parse::token::Bracket)));
                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                ::syntax::parse::token::Ident(ext_cx.ident_of("allow"))));
                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                ::syntax::parse::token::Ident(ext_cx.ident_of("unused_imports"))));
                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                ::syntax::parse::token::CloseDelim(::syntax::parse::token::Bracket)));
                                                                                tt.extend(::quasi::ToTokens::to_tokens(&use_root,
                                                                                                                       ext_cx).into_iter());
                                                                                tt
                                                                            }))
    }.unwrap()
}
struct CodegenResult<'a> {
    items: Vec<P<ast::Item>>,
    /// A monotonic counter used to add stable unique id's to stuff that doesn't
    /// need to be referenced by anything.
    codegen_id: &'a Cell<usize>,
    /// Whether an union has been generated at least once.
    saw_union: bool,
    /// Whether an incomplete array has been generated at least once.
    saw_incomplete_array: bool,
    /// Whether Objective C types have been seen at least once.
    saw_objc: bool,
    items_seen: HashSet<ItemId>,
    /// The set of generated function/var names, needed because in C/C++ is
    /// legal to do something like:
    ///
    /// ```c++
    /// extern "C" {
    ///   void foo();
    ///   extern int bar;
    /// }
    ///
    /// extern "C" {
    ///   void foo();
    ///   extern int bar;
    /// }
    /// ```
    ///
    /// Being these two different declarations.
    functions_seen: HashSet<String>,
    vars_seen: HashSet<String>,
    /// Used for making bindings to overloaded functions. Maps from a canonical
    /// function name to the number of overloads we have already codegen'd for
    /// that name. This lets us give each overload a unique suffix.
    overload_counters: HashMap<String, u32>,
}
impl <'a> CodegenResult<'a> {
    fn new(codegen_id: &'a Cell<usize>) -> Self {
        CodegenResult{items: vec!(),
                      saw_union: false,
                      saw_incomplete_array: false,
                      saw_objc: false,
                      codegen_id: codegen_id,
                      items_seen: Default::default(),
                      functions_seen: Default::default(),
                      vars_seen: Default::default(),
                      overload_counters: Default::default(),}
    }
    fn saw_union(&mut self) { self.saw_union = true; }
    fn saw_incomplete_array(&mut self) { self.saw_incomplete_array = true; }
    fn saw_objc(&mut self) { self.saw_objc = true; }
    fn seen(&self, item: ItemId) -> bool { self.items_seen.contains(&item) }
    fn set_seen(&mut self, item: ItemId) { self.items_seen.insert(item); }
    fn seen_function(&self, name: &str) -> bool {
        self.functions_seen.contains(name)
    }
    fn saw_function(&mut self, name: &str) {
        self.functions_seen.insert(name.into());
    }
    /// Get the overload number for the given function name. Increments the
    /// counter internally so the next time we ask for the overload for this
    /// name, we get the incremented value, and so on.
    fn overload_number(&mut self, name: &str) -> u32 {
        let mut counter =
            self.overload_counters.entry(name.into()).or_insert(0);
        let number = *counter;
        *counter += 1;
        number
    }
    fn seen_var(&self, name: &str) -> bool { self.vars_seen.contains(name) }
    fn saw_var(&mut self, name: &str) { self.vars_seen.insert(name.into()); }
    fn inner<F>(&mut self, cb: F) -> Vec<P<ast::Item>> where
     F: FnOnce(&mut Self) {
        let mut new = Self::new(self.codegen_id);
        cb(&mut new);
        self.saw_union |= new.saw_union;
        self.saw_incomplete_array |= new.saw_incomplete_array;
        self.saw_objc |= new.saw_objc;
        new.items
    }
}
impl <'a> ops::Deref for CodegenResult<'a> {
    type
    Target
    =
    Vec<P<ast::Item>>;
    fn deref(&self) -> &Self::Target { &self.items }
}
impl <'a> ops::DerefMut for CodegenResult<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.items }
}
struct ForeignModBuilder {
    inner: ast::ForeignMod,
}
impl ForeignModBuilder {
    fn new(abi: Abi) -> Self {
        ForeignModBuilder{inner: ast::ForeignMod{abi: abi, items: vec!(),},}
    }
    fn with_foreign_item(mut self, item: ast::ForeignItem) -> Self {
        self.inner.items.push(item);
        self
    }
    #[allow(dead_code)]
    fn with_foreign_items<I>(mut self, items: I) -> Self where
     I: IntoIterator<Item = ast::ForeignItem> {
        self.inner.items.extend(items.into_iter());
        self
    }
    fn build(self, ctx: &BindgenContext) -> P<ast::Item> {
        use syntax::codemap::DUMMY_SP;
        P(ast::Item{ident: ctx.rust_ident(""),
                    id: ast::DUMMY_NODE_ID,
                    node: ast::ItemKind::ForeignMod(self.inner),
                    vis: ast::Visibility::Public,
                    attrs: vec!(),
                    span: DUMMY_SP,})
    }
}
/// A trait to convert a rust type into a pointer, optionally const, to the same
/// type.
///
/// This is done due to aster's lack of pointer builder, I guess I should PR
/// there.
trait ToPtr {
    fn to_ptr(self, is_const: bool, span: Span)
    -> P<ast::Ty>;
}
impl ToPtr for P<ast::Ty> {
    fn to_ptr(self, is_const: bool, span: Span) -> Self {
        let ty =
            ast::TyKind::Ptr(ast::MutTy{ty: self,
                                        mutbl:
                                            if is_const {
                                                ast::Mutability::Immutable
                                            } else {
                                                ast::Mutability::Mutable
                                            },});
        P(ast::Ty{id: ast::DUMMY_NODE_ID, node: ty, span: span,})
    }
}
trait CodeGenerator {
    /// Extra information from the caller.
    type
    Extra;
    fn codegen<'a>(&self, ctx: &BindgenContext,
                   result: &mut CodegenResult<'a>,
                   whitelisted_items: &ItemSet, extra: &Self::Extra);
}
impl CodeGenerator for Item {
    type
    Extra
    =
    ();
    fn codegen<'a>(&self, ctx: &BindgenContext,
                   result: &mut CodegenResult<'a>,
                   whitelisted_items: &ItemSet, _extra: &()) {
        if self.is_hidden(ctx) || result.seen(self.id()) {
            debug!("<Item as CodeGenerator>::codegen: Ignoring hidden or seen: \
                   self = {:?}"
                   , self);
            return;
        }
        debug!("<Item as CodeGenerator>::codegen: self = {:?}" , self);
        if !whitelisted_items.contains(&self.id()) {
            error!("Found non-whitelisted item in code generation: {:?}" ,
                   self);
        }
        result.set_seen(self.id());
        match *self.kind() {
            ItemKind::Module(ref module) => {
                module.codegen(ctx, result, whitelisted_items, self);
            }
            ItemKind::Function(ref fun) => {
                if ctx.options().codegen_config.functions {
                    fun.codegen(ctx, result, whitelisted_items, self);
                }
            }
            ItemKind::Var(ref var) => {
                if ctx.options().codegen_config.vars {
                    var.codegen(ctx, result, whitelisted_items, self);
                }
            }
            ItemKind::Type(ref ty) => {
                if ctx.options().codegen_config.types {
                    ty.codegen(ctx, result, whitelisted_items, self);
                }
            }
        }
    }
}
impl CodeGenerator for Module {
    type
    Extra
    =
    Item;
    fn codegen<'a>(&self, ctx: &BindgenContext,
                   result: &mut CodegenResult<'a>,
                   whitelisted_items: &ItemSet, item: &Item) {
        debug!("<Module as CodeGenerator>::codegen: item = {:?}" , item);
        let codegen_self =
            |result: &mut CodegenResult, found_any: &mut bool|
                {
                    for child in self.children() {
                        if whitelisted_items.contains(child) {
                            *found_any = true;
                            ctx.resolve_item(*child).codegen(ctx, result,
                                                             whitelisted_items,
                                                             &());
                        }
                    }
                    if item.id() == ctx.root_module() {
                        if result.saw_union && !ctx.options().unstable_rust {
                            utils::prepend_union_types(ctx, &mut *result);
                        }
                        if result.saw_incomplete_array {
                            utils::prepend_incomplete_array_types(ctx,
                                                                  &mut *result);
                        }
                        if ctx.need_bindegen_complex_type() {
                            utils::prepend_complex_type(ctx, &mut *result);
                        }
                        if result.saw_objc {
                            utils::prepend_objc_header(ctx, &mut *result);
                        }
                    }
                };
        if !ctx.options().enable_cxx_namespaces ||
               (self.is_inline() &&
                    !ctx.options().conservative_inline_namespaces) {
            codegen_self(result, &mut false);
            return;
        }
        let mut found_any = false;
        let inner_items =
            result.inner(|result|
                             {
                                 result.push(root_import(ctx, item));
                                 codegen_self(result, &mut found_any);
                             });
        if !found_any { return; }
        let module =
            ast::ItemKind::Mod(ast::Mod{inner: ctx.span(),
                                        items: inner_items,});
        let name = item.canonical_name(ctx);
        let item_builder = aster::AstBuilder::new().item().pub_();
        let item =
            if name == "root" {
                let attrs =
                    &["non_snake_case", "non_camel_case_types",
                      "non_upper_case_globals"];
                item_builder.with_attr(attributes::allow(attrs)).build_item_kind(name,
                                                                                 module)
            } else { item_builder.build_item_kind(name, module) };
        result.push(item);
    }
}
impl CodeGenerator for Var {
    type
    Extra
    =
    Item;
    fn codegen<'a>(&self, ctx: &BindgenContext,
                   result: &mut CodegenResult<'a>,
                   _whitelisted_items: &ItemSet, item: &Item) {
        use ir::var::VarType;
        debug!("<Var as CodeGenerator>::codegen: item = {:?}" , item);
        let canonical_name = item.canonical_name(ctx);
        if result.seen_var(&canonical_name) { return; }
        result.saw_var(&canonical_name);
        let ty = self.ty().to_rust_ty_or_opaque(ctx, &());
        if let Some(val) = self.val() {
            let const_item =
                aster::AstBuilder::new().item().pub_().const_(canonical_name).expr();
            let item =
                match *val {
                    VarType::Bool(val) => {
                        const_item.build(helpers::ast_ty::bool_expr(val)).build(ty)
                    }
                    VarType::Int(val) => {
                        const_item.build(helpers::ast_ty::int_expr(val)).build(ty)
                    }
                    VarType::String(ref bytes) => {
                        let len = bytes.len() + 1;
                        let ty =
                            {
                                let ext_cx = &*ctx.ext_cx();
                                ::quasi::parse_ty_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                                  {
                                                                                                      #[allow(unused_imports)]
                                                                                                      use quasi::IntoWrappedIterator;
                                                                                                      #[allow(unused_imports)]
                                                                                                      use quasi::IntoWrappedRepeat;
                                                                                                      let _sp =
                                                                                                          ext_cx.call_site();
                                                                                                      let mut tt =
                                                                                                          ::std::vec::Vec::new();
                                                                                                      tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                      ::syntax::parse::token::OpenDelim(::syntax::parse::token::Bracket)));
                                                                                                      tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                      ::syntax::parse::token::Ident(ext_cx.ident_of("u8"))));
                                                                                                      tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                      ::syntax::parse::token::Semi));
                                                                                                      tt.extend(::quasi::ToTokens::to_tokens(&len,
                                                                                                                                             ext_cx).into_iter());
                                                                                                      tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                      ::syntax::parse::token::CloseDelim(::syntax::parse::token::Bracket)));
                                                                                                      tt
                                                                                                  }))
                            };
                        match String::from_utf8(bytes.clone()) {
                            Ok(string) => {
                                const_item.build(helpers::ast_ty::cstr_expr(string)).build({
                                                                                               let ext_cx =
                                                                                                   &*ctx.ext_cx();
                                                                                               ::quasi::parse_ty_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                                                                                                 {
                                                                                                                                                                     #[allow(unused_imports)]
                                                                                                                                                                     use quasi::IntoWrappedIterator;
                                                                                                                                                                     #[allow(unused_imports)]
                                                                                                                                                                     use quasi::IntoWrappedRepeat;
                                                                                                                                                                     let _sp =
                                                                                                                                                                         ext_cx.call_site();
                                                                                                                                                                     let mut tt =
                                                                                                                                                                         ::std::vec::Vec::new();
                                                                                                                                                                     tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                     ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                                                                                                     tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                     ::syntax::parse::token::Lifetime(ext_cx.ident_of("\'static"))));
                                                                                                                                                                     tt.extend(::quasi::ToTokens::to_tokens(&ty,
                                                                                                                                                                                                            ext_cx).into_iter());
                                                                                                                                                                     tt
                                                                                                                                                                 }))
                                                                                           })
                            }
                            Err(..) => {
                                const_item.build(helpers::ast_ty::byte_array_expr(bytes)).build(ty)
                            }
                        }
                    }
                    VarType::Float(f) => {
                        match helpers::ast_ty::float_expr(ctx, f) {
                            Ok(expr) => { const_item.build(expr).build(ty) }
                            Err(..) => return,
                        }
                    }
                    VarType::Char(c) => {
                        const_item.build(aster::AstBuilder::new().expr().lit().byte(c)).build(ty)
                    }
                };
            result.push(item);
        } else {
            let mut attrs = vec!();
            if let Some(mangled) = self.mangled_name() {
                attrs.push(attributes::link_name(mangled));
            } else if canonical_name != self.name() {
                attrs.push(attributes::link_name(self.name()));
            }
            let item =
                ast::ForeignItem{ident: ctx.rust_ident_raw(&canonical_name),
                                 attrs: attrs,
                                 node:
                                     ast::ForeignItemKind::Static(ty,
                                                                  !self.is_const()),
                                 id: ast::DUMMY_NODE_ID,
                                 span: ctx.span(),
                                 vis: ast::Visibility::Public,};
            let item =
                ForeignModBuilder::new(Abi::C).with_foreign_item(item).build(ctx);
            result.push(item);
        }
    }
}
impl CodeGenerator for Type {
    type
    Extra
    =
    Item;
    fn codegen<'a>(&self, ctx: &BindgenContext,
                   result: &mut CodegenResult<'a>,
                   whitelisted_items: &ItemSet, item: &Item) {
        debug!("<Type as CodeGenerator>::codegen: item = {:?}" , item);
        match *self.kind() {
            TypeKind::Void | TypeKind::NullPtr | TypeKind::Int(..) |
            TypeKind::Float(..) | TypeKind::Complex(..) | TypeKind::Array(..)
            | TypeKind::Pointer(..) | TypeKind::BlockPointer |
            TypeKind::Reference(..) | TypeKind::Function(..) |
            TypeKind::ResolvedTypeRef(..) | TypeKind::Opaque | TypeKind::Named
            => {
                return;
            }
            TypeKind::TemplateInstantiation(ref inst) => {
                inst.codegen(ctx, result, whitelisted_items, item)
            }
            TypeKind::Comp(ref ci) => {
                ci.codegen(ctx, result, whitelisted_items, item)
            }
            TypeKind::TemplateAlias(inner, _) | TypeKind::Alias(inner) => {
                let inner_item = ctx.resolve_item(inner);
                let name = item.canonical_name(ctx);
                if inner_item.canonical_name(ctx) == name { return; }
                let spelling = self.name().expect("Unnamed alias?");
                if utils::type_from_named(ctx, spelling, inner).is_some() {
                    return;
                }
                let mut used_template_params = item.used_template_params(ctx);
                let inner_rust_type =
                    if item.is_opaque(ctx) {
                        used_template_params = None;
                        self.to_opaque(ctx, item)
                    } else {
                        inner_item.try_to_rust_ty_or_opaque(ctx,
                                                            &()).unwrap_or_else(|_|
                                                                                    self.to_opaque(ctx,
                                                                                                   item))
                    };
                {
                    let inner_canon_type =
                        inner_item.expect_type().canonical_type(ctx);
                    if inner_canon_type.is_invalid_named_type() {
                        warn!("Item contained invalid named type, skipping: \
                              {:?}, {:?}"
                              , item , inner_item);
                        return;
                    }
                }
                let rust_name = ctx.rust_ident(&name);
                let mut typedef = aster::AstBuilder::new().item().pub_();
                if ctx.options().generate_comments {
                    if let Some(comment) = item.comment() {
                        typedef = typedef.attr().doc(comment);
                    }
                }
                let simple_enum_path =
                    match inner_rust_type.node {
                        ast::TyKind::Path(None, ref p) => {
                            if used_template_params.is_none() &&
                                   inner_item.expect_type().canonical_type(ctx).is_enum()
                                   &&
                                   p.segments.iter().all(|p|
                                                             p.parameters.is_none())
                               {
                                Some(p.clone())
                            } else { None }
                        }
                        _ => None,
                    };
                let typedef =
                    if let Some(mut p) = simple_enum_path {
                        for ident in
                            top_level_path(ctx, item).into_iter().rev() {
                            p.segments.insert(0,
                                              ast::PathSegment{identifier:
                                                                   ident,
                                                               parameters:
                                                                   None,});
                        }
                        typedef.use_().build(p).as_(rust_name)
                    } else {
                        let mut generics =
                            typedef.type_(rust_name).generics();
                        if let Some(ref params) = used_template_params {
                            for template_param in params {
                                if let Some(id) =
                                       template_param.as_named(ctx, &()) {
                                    let template_param = ctx.resolve_type(id);
                                    if template_param.is_invalid_named_type()
                                       {
                                        warn!("Item contained invalid template \
                                           parameter: {:?}"
                                              , item);
                                        return;
                                    }
                                    generics =
                                        generics.ty_param_id(template_param.name().unwrap());
                                }
                            }
                        }
                        generics.build().build_ty(inner_rust_type)
                    };
                result.push(typedef)
            }
            TypeKind::Enum(ref ei) => {
                ei.codegen(ctx, result, whitelisted_items, item)
            }
            TypeKind::ObjCId | TypeKind::ObjCSel => { result.saw_objc(); }
            TypeKind::ObjCInterface(ref interface) => {
                interface.codegen(ctx, result, whitelisted_items, item)
            }
            ref u@TypeKind::UnresolvedTypeRef(..) => {
                unreachable!("Should have been resolved after parsing {:?}!" ,
                             u)
            }
        }
    }
}
struct Vtable<'a> {
    item_id: ItemId,
    #[allow(dead_code)]
    methods: &'a [Method],
    #[allow(dead_code)]
    base_classes: &'a [Base],
}
impl <'a> Vtable<'a> {
    fn new(item_id: ItemId, methods: &'a [Method], base_classes: &'a [Base])
     -> Self {
        Vtable{item_id: item_id,
               methods: methods,
               base_classes: base_classes,}
    }
}
impl <'a> CodeGenerator for Vtable<'a> {
    type
    Extra
    =
    Item;
    fn codegen<'b>(&self, ctx: &BindgenContext,
                   result: &mut CodegenResult<'b>,
                   _whitelisted_items: &ItemSet, item: &Item) {
        assert_eq!(item . id (  ) , self . item_id);
        let attributes = vec!(attributes :: repr ( "C" ));
        let vtable =
            aster::AstBuilder::new().item().pub_().with_attrs(attributes).tuple_struct(self.canonical_name(ctx)).field().build_ty(helpers::ast_ty::raw_type(ctx,
                                                                                                                                                            "c_void")).build();
        result.push(vtable);
    }
}
impl <'a> ItemCanonicalName for Vtable<'a> {
    fn canonical_name(&self, ctx: &BindgenContext) -> String {
        format!("{}__bindgen_vtable" , self . item_id . canonical_name ( ctx
                ))
    }
}
impl <'a> TryToRustTy for Vtable<'a> {
    type
    Extra
    =
    ();
    fn try_to_rust_ty(&self, ctx: &BindgenContext, _: &())
     -> error::Result<P<ast::Ty>> {
        Ok(aster::ty::TyBuilder::new().id(self.canonical_name(ctx)))
    }
}
struct Bitfield<'a> {
    index: &'a mut usize,
    fields: Vec<&'a Field>,
}
impl <'a> Bitfield<'a> {
    fn new(index: &'a mut usize, fields: Vec<&'a Field>) -> Self {
        Bitfield{index: index, fields: fields,}
    }
    fn codegen_fields(self, ctx: &BindgenContext, parent: &CompInfo,
                      fields: &mut Vec<ast::StructField>,
                      methods: &mut Vec<ast::ImplItem>) -> Layout {
        let mut total_size_in_bits = 0;
        let mut max_align = 0;
        let mut unfilled_bits_in_last_unit = 0;
        let mut field_size_in_bits = 0;
        *self.index += 1;
        let mut last_field_name = format!("_bitfield_{}" , self . index);
        let mut last_field_align = 0;
        let mut bitfields: Vec<(&str, usize, usize, ast::Ty, Layout)> =
            vec!();
        for field in self.fields {
            let width = field.bitfield().unwrap() as usize;
            let field_item = ctx.resolve_item(field.ty());
            let field_ty_layout =
                field_item.kind().expect_type().layout(ctx).expect("Bitfield without layout? Gah!");
            let field_align = field_ty_layout.align;
            if field_size_in_bits != 0 &&
                   (width == 0 || width > unfilled_bits_in_last_unit) {
                field_size_in_bits =
                    align_to(field_size_in_bits, field_align);
                fields.push(flush_bitfields(ctx, parent, field_size_in_bits,
                                            last_field_align,
                                            &last_field_name,
                                            bitfields.drain(..), methods));
                *self.index += 1;
                last_field_name = format!("_bitfield_{}" , self . index);
                field_size_in_bits = 0;
                last_field_align = 0;
            }
            if let Some(name) = field.name() {
                let field_item_ty = field_item.to_rust_ty_or_opaque(ctx, &());
                bitfields.push((name, field_size_in_bits, width,
                                field_item_ty.unwrap(), field_ty_layout));
            }
            field_size_in_bits += width;
            total_size_in_bits += width;
            let data_size = align_to(field_size_in_bits, field_align * 8);
            max_align = cmp::max(max_align, field_align);
            last_field_align = cmp::max(last_field_align, width);
            unfilled_bits_in_last_unit = data_size - field_size_in_bits;
        }
        if field_size_in_bits != 0 {
            fields.push(flush_bitfields(ctx, parent, field_size_in_bits,
                                        last_field_align, &last_field_name,
                                        bitfields.drain(..), methods));
        }
        Layout::new(bytes_from_bits(total_size_in_bits), max_align)
    }
}
fn parent_has_method(ctx: &BindgenContext, parent: &CompInfo, name: &str)
 -> bool {
    parent.methods().iter().any(|method|
                                    {
                                        let method_name =
                                            match *ctx.resolve_item(method.signature()).kind()
                                                {
                                                ItemKind::Function(ref func)
                                                => func.name(),
                                                ref otherwise =>
                                                panic!("a method's signature should always be a \
                                     item of kind ItemKind::Function, found: \
                                     {:?}"
                                                       , otherwise),
                                            };
                                        method_name == name ||
                                            ctx.rust_mangle(&method_name) ==
                                                name
                                    })
}
fn bitfield_getter_name(ctx: &BindgenContext, parent: &CompInfo,
                        bitfield_name: &str) -> ast::Ident {
    let name = ctx.rust_mangle(bitfield_name);
    if parent_has_method(ctx, parent, &name) {
        let mut name = name.to_string();
        name.push_str("_bindgen_bitfield");
        return ctx.ext_cx().ident_of(&name);
    }
    ctx.ext_cx().ident_of(&name)
}
fn bitfield_setter_name(ctx: &BindgenContext, parent: &CompInfo,
                        bitfield_name: &str) -> ast::Ident {
    let setter = format!("set_{}" , bitfield_name);
    let mut setter = ctx.rust_mangle(&setter).to_string();
    if parent_has_method(ctx, parent, &setter) {
        setter.push_str("_bindgen_bitfield");
    }
    ctx.ext_cx().ident_of(&setter)
}
/// A physical field (which is a word or byte or ...) has many logical bitfields
/// contained within it, but not all bitfields are in the same physical field of
/// a struct. This function creates a single physical field and flushes all the
/// accessors for the logical `bitfields` within that physical field to the
/// outgoing `methods`.
fn flush_bitfields<'a,
                   I>(ctx: &BindgenContext, parent: &CompInfo,
                      field_size_in_bits: usize, field_align: usize,
                      field_name: &str, bitfields: I,
                      methods: &mut Vec<ast::ImplItem>) -> ast::StructField
 where I: IntoIterator<Item = (&'a str, usize, usize, ast::Ty, Layout)> {
    use aster::struct_field::StructFieldBuilder;
    let field_layout =
        Layout::new(bytes_from_bits_pow2(field_size_in_bits),
                    bytes_from_bits_pow2(field_align));
    let field_ty = BlobTyBuilder::new(field_layout).build();
    let field =
        StructFieldBuilder::named(field_name).pub_().build_ty(field_ty.clone());
    let field_int_ty =
        match field_layout.size {
            8 => {
                let ext_cx = &*ctx.ext_cx();
                ::quasi::parse_ty_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                  {
                                                                                      #[allow(unused_imports)]
                                                                                      use quasi::IntoWrappedIterator;
                                                                                      #[allow(unused_imports)]
                                                                                      use quasi::IntoWrappedRepeat;
                                                                                      let _sp =
                                                                                          ext_cx.call_site();
                                                                                      let mut tt =
                                                                                          ::std::vec::Vec::new();
                                                                                      tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                      ::syntax::parse::token::Ident(ext_cx.ident_of("u64"))));
                                                                                      tt
                                                                                  }))
            }
            4 => {
                let ext_cx = &*ctx.ext_cx();
                ::quasi::parse_ty_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                  {
                                                                                      #[allow(unused_imports)]
                                                                                      use quasi::IntoWrappedIterator;
                                                                                      #[allow(unused_imports)]
                                                                                      use quasi::IntoWrappedRepeat;
                                                                                      let _sp =
                                                                                          ext_cx.call_site();
                                                                                      let mut tt =
                                                                                          ::std::vec::Vec::new();
                                                                                      tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                      ::syntax::parse::token::Ident(ext_cx.ident_of("u32"))));
                                                                                      tt
                                                                                  }))
            }
            2 => {
                let ext_cx = &*ctx.ext_cx();
                ::quasi::parse_ty_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                  {
                                                                                      #[allow(unused_imports)]
                                                                                      use quasi::IntoWrappedIterator;
                                                                                      #[allow(unused_imports)]
                                                                                      use quasi::IntoWrappedRepeat;
                                                                                      let _sp =
                                                                                          ext_cx.call_site();
                                                                                      let mut tt =
                                                                                          ::std::vec::Vec::new();
                                                                                      tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                      ::syntax::parse::token::Ident(ext_cx.ident_of("u16"))));
                                                                                      tt
                                                                                  }))
            }
            1 => {
                let ext_cx = &*ctx.ext_cx();
                ::quasi::parse_ty_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                  {
                                                                                      #[allow(unused_imports)]
                                                                                      use quasi::IntoWrappedIterator;
                                                                                      #[allow(unused_imports)]
                                                                                      use quasi::IntoWrappedRepeat;
                                                                                      let _sp =
                                                                                          ext_cx.call_site();
                                                                                      let mut tt =
                                                                                          ::std::vec::Vec::new();
                                                                                      tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                      ::syntax::parse::token::Ident(ext_cx.ident_of("u8"))));
                                                                                      tt
                                                                                  }))
            }
            _ => return field,
        };
    for (name, offset, width, bitfield_ty, bitfield_layout) in bitfields {
        let prefix = ctx.trait_prefix();
        let getter_name = bitfield_getter_name(ctx, parent, name);
        let setter_name = bitfield_setter_name(ctx, parent, name);
        let field_ident = ctx.ext_cx().ident_of(field_name);
        let bitfield_int_ty = BlobTyBuilder::new(bitfield_layout).build();
        let mask: usize = ((1usize << width) - 1usize) << offset;
        let impl_item =
            {
                let ext_cx = &*ctx.ext_cx();
                ::quasi::parse_item_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                    {
                                                                                        #[allow(unused_imports)]
                                                                                        use quasi::IntoWrappedIterator;
                                                                                        #[allow(unused_imports)]
                                                                                        use quasi::IntoWrappedRepeat;
                                                                                        let _sp =
                                                                                            ext_cx.call_site();
                                                                                        let mut tt =
                                                                                            ::std::vec::Vec::new();
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("impl"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("XxxIgnored"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Pound));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("inline"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("pub"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("fn"))));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&getter_name,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("self"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::RArrow));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&bitfield_ty,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("let"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("mask"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Eq));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&mask,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("as"))));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&field_int_ty,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Semi));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("let"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("field_val"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Colon));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&field_int_ty,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Eq));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("unsafe"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&prefix,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("mem"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("transmute"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("self"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Dot));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&field_ident,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Semi));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("let"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("val"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Eq));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("field_val"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("mask"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::BinOp(::syntax::parse::token::Shr)));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&offset,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Semi));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("unsafe"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&prefix,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("mem"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("transmute"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("val"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("as"))));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&bitfield_int_ty,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Pound));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("inline"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("pub"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("fn"))));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&setter_name,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("mut"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("self"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Comma));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("val"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Colon));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&bitfield_ty,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("let"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("mask"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Eq));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&mask,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("as"))));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&field_int_ty,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Semi));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("let"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("val"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Eq));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("val"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("as"))));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&bitfield_int_ty,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("as"))));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&field_int_ty,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Semi));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("let"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("mut"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("field_val"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Colon));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&field_int_ty,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Eq));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("unsafe"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&prefix,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("mem"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("transmute"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("self"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Dot));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&field_ident,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Semi));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("field_val"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::BinOpEq(::syntax::parse::token::And)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Not));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("mask"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Semi));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("field_val"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::BinOpEq(::syntax::parse::token::Or)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("val"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::BinOp(::syntax::parse::token::Shl)));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&offset,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("mask"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Semi));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("self"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Dot));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&field_ident,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Eq));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("unsafe"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&prefix,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("mem"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("transmute"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("field_val"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Semi));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                        tt
                                                                                    }))
            }.unwrap();
        match impl_item.unwrap().node {
            ast::ItemKind::Impl(_, _, _, _, _, items) => {
                methods.extend(items.into_iter());
            }
            _ => unreachable!(),
        };
    }
    field
}
impl CodeGenerator for TemplateInstantiation {
    type
    Extra
    =
    Item;
    fn codegen<'a>(&self, ctx: &BindgenContext,
                   result: &mut CodegenResult<'a>,
                   _whitelisted_items: &ItemSet, item: &Item) {
        if !ctx.options().layout_tests { return }
        let layout = item.kind().expect_type().layout(ctx);
        if let Some(layout) = layout {
            let size = layout.size;
            let align = layout.align;
            let name = item.canonical_name(ctx);
            let fn_name =
                format!("__bindgen_test_layout_{}_instantiation_{}" , name ,
                        item . id (  ) . as_usize (  ));
            let fn_name = ctx.rust_ident_raw(&fn_name);
            let prefix = ctx.trait_prefix();
            let ident = item.to_rust_ty_or_opaque(ctx, &());
            let size_of_expr =
                {
                    let ext_cx = &*ctx.ext_cx();
                    ::quasi::parse_expr_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                        {
                                                                                            #[allow(unused_imports)]
                                                                                            use quasi::IntoWrappedIterator;
                                                                                            #[allow(unused_imports)]
                                                                                            use quasi::IntoWrappedRepeat;
                                                                                            let _sp =
                                                                                                ext_cx.call_site();
                                                                                            let mut tt =
                                                                                                ::std::vec::Vec::new();
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::ModSep));
                                                                                            tt.extend(::quasi::ToTokens::to_tokens(&prefix,
                                                                                                                                   ext_cx).into_iter());
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::ModSep));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("mem"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::ModSep));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("size_of"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::ModSep));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Lt));
                                                                                            tt.extend(::quasi::ToTokens::to_tokens(&ident,
                                                                                                                                   ext_cx).into_iter());
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Gt));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                            tt
                                                                                        }))
                };
            let align_of_expr =
                {
                    let ext_cx = &*ctx.ext_cx();
                    ::quasi::parse_expr_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                        {
                                                                                            #[allow(unused_imports)]
                                                                                            use quasi::IntoWrappedIterator;
                                                                                            #[allow(unused_imports)]
                                                                                            use quasi::IntoWrappedRepeat;
                                                                                            let _sp =
                                                                                                ext_cx.call_site();
                                                                                            let mut tt =
                                                                                                ::std::vec::Vec::new();
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::ModSep));
                                                                                            tt.extend(::quasi::ToTokens::to_tokens(&prefix,
                                                                                                                                   ext_cx).into_iter());
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::ModSep));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("mem"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::ModSep));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("align_of"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::ModSep));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Lt));
                                                                                            tt.extend(::quasi::ToTokens::to_tokens(&ident,
                                                                                                                                   ext_cx).into_iter());
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Gt));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                            tt
                                                                                        }))
                };
            let item =
                {
                    let ext_cx = &*ctx.ext_cx();
                    ::quasi::parse_item_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                        {
                                                                                            #[allow(unused_imports)]
                                                                                            use quasi::IntoWrappedIterator;
                                                                                            #[allow(unused_imports)]
                                                                                            use quasi::IntoWrappedRepeat;
                                                                                            let _sp =
                                                                                                ext_cx.call_site();
                                                                                            let mut tt =
                                                                                                ::std::vec::Vec::new();
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Pound));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::OpenDelim(::syntax::parse::token::Bracket)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("test"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::CloseDelim(::syntax::parse::token::Bracket)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("fn"))));
                                                                                            tt.extend(::quasi::ToTokens::to_tokens(&fn_name,
                                                                                                                                   ext_cx).into_iter());
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("assert_eq"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Not));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                            tt.extend(::quasi::ToTokens::to_tokens(&size_of_expr,
                                                                                                                                   ext_cx).into_iter());
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Comma));
                                                                                            tt.extend(::quasi::ToTokens::to_tokens(&size,
                                                                                                                                   ext_cx).into_iter());
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Comma));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("concat"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Not));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Literal(::syntax::parse::token::Str_(ext_cx.name_of("Size of template specialization: ")),
                                                                                                                                                                            ::std::option::Option::None)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Comma));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("stringify"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Not));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                            tt.extend(::quasi::ToTokens::to_tokens(&ident,
                                                                                                                                   ext_cx).into_iter());
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Semi));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("assert_eq"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Not));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                            tt.extend(::quasi::ToTokens::to_tokens(&align_of_expr,
                                                                                                                                   ext_cx).into_iter());
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Comma));
                                                                                            tt.extend(::quasi::ToTokens::to_tokens(&align,
                                                                                                                                   ext_cx).into_iter());
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Comma));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("concat"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Not));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Literal(::syntax::parse::token::Str_(ext_cx.name_of("Alignment of template specialization: ")),
                                                                                                                                                                            ::std::option::Option::None)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Comma));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("stringify"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Not));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                            tt.extend(::quasi::ToTokens::to_tokens(&ident,
                                                                                                                                   ext_cx).into_iter());
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Semi));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                            tt
                                                                                        }))
                }.unwrap();
            result.push(item);
        }
    }
}
impl CodeGenerator for CompInfo {
    type
    Extra
    =
    Item;
    fn codegen<'a>(&self, ctx: &BindgenContext,
                   result: &mut CodegenResult<'a>,
                   whitelisted_items: &ItemSet, item: &Item) {
        use aster::struct_field::StructFieldBuilder;
        debug!("<CompInfo as CodeGenerator>::codegen: item = {:?}" , item);
        if self.has_non_type_template_params() { return; }
        let used_template_params = item.used_template_params(ctx);
        if self.is_forward_declaration() && used_template_params.is_none() {
            let struct_name = item.canonical_name(ctx);
            let struct_name = ctx.rust_ident_raw(&struct_name);
            let tuple_struct =
                {
                    let ext_cx = &*ctx.ext_cx();
                    ::quasi::parse_item_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                        {
                                                                                            #[allow(unused_imports)]
                                                                                            use quasi::IntoWrappedIterator;
                                                                                            #[allow(unused_imports)]
                                                                                            use quasi::IntoWrappedRepeat;
                                                                                            let _sp =
                                                                                                ext_cx.call_site();
                                                                                            let mut tt =
                                                                                                ::std::vec::Vec::new();
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Pound));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::OpenDelim(::syntax::parse::token::Bracket)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("repr"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("C"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::CloseDelim(::syntax::parse::token::Bracket)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Pound));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::OpenDelim(::syntax::parse::token::Bracket)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("derive"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("Debug"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Comma));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("Copy"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Comma));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("Clone"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::CloseDelim(::syntax::parse::token::Bracket)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("pub"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("struct"))));
                                                                                            tt.extend(::quasi::ToTokens::to_tokens(&struct_name,
                                                                                                                                   ext_cx).into_iter());
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::OpenDelim(::syntax::parse::token::Bracket)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("u8"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Semi));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Literal(::syntax::parse::token::Integer(ext_cx.name_of("0")),
                                                                                                                                                                            ::std::option::Option::None)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::CloseDelim(::syntax::parse::token::Bracket)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Semi));
                                                                                            tt
                                                                                        }))
                }.unwrap();
            result.push(tuple_struct);
            return;
        }
        let mut attributes = vec!();
        let mut needs_clone_impl = false;
        let mut needs_default_impl = false;
        if ctx.options().generate_comments {
            if let Some(comment) = item.comment() {
                attributes.push(attributes::doc(comment));
            }
        }
        if self.packed() {
            attributes.push(attributes::repr_list(&["C", "packed"]));
        } else { attributes.push(attributes::repr("C")); }
        let is_union = self.kind() == CompKind::Union;
        let mut derives = vec!();
        if item.can_derive_debug(ctx, ()) { derives.push("Debug"); }
        if item.can_derive_default(ctx, ()) {
            derives.push("Default");
        } else { needs_default_impl = ctx.options().derive_default; }
        if item.can_derive_copy(ctx, ()) &&
               !item.annotations().disallow_copy() {
            derives.push("Copy");
            if used_template_params.is_some() {
                derives.push("Clone");
            } else { needs_clone_impl = true; }
        }
        if !derives.is_empty() {
            attributes.push(attributes::derives(&derives))
        }
        let canonical_name = item.canonical_name(ctx);
        let builder =
            if is_union && ctx.options().unstable_rust {
                aster::AstBuilder::new().item().pub_().with_attrs(attributes).union_(&canonical_name)
            } else {
                aster::AstBuilder::new().item().pub_().with_attrs(attributes).struct_(&canonical_name)
            };
        let mut fields = vec!();
        let mut struct_layout = StructLayoutTracker::new(ctx, self);
        if self.needs_explicit_vtable(ctx) {
            let vtable =
                Vtable::new(item.id(), self.methods(), self.base_members());
            vtable.codegen(ctx, result, whitelisted_items, item);
            let vtable_type =
                vtable.try_to_rust_ty(ctx,
                                      &()).expect("vtable to Rust type conversion is infallible").to_ptr(true,
                                                                                                         ctx.span());
            let vtable_field =
                StructFieldBuilder::named("vtable_").pub_().build_ty(vtable_type);
            struct_layout.saw_vtable();
            fields.push(vtable_field);
        }
        for (i, base) in self.base_members().iter().enumerate() {
            if base.is_virtual() { continue ; }
            let base_ty = ctx.resolve_type(base.ty);
            if base_ty.is_unsized(ctx) { continue ; }
            let inner = base.ty.to_rust_ty_or_opaque(ctx, &());
            let field_name =
                if i == 0 { "_base".into() } else { format!("_base_{}" , i) };
            struct_layout.saw_base(base_ty);
            let field =
                StructFieldBuilder::named(field_name).pub_().build_ty(inner);
            fields.push(field);
        }
        if is_union { result.saw_union(); }
        let layout = item.kind().expect_type().layout(ctx);
        let mut current_bitfield_width = None;
        let mut current_bitfield_layout: Option<Layout> = None;
        let mut current_bitfield_fields = vec!();
        let mut bitfield_count = 0;
        let struct_fields = self.fields();
        let fields_should_be_private =
            item.annotations().private_fields().unwrap_or(false);
        let struct_accessor_kind =
            item.annotations().accessor_kind().unwrap_or(FieldAccessorKind::None);
        let mut methods = vec!();
        let mut anonymous_field_count = 0;
        for field in struct_fields {
            debug_assert_eq!(current_bitfield_width . is_some (  ) ,
                             current_bitfield_layout . is_some (  ));
            debug_assert_eq!(current_bitfield_width . is_some (  ) , !
                             current_bitfield_fields . is_empty (  ));
            let field_ty = ctx.resolve_type(field.ty());
            if let (Some(ref mut bitfield_width), Some(width)) =
                   (current_bitfield_width, field.bitfield()) {
                let layout = current_bitfield_layout.unwrap();
                debug!("Testing bitfield continuation {} {} {:?}" , *
                       bitfield_width , width , layout);
                if *bitfield_width + width <= ((layout.size * 8) as u32) {
                    *bitfield_width += width;
                    current_bitfield_fields.push(field);
                    continue ;
                }
            }
            if current_bitfield_width.is_some() {
                debug_assert!(! current_bitfield_fields . is_empty (  ));
                let bitfield_fields =
                    mem::replace(&mut current_bitfield_fields, vec!());
                let bitfield_layout =
                    Bitfield::new(&mut bitfield_count,
                                  bitfield_fields).codegen_fields(ctx, self,
                                                                  &mut fields,
                                                                  &mut methods);
                struct_layout.saw_bitfield_batch(bitfield_layout);
                current_bitfield_width = None;
                current_bitfield_layout = None;
            }
            debug_assert!(current_bitfield_fields . is_empty (  ));
            if let Some(width) = field.bitfield() {
                let layout =
                    field_ty.layout(ctx).expect("Bitfield type without layout?");
                current_bitfield_width = Some(width);
                current_bitfield_layout = Some(layout);
                current_bitfield_fields.push(field);
                continue ;
            }
            let ty = field.ty().to_rust_ty_or_opaque(ctx, &());
            let ty =
                if is_union && !ctx.options().unstable_rust {
                    if ctx.options().enable_cxx_namespaces {
                        {
                            let ext_cx = &*ctx.ext_cx();
                            ::quasi::parse_ty_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                              {
                                                                                                  #[allow(unused_imports)]
                                                                                                  use quasi::IntoWrappedIterator;
                                                                                                  #[allow(unused_imports)]
                                                                                                  use quasi::IntoWrappedRepeat;
                                                                                                  let _sp =
                                                                                                      ext_cx.call_site();
                                                                                                  let mut tt =
                                                                                                      ::std::vec::Vec::new();
                                                                                                  tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                  ::syntax::parse::token::Ident(ext_cx.ident_of("root"))));
                                                                                                  tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                  ::syntax::parse::token::ModSep));
                                                                                                  tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                  ::syntax::parse::token::Ident(ext_cx.ident_of("__BindgenUnionField"))));
                                                                                                  tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                  ::syntax::parse::token::Lt));
                                                                                                  tt.extend(::quasi::ToTokens::to_tokens(&ty,
                                                                                                                                         ext_cx).into_iter());
                                                                                                  tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                  ::syntax::parse::token::Gt));
                                                                                                  tt
                                                                                              }))
                        }
                    } else {
                        {
                            let ext_cx = &*ctx.ext_cx();
                            ::quasi::parse_ty_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                              {
                                                                                                  #[allow(unused_imports)]
                                                                                                  use quasi::IntoWrappedIterator;
                                                                                                  #[allow(unused_imports)]
                                                                                                  use quasi::IntoWrappedRepeat;
                                                                                                  let _sp =
                                                                                                      ext_cx.call_site();
                                                                                                  let mut tt =
                                                                                                      ::std::vec::Vec::new();
                                                                                                  tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                  ::syntax::parse::token::Ident(ext_cx.ident_of("__BindgenUnionField"))));
                                                                                                  tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                  ::syntax::parse::token::Lt));
                                                                                                  tt.extend(::quasi::ToTokens::to_tokens(&ty,
                                                                                                                                         ext_cx).into_iter());
                                                                                                  tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                  ::syntax::parse::token::Gt));
                                                                                                  tt
                                                                                              }))
                        }
                    }
                } else if let Some(item) = field_ty.is_incomplete_array(ctx) {
                    result.saw_incomplete_array();
                    let inner = item.to_rust_ty_or_opaque(ctx, &());
                    if ctx.options().enable_cxx_namespaces {
                        {
                            let ext_cx = &*ctx.ext_cx();
                            ::quasi::parse_ty_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                              {
                                                                                                  #[allow(unused_imports)]
                                                                                                  use quasi::IntoWrappedIterator;
                                                                                                  #[allow(unused_imports)]
                                                                                                  use quasi::IntoWrappedRepeat;
                                                                                                  let _sp =
                                                                                                      ext_cx.call_site();
                                                                                                  let mut tt =
                                                                                                      ::std::vec::Vec::new();
                                                                                                  tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                  ::syntax::parse::token::Ident(ext_cx.ident_of("root"))));
                                                                                                  tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                  ::syntax::parse::token::ModSep));
                                                                                                  tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                  ::syntax::parse::token::Ident(ext_cx.ident_of("__IncompleteArrayField"))));
                                                                                                  tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                  ::syntax::parse::token::Lt));
                                                                                                  tt.extend(::quasi::ToTokens::to_tokens(&inner,
                                                                                                                                         ext_cx).into_iter());
                                                                                                  tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                  ::syntax::parse::token::Gt));
                                                                                                  tt
                                                                                              }))
                        }
                    } else {
                        {
                            let ext_cx = &*ctx.ext_cx();
                            ::quasi::parse_ty_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                              {
                                                                                                  #[allow(unused_imports)]
                                                                                                  use quasi::IntoWrappedIterator;
                                                                                                  #[allow(unused_imports)]
                                                                                                  use quasi::IntoWrappedRepeat;
                                                                                                  let _sp =
                                                                                                      ext_cx.call_site();
                                                                                                  let mut tt =
                                                                                                      ::std::vec::Vec::new();
                                                                                                  tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                  ::syntax::parse::token::Ident(ext_cx.ident_of("__IncompleteArrayField"))));
                                                                                                  tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                  ::syntax::parse::token::Lt));
                                                                                                  tt.extend(::quasi::ToTokens::to_tokens(&inner,
                                                                                                                                         ext_cx).into_iter());
                                                                                                  tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                  ::syntax::parse::token::Gt));
                                                                                                  tt
                                                                                              }))
                        }
                    }
                } else { ty };
            let mut attrs = vec!();
            if ctx.options().generate_comments {
                if let Some(comment) = field.comment() {
                    attrs.push(attributes::doc(comment));
                }
            }
            let field_name =
                match field.name() {
                    Some(name) => ctx.rust_mangle(name).into_owned(),
                    None => {
                        anonymous_field_count += 1;
                        format!("__bindgen_anon_{}" , anonymous_field_count)
                    }
                };
            if !is_union {
                if let Some(padding_field) =
                       struct_layout.pad_field(&field_name, field_ty,
                                               field.offset()) {
                    fields.push(padding_field);
                }
            }
            let is_private =
                field.annotations().private_fields().unwrap_or(fields_should_be_private);
            let accessor_kind =
                field.annotations().accessor_kind().unwrap_or(struct_accessor_kind);
            let mut field = StructFieldBuilder::named(&field_name);
            if !is_private { field = field.pub_(); }
            let field = field.with_attrs(attrs).build_ty(ty.clone());
            fields.push(field);
            if accessor_kind == FieldAccessorKind::None { continue ; }
            let getter_name =
                ctx.rust_ident_raw(&format!("get_{}" , field_name));
            let mutable_getter_name =
                ctx.rust_ident_raw(&format!("get_{}_mut" , field_name));
            let field_name = ctx.rust_ident_raw(&field_name);
            let accessor_methods_impl =
                match accessor_kind {
                    FieldAccessorKind::None => unreachable!(),
                    FieldAccessorKind::Regular => {
                        {
                            let ext_cx = &*ctx.ext_cx();
                            ::quasi::parse_item_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                                {
                                                                                                    #[allow(unused_imports)]
                                                                                                    use quasi::IntoWrappedIterator;
                                                                                                    #[allow(unused_imports)]
                                                                                                    use quasi::IntoWrappedRepeat;
                                                                                                    let _sp =
                                                                                                        ext_cx.call_site();
                                                                                                    let mut tt =
                                                                                                        ::std::vec::Vec::new();
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("impl"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("X"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Pound));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::OpenDelim(::syntax::parse::token::Bracket)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("inline"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::CloseDelim(::syntax::parse::token::Bracket)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("pub"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("fn"))));
                                                                                                    tt.extend(::quasi::ToTokens::to_tokens(&getter_name,
                                                                                                                                           ext_cx).into_iter());
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("self"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::RArrow));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                                    tt.extend(::quasi::ToTokens::to_tokens(&ty,
                                                                                                                                           ext_cx).into_iter());
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("self"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Dot));
                                                                                                    tt.extend(::quasi::ToTokens::to_tokens(&field_name,
                                                                                                                                           ext_cx).into_iter());
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Pound));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::OpenDelim(::syntax::parse::token::Bracket)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("inline"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::CloseDelim(::syntax::parse::token::Bracket)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("pub"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("fn"))));
                                                                                                    tt.extend(::quasi::ToTokens::to_tokens(&mutable_getter_name,
                                                                                                                                           ext_cx).into_iter());
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("mut"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("self"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::RArrow));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("mut"))));
                                                                                                    tt.extend(::quasi::ToTokens::to_tokens(&ty,
                                                                                                                                           ext_cx).into_iter());
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("mut"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("self"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Dot));
                                                                                                    tt.extend(::quasi::ToTokens::to_tokens(&field_name,
                                                                                                                                           ext_cx).into_iter());
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                                    tt
                                                                                                }))
                        }
                    }
                    FieldAccessorKind::Unsafe => {
                        {
                            let ext_cx = &*ctx.ext_cx();
                            ::quasi::parse_item_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                                {
                                                                                                    #[allow(unused_imports)]
                                                                                                    use quasi::IntoWrappedIterator;
                                                                                                    #[allow(unused_imports)]
                                                                                                    use quasi::IntoWrappedRepeat;
                                                                                                    let _sp =
                                                                                                        ext_cx.call_site();
                                                                                                    let mut tt =
                                                                                                        ::std::vec::Vec::new();
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("impl"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("X"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Pound));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::OpenDelim(::syntax::parse::token::Bracket)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("inline"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::CloseDelim(::syntax::parse::token::Bracket)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("pub"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("unsafe"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("fn"))));
                                                                                                    tt.extend(::quasi::ToTokens::to_tokens(&getter_name,
                                                                                                                                           ext_cx).into_iter());
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("self"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::RArrow));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                                    tt.extend(::quasi::ToTokens::to_tokens(&ty,
                                                                                                                                           ext_cx).into_iter());
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("self"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Dot));
                                                                                                    tt.extend(::quasi::ToTokens::to_tokens(&field_name,
                                                                                                                                           ext_cx).into_iter());
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Pound));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::OpenDelim(::syntax::parse::token::Bracket)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("inline"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::CloseDelim(::syntax::parse::token::Bracket)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("pub"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("unsafe"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("fn"))));
                                                                                                    tt.extend(::quasi::ToTokens::to_tokens(&mutable_getter_name,
                                                                                                                                           ext_cx).into_iter());
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("mut"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("self"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::RArrow));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("mut"))));
                                                                                                    tt.extend(::quasi::ToTokens::to_tokens(&ty,
                                                                                                                                           ext_cx).into_iter());
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("mut"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("self"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Dot));
                                                                                                    tt.extend(::quasi::ToTokens::to_tokens(&field_name,
                                                                                                                                           ext_cx).into_iter());
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                                    tt
                                                                                                }))
                        }
                    }
                    FieldAccessorKind::Immutable => {
                        {
                            let ext_cx = &*ctx.ext_cx();
                            ::quasi::parse_item_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                                {
                                                                                                    #[allow(unused_imports)]
                                                                                                    use quasi::IntoWrappedIterator;
                                                                                                    #[allow(unused_imports)]
                                                                                                    use quasi::IntoWrappedRepeat;
                                                                                                    let _sp =
                                                                                                        ext_cx.call_site();
                                                                                                    let mut tt =
                                                                                                        ::std::vec::Vec::new();
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("impl"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("X"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Pound));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::OpenDelim(::syntax::parse::token::Bracket)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("inline"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::CloseDelim(::syntax::parse::token::Bracket)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("pub"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("fn"))));
                                                                                                    tt.extend(::quasi::ToTokens::to_tokens(&getter_name,
                                                                                                                                           ext_cx).into_iter());
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("self"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::RArrow));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                                    tt.extend(::quasi::ToTokens::to_tokens(&ty,
                                                                                                                                           ext_cx).into_iter());
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("self"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Dot));
                                                                                                    tt.extend(::quasi::ToTokens::to_tokens(&field_name,
                                                                                                                                           ext_cx).into_iter());
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                                    tt
                                                                                                }))
                        }
                    }
                };
            match accessor_methods_impl.unwrap().node {
                ast::ItemKind::Impl(_, _, _, _, _, ref items) => {
                    methods.extend(items.clone())
                }
                _ => unreachable!(),
            }
        }
        if current_bitfield_width.is_some() {
            debug_assert!(! current_bitfield_fields . is_empty (  ));
            let bitfield_fields =
                mem::replace(&mut current_bitfield_fields, vec!());
            let bitfield_layout =
                Bitfield::new(&mut bitfield_count,
                              bitfield_fields).codegen_fields(ctx, self,
                                                              &mut fields,
                                                              &mut methods);
            struct_layout.saw_bitfield_batch(bitfield_layout);
        }
        debug_assert!(current_bitfield_fields . is_empty (  ));
        if is_union && !ctx.options().unstable_rust {
            let layout = layout.expect("Unable to get layout information?");
            let ty = BlobTyBuilder::new(layout).build();
            let field =
                StructFieldBuilder::named("bindgen_union_field").pub_().build_ty(ty);
            struct_layout.saw_union(layout);
            fields.push(field);
        }
        if item.is_opaque(ctx) {
            fields.clear();
            methods.clear();
            match layout {
                Some(l) => {
                    let ty = BlobTyBuilder::new(l).build();
                    let field =
                        StructFieldBuilder::named("_bindgen_opaque_blob").pub_().build_ty(ty);
                    fields.push(field);
                }
                None => {
                    warn!("Opaque type without layout! Expect dragons!");
                }
            }
        } else if !is_union && !self.is_unsized(ctx) {
            if let Some(padding_field) =
                   layout.and_then(|layout|
                                       {
                                           struct_layout.pad_struct(&canonical_name,
                                                                    layout)
                                       }) {
                fields.push(padding_field);
            }
            if let Some(align_field) =
                   layout.and_then(|layout|
                                       struct_layout.align_struct(layout)) {
                fields.push(align_field);
            }
        }
        if self.is_unsized(ctx) {
            let has_address = layout.map_or(true, |l| l.size != 0);
            if has_address {
                let ty = BlobTyBuilder::new(Layout::new(1, 1)).build();
                let field =
                    StructFieldBuilder::named("_address").pub_().build_ty(ty);
                fields.push(field);
            }
        }
        let mut generics = aster::AstBuilder::new().generics();
        if let Some(ref params) = used_template_params {
            for ty in params.iter() {
                let param = ctx.resolve_type(*ty);
                let name = param.name().unwrap();
                let ident = ctx.rust_ident(name);
                generics = generics.ty_param_id(ident);
            }
        }
        let generics = generics.build();
        let rust_struct =
            builder.with_generics(generics.clone()).with_fields(fields).build();
        result.push(rust_struct);
        for ty in self.inner_types() {
            let child_item = ctx.resolve_item(*ty);
            child_item.codegen(ctx, result, whitelisted_items, &());
        }
        if self.found_unknown_attr() {
            warn!("Type {} has an unkown attribute that may affect layout" ,
                  canonical_name);
        }
        if used_template_params.is_none() {
            for var in self.inner_vars() {
                ctx.resolve_item(*var).codegen(ctx, result, whitelisted_items,
                                               &());
            }
            if ctx.options().layout_tests {
                if let Some(layout) = layout {
                    let fn_name =
                        format!("bindgen_test_layout_{}" , canonical_name);
                    let fn_name = ctx.rust_ident_raw(&fn_name);
                    let type_name = ctx.rust_ident_raw(&canonical_name);
                    let prefix = ctx.trait_prefix();
                    let size_of_expr =
                        {
                            let ext_cx = &*ctx.ext_cx();
                            ::quasi::parse_expr_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                                {
                                                                                                    #[allow(unused_imports)]
                                                                                                    use quasi::IntoWrappedIterator;
                                                                                                    #[allow(unused_imports)]
                                                                                                    use quasi::IntoWrappedRepeat;
                                                                                                    let _sp =
                                                                                                        ext_cx.call_site();
                                                                                                    let mut tt =
                                                                                                        ::std::vec::Vec::new();
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::ModSep));
                                                                                                    tt.extend(::quasi::ToTokens::to_tokens(&prefix,
                                                                                                                                           ext_cx).into_iter());
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::ModSep));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("mem"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::ModSep));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("size_of"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::ModSep));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Lt));
                                                                                                    tt.extend(::quasi::ToTokens::to_tokens(&type_name,
                                                                                                                                           ext_cx).into_iter());
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Gt));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                                    tt
                                                                                                }))
                        };
                    let align_of_expr =
                        {
                            let ext_cx = &*ctx.ext_cx();
                            ::quasi::parse_expr_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                                {
                                                                                                    #[allow(unused_imports)]
                                                                                                    use quasi::IntoWrappedIterator;
                                                                                                    #[allow(unused_imports)]
                                                                                                    use quasi::IntoWrappedRepeat;
                                                                                                    let _sp =
                                                                                                        ext_cx.call_site();
                                                                                                    let mut tt =
                                                                                                        ::std::vec::Vec::new();
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::ModSep));
                                                                                                    tt.extend(::quasi::ToTokens::to_tokens(&prefix,
                                                                                                                                           ext_cx).into_iter());
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::ModSep));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("mem"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::ModSep));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("align_of"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::ModSep));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Lt));
                                                                                                    tt.extend(::quasi::ToTokens::to_tokens(&type_name,
                                                                                                                                           ext_cx).into_iter());
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Gt));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                                    tt
                                                                                                }))
                        };
                    let size = layout.size;
                    let align = layout.align;
                    let check_struct_align =
                        if align > mem::size_of::<*mut ()>() {
                            None
                        } else {
                            {
                                let ext_cx = &*ctx.ext_cx();
                                ::quasi::parse_item_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                                    {
                                                                                                        #[allow(unused_imports)]
                                                                                                        use quasi::IntoWrappedIterator;
                                                                                                        #[allow(unused_imports)]
                                                                                                        use quasi::IntoWrappedRepeat;
                                                                                                        let _sp =
                                                                                                            ext_cx.call_site();
                                                                                                        let mut tt =
                                                                                                            ::std::vec::Vec::new();
                                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("assert_eq"))));
                                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                        ::syntax::parse::token::Not));
                                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&align_of_expr,
                                                                                                                                               ext_cx).into_iter());
                                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                        ::syntax::parse::token::Comma));
                                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&align,
                                                                                                                                               ext_cx).into_iter());
                                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                        ::syntax::parse::token::Comma));
                                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("concat"))));
                                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                        ::syntax::parse::token::Not));
                                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                        ::syntax::parse::token::Literal(::syntax::parse::token::Str_(ext_cx.name_of("Alignment of ")),
                                                                                                                                                                                        ::std::option::Option::None)));
                                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                        ::syntax::parse::token::Comma));
                                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("stringify"))));
                                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                        ::syntax::parse::token::Not));
                                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&type_name,
                                                                                                                                               ext_cx).into_iter());
                                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                        ::syntax::parse::token::Semi));
                                                                                                        tt
                                                                                                    }))
                            }
                        };
                    let too_many_base_vtables =
                        self.base_members().iter().filter(|base|
                                                              {
                                                                  ctx.resolve_type(base.ty).has_vtable(ctx)
                                                              }).count() > 1;
                    let should_skip_field_offset_checks =
                        item.is_opaque(ctx) || too_many_base_vtables;
                    let check_field_offset =
                        if should_skip_field_offset_checks {
                            None
                        } else {
                            let asserts =
                                self.fields().iter().filter(|field|
                                                                field.bitfield().is_none()).flat_map(|field|
                                                                                                         {
                                                                                                             field.name().and_then(|name|
                                                                                                                                       {
                                                                                                                                           field.offset().and_then(|offset|
                                                                                                                                                                       {
                                                                                                                                                                           let field_offset =
                                                                                                                                                                               offset
                                                                                                                                                                                   /
                                                                                                                                                                                   8;
                                                                                                                                                                           let field_name =
                                                                                                                                                                               ctx.rust_ident(name);
                                                                                                                                                                           {
                                                                                                                                                                               let ext_cx =
                                                                                                                                                                                   &*ctx.ext_cx();
                                                                                                                                                                               ::quasi::parse_item_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                                                                                                                                                                                   {
                                                                                                                                                                                                                                                       #[allow(unused_imports)]
                                                                                                                                                                                                                                                       use quasi::IntoWrappedIterator;
                                                                                                                                                                                                                                                       #[allow(unused_imports)]
                                                                                                                                                                                                                                                       use quasi::IntoWrappedRepeat;
                                                                                                                                                                                                                                                       let _sp =
                                                                                                                                                                                                                                                           ext_cx.call_site();
                                                                                                                                                                                                                                                       let mut tt =
                                                                                                                                                                                                                                                           ::std::vec::Vec::new();
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::Ident(ext_cx.ident_of("assert_eq"))));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::Not));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::Ident(ext_cx.ident_of("unsafe"))));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::BinOp(::syntax::parse::token::Star)));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::Literal(::syntax::parse::token::Integer(ext_cx.name_of("0")),
                                                                                                                                                                                                                                                                                                                                       ::std::option::Option::None)));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::Ident(ext_cx.ident_of("as"))));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::BinOp(::syntax::parse::token::Star)));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::Ident(ext_cx.ident_of("const"))));
                                                                                                                                                                                                                                                       tt.extend(::quasi::ToTokens::to_tokens(&type_name,
                                                                                                                                                                                                                                                                                              ext_cx).into_iter());
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::Dot));
                                                                                                                                                                                                                                                       tt.extend(::quasi::ToTokens::to_tokens(&field_name,
                                                                                                                                                                                                                                                                                              ext_cx).into_iter());
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::Ident(ext_cx.ident_of("as"))));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::BinOp(::syntax::parse::token::Star)));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::Ident(ext_cx.ident_of("const"))));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::Underscore));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::Ident(ext_cx.ident_of("as"))));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::Ident(ext_cx.ident_of("usize"))));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::Comma));
                                                                                                                                                                                                                                                       tt.extend(::quasi::ToTokens::to_tokens(&field_offset,
                                                                                                                                                                                                                                                                                              ext_cx).into_iter());
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::Comma));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::Ident(ext_cx.ident_of("concat"))));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::Not));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::Literal(::syntax::parse::token::Str_(ext_cx.name_of("Alignment of field: ")),
                                                                                                                                                                                                                                                                                                                                       ::std::option::Option::None)));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::Comma));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::Ident(ext_cx.ident_of("stringify"))));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::Not));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                                                                                                                                                                                       tt.extend(::quasi::ToTokens::to_tokens(&type_name,
                                                                                                                                                                                                                                                                                              ext_cx).into_iter());
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::Comma));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::Literal(::syntax::parse::token::Str_(ext_cx.name_of("::")),
                                                                                                                                                                                                                                                                                                                                       ::std::option::Option::None)));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::Comma));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::Ident(ext_cx.ident_of("stringify"))));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::Not));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                                                                                                                                                                                       tt.extend(::quasi::ToTokens::to_tokens(&field_name,
                                                                                                                                                                                                                                                                                              ext_cx).into_iter());
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                                                                                                                                                                                       tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                                                                                                                                       ::syntax::parse::token::Semi));
                                                                                                                                                                                                                                                       tt
                                                                                                                                                                                                                                                   }))
                                                                                                                                                                           }
                                                                                                                                                                       })
                                                                                                                                       })
                                                                                                         }).collect::<Vec<P<ast::Item>>>();
                            Some(asserts)
                        };
                    let item =
                        {
                            let ext_cx = &*ctx.ext_cx();
                            ::quasi::parse_item_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                                {
                                                                                                    #[allow(unused_imports)]
                                                                                                    use quasi::IntoWrappedIterator;
                                                                                                    #[allow(unused_imports)]
                                                                                                    use quasi::IntoWrappedRepeat;
                                                                                                    let _sp =
                                                                                                        ext_cx.call_site();
                                                                                                    let mut tt =
                                                                                                        ::std::vec::Vec::new();
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Pound));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::OpenDelim(::syntax::parse::token::Bracket)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("test"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::CloseDelim(::syntax::parse::token::Bracket)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("fn"))));
                                                                                                    tt.extend(::quasi::ToTokens::to_tokens(&fn_name,
                                                                                                                                           ext_cx).into_iter());
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("assert_eq"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Not));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                                    tt.extend(::quasi::ToTokens::to_tokens(&size_of_expr,
                                                                                                                                           ext_cx).into_iter());
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Comma));
                                                                                                    tt.extend(::quasi::ToTokens::to_tokens(&size,
                                                                                                                                           ext_cx).into_iter());
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Comma));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("concat"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Not));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Literal(::syntax::parse::token::Str_(ext_cx.name_of("Size of: ")),
                                                                                                                                                                                    ::std::option::Option::None)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Comma));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Ident(ext_cx.ident_of("stringify"))));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Not));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                                    tt.extend(::quasi::ToTokens::to_tokens(&type_name,
                                                                                                                                           ext_cx).into_iter());
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::Semi));
                                                                                                    tt.extend(::quasi::ToTokens::to_tokens(&check_struct_align,
                                                                                                                                           ext_cx).into_iter());
                                                                                                    tt.extend(::quasi::ToTokens::to_tokens(&check_field_offset,
                                                                                                                                           ext_cx).into_iter());
                                                                                                    tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                    ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                                    tt
                                                                                                }))
                        }.unwrap();
                    result.push(item);
                }
            }
            let mut method_names = Default::default();
            if ctx.options().codegen_config.methods {
                for method in self.methods() {
                    assert!(method . kind (  ) != MethodKind :: Constructor);
                    method.codegen_method(ctx, &mut methods,
                                          &mut method_names, result,
                                          whitelisted_items, self);
                }
            }
            if ctx.options().codegen_config.constructors {
                for sig in self.constructors() {
                    Method::new(MethodKind::Constructor, *sig,
                                false).codegen_method(ctx, &mut methods,
                                                      &mut method_names,
                                                      result,
                                                      whitelisted_items,
                                                      self);
                }
            }
            if ctx.options().codegen_config.destructors {
                if let Some((is_virtual, destructor)) = self.destructor() {
                    let kind =
                        if is_virtual {
                            MethodKind::VirtualDestructor
                        } else { MethodKind::Destructor };
                    Method::new(kind, destructor,
                                false).codegen_method(ctx, &mut methods,
                                                      &mut method_names,
                                                      result,
                                                      whitelisted_items,
                                                      self);
                }
            }
        }
        let ty_for_impl =
            aster::AstBuilder::new().ty().path().segment(&canonical_name).with_generics(generics.clone()).build().build();
        if needs_clone_impl {
            let impl_ =
                {
                    let ext_cx = &*ctx.ext_cx();
                    ::quasi::parse_item_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                        {
                                                                                            #[allow(unused_imports)]
                                                                                            use quasi::IntoWrappedIterator;
                                                                                            #[allow(unused_imports)]
                                                                                            use quasi::IntoWrappedRepeat;
                                                                                            let _sp =
                                                                                                ext_cx.call_site();
                                                                                            let mut tt =
                                                                                                ::std::vec::Vec::new();
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("impl"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("X"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("fn"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("clone"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("self"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::RArrow));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("Self"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::BinOp(::syntax::parse::token::Star)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("self"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                            tt
                                                                                        }))
                };
            let impl_ =
                match impl_.unwrap().node {
                    ast::ItemKind::Impl(_, _, _, _, _, ref items) =>
                    items.clone(),
                    _ => unreachable!(),
                };
            let clone_impl =
                aster::AstBuilder::new().item().impl_().trait_().id("Clone").build().with_generics(generics.clone()).with_items(impl_).build_ty(ty_for_impl.clone());
            result.push(clone_impl);
        }
        if needs_default_impl {
            let prefix = ctx.trait_prefix();
            let impl_ =
                {
                    let ext_cx = &*ctx.ext_cx();
                    ::quasi::parse_item_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                        {
                                                                                            #[allow(unused_imports)]
                                                                                            use quasi::IntoWrappedIterator;
                                                                                            #[allow(unused_imports)]
                                                                                            use quasi::IntoWrappedRepeat;
                                                                                            let _sp =
                                                                                                ext_cx.call_site();
                                                                                            let mut tt =
                                                                                                ::std::vec::Vec::new();
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("impl"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("X"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("fn"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("default"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::RArrow));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("Self"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("unsafe"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::ModSep));
                                                                                            tt.extend(::quasi::ToTokens::to_tokens(&prefix,
                                                                                                                                   ext_cx).into_iter());
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::ModSep));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("mem"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::ModSep));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("zeroed"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                            tt
                                                                                        }))
                };
            let impl_ =
                match impl_.unwrap().node {
                    ast::ItemKind::Impl(_, _, _, _, _, ref items) =>
                    items.clone(),
                    _ => unreachable!(),
                };
            let default_impl =
                aster::AstBuilder::new().item().impl_().trait_().id("Default").build().with_generics(generics.clone()).with_items(impl_).build_ty(ty_for_impl.clone());
            result.push(default_impl);
        }
        if !methods.is_empty() {
            let methods =
                aster::AstBuilder::new().item().impl_().with_generics(generics).with_items(methods).build_ty(ty_for_impl);
            result.push(methods);
        }
    }
}
trait MethodCodegen {
    fn codegen_method<'a>(&self, ctx: &BindgenContext,
                          methods: &mut Vec<ast::ImplItem>,
                          method_names: &mut HashMap<String, usize>,
                          result: &mut CodegenResult<'a>,
                          whitelisted_items: &ItemSet, parent: &CompInfo);
}
impl MethodCodegen for Method {
    fn codegen_method<'a>(&self, ctx: &BindgenContext,
                          methods: &mut Vec<ast::ImplItem>,
                          method_names: &mut HashMap<String, usize>,
                          result: &mut CodegenResult<'a>,
                          whitelisted_items: &ItemSet, _parent: &CompInfo) {
        if self.is_virtual() { return; }
        let function_item = ctx.resolve_item(self.signature());
        function_item.codegen(ctx, result, whitelisted_items, &());
        let function = function_item.expect_function();
        let signature_item = ctx.resolve_item(function.signature());
        let mut name =
            match self.kind() {
                MethodKind::Constructor => "new".into(),
                MethodKind::Destructor => "destruct".into(),
                _ => function.name().to_owned(),
            };
        let signature =
            match *signature_item.expect_type().kind() {
                TypeKind::Function(ref sig) => sig,
                _ => panic!("How in the world?"),
            };
        if signature.is_variadic() { return; }
        let count =
            {
                let mut count = method_names.entry(name.clone()).or_insert(0);
                *count += 1;
                *count - 1
            };
        if count != 0 { name.push_str(&count.to_string()); }
        let function_name = function_item.canonical_name(ctx);
        let mut fndecl =
            utils::rust_fndecl_from_signature(ctx, signature_item).unwrap();
        if !self.is_static() && !self.is_constructor() {
            let mutability =
                if self.is_const() {
                    ast::Mutability::Immutable
                } else { ast::Mutability::Mutable };
            assert!(! fndecl . inputs . is_empty (  ));
            fndecl.inputs[0] =
                ast::Arg{ty:
                             P(ast::Ty{id: ast::DUMMY_NODE_ID,
                                       node:
                                           ast::TyKind::Rptr(None,
                                                             ast::MutTy{ty:
                                                                            P(ast::Ty{id:
                                                                                          ast::DUMMY_NODE_ID,
                                                                                      node:
                                                                                          ast::TyKind::ImplicitSelf,
                                                                                      span:
                                                                                          ctx.span(),}),
                                                                        mutbl:
                                                                            mutability,}),
                                       span: ctx.span(),}),
                         pat:
                             P(ast::Pat{id: ast::DUMMY_NODE_ID,
                                        node:
                                            ast::PatKind::Ident(ast::BindingMode::ByValue(ast::Mutability::Immutable),
                                                                respan(ctx.span(),
                                                                       ctx.ext_cx().ident_of("self")),
                                                                None),
                                        span: ctx.span(),}),
                         id: ast::DUMMY_NODE_ID,};
        }
        if self.is_constructor() {
            fndecl.inputs.remove(0);
            fndecl.output =
                ast::FunctionRetTy::Ty({
                                           let ext_cx = &*ctx.ext_cx();
                                           ::quasi::parse_ty_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                                             {
                                                                                                                 #[allow(unused_imports)]
                                                                                                                 use quasi::IntoWrappedIterator;
                                                                                                                 #[allow(unused_imports)]
                                                                                                                 use quasi::IntoWrappedRepeat;
                                                                                                                 let _sp =
                                                                                                                     ext_cx.call_site();
                                                                                                                 let mut tt =
                                                                                                                     ::std::vec::Vec::new();
                                                                                                                 tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                 ::syntax::parse::token::Ident(ext_cx.ident_of("Self"))));
                                                                                                                 tt
                                                                                                             }))
                                       });
        }
        let sig =
            ast::MethodSig{unsafety: ast::Unsafety::Unsafe,
                           abi: Abi::Rust,
                           decl: P(fndecl),
                           generics: ast::Generics::default(),
                           constness:
                               respan(ctx.span(), ast::Constness::NotConst),};
        let mut exprs =
            helpers::ast_ty::arguments_from_signature(&signature, ctx);
        let mut stmts = vec!();
        if self.is_constructor() {
            let prefix = ctx.trait_prefix();
            let tmp_variable_decl =
                {
                    let ext_cx = &*ctx.ext_cx();
                    ::quasi::parse_stmt_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                        {
                                                                                            #[allow(unused_imports)]
                                                                                            use quasi::IntoWrappedIterator;
                                                                                            #[allow(unused_imports)]
                                                                                            use quasi::IntoWrappedRepeat;
                                                                                            let _sp =
                                                                                                ext_cx.call_site();
                                                                                            let mut tt =
                                                                                                ::std::vec::Vec::new();
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("let"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("mut"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("__bindgen_tmp"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Eq));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::ModSep));
                                                                                            tt.extend(::quasi::ToTokens::to_tokens(&prefix,
                                                                                                                                   ext_cx).into_iter());
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::ModSep));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("mem"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::ModSep));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("uninitialized"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                            tt
                                                                                        }))
                }.unwrap();
            stmts.push(tmp_variable_decl);
            exprs[0] =
                {
                    let ext_cx = &*ctx.ext_cx();
                    ::quasi::parse_expr_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                        {
                                                                                            #[allow(unused_imports)]
                                                                                            use quasi::IntoWrappedIterator;
                                                                                            #[allow(unused_imports)]
                                                                                            use quasi::IntoWrappedRepeat;
                                                                                            let _sp =
                                                                                                ext_cx.call_site();
                                                                                            let mut tt =
                                                                                                ::std::vec::Vec::new();
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("mut"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("__bindgen_tmp"))));
                                                                                            tt
                                                                                        }))
                };
        } else if !self.is_static() {
            assert!(! exprs . is_empty (  ));
            exprs[0] =
                {
                    let ext_cx = &*ctx.ext_cx();
                    ::quasi::parse_expr_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                        {
                                                                                            #[allow(unused_imports)]
                                                                                            use quasi::IntoWrappedIterator;
                                                                                            #[allow(unused_imports)]
                                                                                            use quasi::IntoWrappedRepeat;
                                                                                            let _sp =
                                                                                                ext_cx.call_site();
                                                                                            let mut tt =
                                                                                                ::std::vec::Vec::new();
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("self"))));
                                                                                            tt
                                                                                        }))
                };
        };
        let call =
            aster::expr::ExprBuilder::new().call().id(function_name).with_args(exprs).build();
        stmts.push(ast::Stmt{id: ast::DUMMY_NODE_ID,
                             node: ast::StmtKind::Expr(call),
                             span: ctx.span(),});
        if self.is_constructor() {
            stmts.push({
                           let ext_cx = &*ctx.ext_cx();
                           ::quasi::parse_stmt_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                               {
                                                                                                   #[allow(unused_imports)]
                                                                                                   use quasi::IntoWrappedIterator;
                                                                                                   #[allow(unused_imports)]
                                                                                                   use quasi::IntoWrappedRepeat;
                                                                                                   let _sp =
                                                                                                       ext_cx.call_site();
                                                                                                   let mut tt =
                                                                                                       ::std::vec::Vec::new();
                                                                                                   tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                   ::syntax::parse::token::Ident(ext_cx.ident_of("__bindgen_tmp"))));
                                                                                                   tt
                                                                                               }))
                       }.unwrap());
        }
        let block =
            ast::Block{stmts: stmts,
                       id: ast::DUMMY_NODE_ID,
                       rules: ast::BlockCheckMode::Default,
                       span: ctx.span(),};
        let mut attrs = vec!();
        attrs.push(attributes::inline());
        let item =
            ast::ImplItem{id: ast::DUMMY_NODE_ID,
                          ident: ctx.rust_ident(&name),
                          vis: ast::Visibility::Public,
                          attrs: attrs,
                          node: ast::ImplItemKind::Method(sig, P(block)),
                          defaultness: ast::Defaultness::Final,
                          span: ctx.span(),};
        methods.push(item);
    }
}
/// A helper type to construct enums, either bitfield ones or rust-style ones.
enum EnumBuilder<'a> {
    Rust(aster::item::ItemEnumBuilder<aster::invoke::Identity>),
    Bitfield {
        canonical_name: &'a str,
        aster: P<ast::Item>,
    },
    Consts {
        aster: P<ast::Item>,
    },
}
impl <'a> EnumBuilder<'a> {
    /// Create a new enum given an item builder, a canonical name, a name for
    /// the representation, and whether it should be represented as a rust enum.
    fn new(aster: aster::item::ItemBuilder<aster::invoke::Identity>,
           name: &'a str, repr: P<ast::Ty>, bitfield_like: bool,
           constify: bool) -> Self {
        if bitfield_like {
            EnumBuilder::Bitfield{canonical_name: name,
                                  aster:
                                      aster.tuple_struct(name).field().pub_().build_ty(repr).build(),}
        } else if constify {
            EnumBuilder::Consts{aster: aster.type_(name).build_ty(repr),}
        } else { EnumBuilder::Rust(aster.enum_(name)) }
    }
    /// Add a variant to this enum.
    fn with_variant<'b>(self, ctx: &BindgenContext, variant: &EnumVariant,
                        mangling_prefix: Option<&String>, rust_ty: P<ast::Ty>,
                        result: &mut CodegenResult<'b>) -> Self {
        let variant_name = ctx.rust_mangle(variant.name());
        let expr = aster::AstBuilder::new().expr();
        let expr =
            match variant.val() {
                EnumVariantValue::Signed(v) => helpers::ast_ty::int_expr(v),
                EnumVariantValue::Unsigned(v) => expr.uint(v),
            };
        match self {
            EnumBuilder::Rust(b) => {
                EnumBuilder::Rust(b.with_variant_(ast::Variant_{name:
                                                                    ctx.rust_ident(&*variant_name),
                                                                attrs: vec!(),
                                                                data:
                                                                    ast::VariantData::Unit(ast::DUMMY_NODE_ID),
                                                                disr_expr:
                                                                    Some(expr),}))
            }
            EnumBuilder::Bitfield { canonical_name, .. } => {
                let constant_name =
                    match mangling_prefix {
                        Some(prefix) => {
                            Cow::Owned(format!("{}_{}" , prefix ,
                                               variant_name))
                        }
                        None => variant_name,
                    };
                let constant =
                    aster::AstBuilder::new().item().pub_().const_(&*constant_name).expr().call().id(canonical_name).arg().build(expr).build().build(rust_ty);
                result.push(constant);
                self
            }
            EnumBuilder::Consts { .. } => {
                let constant_name =
                    match mangling_prefix {
                        Some(prefix) => {
                            Cow::Owned(format!("{}_{}" , prefix ,
                                               variant_name))
                        }
                        None => variant_name,
                    };
                let constant =
                    aster::AstBuilder::new().item().pub_().const_(&*constant_name).expr().build(expr).build(rust_ty);
                result.push(constant);
                self
            }
        }
    }
    fn build<'b>(self, ctx: &BindgenContext, rust_ty: P<ast::Ty>,
                 result: &mut CodegenResult<'b>) -> P<ast::Item> {
        match self {
            EnumBuilder::Rust(b) => b.build(),
            EnumBuilder::Bitfield { canonical_name, aster } => {
                let rust_ty_name = ctx.rust_ident_raw(canonical_name);
                let prefix = ctx.trait_prefix();
                let impl_ =
                    {
                        let ext_cx = &*ctx.ext_cx();
                        ::quasi::parse_item_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                            {
                                                                                                #[allow(unused_imports)]
                                                                                                use quasi::IntoWrappedIterator;
                                                                                                #[allow(unused_imports)]
                                                                                                use quasi::IntoWrappedRepeat;
                                                                                                let _sp =
                                                                                                    ext_cx.call_site();
                                                                                                let mut tt =
                                                                                                    ::std::vec::Vec::new();
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::Ident(ext_cx.ident_of("impl"))));
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::ModSep));
                                                                                                tt.extend(::quasi::ToTokens::to_tokens(&prefix,
                                                                                                                                       ext_cx).into_iter());
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::ModSep));
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::Ident(ext_cx.ident_of("ops"))));
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::ModSep));
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::Ident(ext_cx.ident_of("BitOr"))));
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::Lt));
                                                                                                tt.extend(::quasi::ToTokens::to_tokens(&rust_ty,
                                                                                                                                       ext_cx).into_iter());
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::Gt));
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::Ident(ext_cx.ident_of("for"))));
                                                                                                tt.extend(::quasi::ToTokens::to_tokens(&rust_ty,
                                                                                                                                       ext_cx).into_iter());
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::Ident(ext_cx.ident_of("type"))));
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::Ident(ext_cx.ident_of("Output"))));
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::Eq));
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::Ident(ext_cx.ident_of("Self"))));
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::Semi));
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::Pound));
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::OpenDelim(::syntax::parse::token::Bracket)));
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::Ident(ext_cx.ident_of("inline"))));
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::CloseDelim(::syntax::parse::token::Bracket)));
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::Ident(ext_cx.ident_of("fn"))));
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::Ident(ext_cx.ident_of("bitor"))));
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::Ident(ext_cx.ident_of("self"))));
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::Comma));
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::Ident(ext_cx.ident_of("other"))));
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::Colon));
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::Ident(ext_cx.ident_of("Self"))));
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::RArrow));
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::Ident(ext_cx.ident_of("Self"))));
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                                tt.extend(::quasi::ToTokens::to_tokens(&rust_ty_name,
                                                                                                                                       ext_cx).into_iter());
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::Ident(ext_cx.ident_of("self"))));
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::Dot));
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::Literal(::syntax::parse::token::Integer(ext_cx.name_of("0")),
                                                                                                                                                                                ::std::option::Option::None)));
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::BinOp(::syntax::parse::token::Or)));
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::Ident(ext_cx.ident_of("other"))));
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::Dot));
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::Literal(::syntax::parse::token::Integer(ext_cx.name_of("0")),
                                                                                                                                                                                ::std::option::Option::None)));
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                                tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                                tt
                                                                                            }))
                    }.unwrap();
                result.push(impl_);
                aster
            }
            EnumBuilder::Consts { aster, .. } => aster,
        }
    }
}
impl CodeGenerator for Enum {
    type
    Extra
    =
    Item;
    fn codegen<'a>(&self, ctx: &BindgenContext,
                   result: &mut CodegenResult<'a>,
                   _whitelisted_items: &ItemSet, item: &Item) {
        debug!("<Enum as CodeGenerator>::codegen: item = {:?}" , item);
        let name = item.canonical_name(ctx);
        let enum_ty = item.expect_type();
        let layout = enum_ty.layout(ctx);
        let repr = self.repr().map(|repr| ctx.resolve_type(repr));
        let repr =
            match repr {
                Some(repr) => {
                    match *repr.canonical_type(ctx).kind() {
                        TypeKind::Int(int_kind) => int_kind,
                        _ => panic!("Unexpected type as enum repr"),
                    }
                }
                None => {
                    warn!("Guessing type of enum! Forward declarations of enums \
                      shouldn't be legal!");
                    IntKind::Int
                }
            };
        let signed = repr.is_signed();
        let size =
            layout.map(|l| l.size).or_else(|| repr.known_size()).unwrap_or(0);
        let repr_name =
            match (signed, size) {
                (true, 1) => "i8",
                (false, 1) => "u8",
                (true, 2) => "i16",
                (false, 2) => "u16",
                (true, 4) => "i32",
                (false, 4) => "u32",
                (true, 8) => "i64",
                (false, 8) => "u64",
                _ => {
                    warn!("invalid enum decl: signed: {}, size: {}" , signed ,
                          size);
                    "i32"
                }
            };
        let mut builder = aster::AstBuilder::new().item().pub_();
        let is_bitfield =
            {
                ctx.options().bitfield_enums.matches(&name) ||
                    (enum_ty.name().is_none() &&
                         self.variants().iter().any(|v|
                                                        ctx.options().bitfield_enums.matches(&v.name())))
            };
        let is_constified_enum =
            {
                ctx.options().constified_enums.matches(&name) ||
                    (enum_ty.name().is_none() &&
                         self.variants().iter().any(|v|
                                                        ctx.options().constified_enums.matches(&v.name())))
            };
        let is_rust_enum = !is_bitfield && !is_constified_enum;
        if is_rust_enum {
            if !self.variants().is_empty() {
                builder = builder.with_attr(attributes::repr(repr_name));
            }
        } else if is_bitfield {
            builder = builder.with_attr(attributes::repr("C"));
        }
        if ctx.options().generate_comments {
            if let Some(comment) = item.comment() {
                builder = builder.with_attr(attributes::doc(comment));
            }
        }
        if !is_constified_enum {
            let derives =
                attributes::derives(&["Debug", "Copy", "Clone", "PartialEq",
                                      "Eq", "Hash"]);
            builder = builder.with_attr(derives);
        }
        fn add_constant<'a>(enum_: &Type, enum_canonical_name: &str,
                            variant_name: &str, referenced_name: &str,
                            enum_rust_ty: P<ast::Ty>,
                            result: &mut CodegenResult<'a>) {
            let constant_name =
                if enum_.name().is_some() {
                    format!("{}_{}" , enum_canonical_name , variant_name)
                } else { variant_name.into() };
            let constant =
                aster::AstBuilder::new().item().pub_().const_(constant_name).expr().path().ids(&[&*enum_canonical_name,
                                                                                                 referenced_name]).build().build(enum_rust_ty);
            result.push(constant);
        }
        let repr =
            self.repr().and_then(|repr|
                                     repr.try_to_rust_ty_or_opaque(ctx,
                                                                   &()).ok()).unwrap_or_else(||
                                                                                                 helpers::ast_ty::raw_type(ctx,
                                                                                                                           repr_name));
        let mut builder =
            EnumBuilder::new(builder, &name, repr, is_bitfield,
                             is_constified_enum);
        let mut seen_values = HashMap::<_, String>::new();
        let enum_rust_ty = item.to_rust_ty_or_opaque(ctx, &());
        let is_toplevel = item.is_toplevel(ctx);
        let parent_canonical_name =
            if is_toplevel {
                None
            } else { Some(item.parent_id().canonical_name(ctx)) };
        let constant_mangling_prefix =
            if ctx.options().prepend_enum_name {
                if enum_ty.name().is_none() {
                    parent_canonical_name.as_ref().map(|n| &*n)
                } else { Some(&name) }
            } else { None };
        let mut constified_variants = VecDeque::new();
        let mut iter = self.variants().iter().peekable();
        while let Some(variant) =
                  iter.next().or_else(|| constified_variants.pop_front()) {
            if variant.hidden() { continue ; }
            if variant.force_constification() && iter.peek().is_some() {
                constified_variants.push_back(variant);
                continue ;
            }
            match seen_values.entry(variant.val()) {
                Entry::Occupied(ref entry) => {
                    if is_rust_enum {
                        let variant_name = ctx.rust_mangle(variant.name());
                        let mangled_name =
                            if is_toplevel || enum_ty.name().is_some() {
                                variant_name
                            } else {
                                let parent_name =
                                    parent_canonical_name.as_ref().unwrap();
                                Cow::Owned(format!("{}_{}" , parent_name ,
                                                   variant_name))
                            };
                        let existing_variant_name = entry.get();
                        add_constant(enum_ty, &name, &*mangled_name,
                                     existing_variant_name,
                                     enum_rust_ty.clone(), result);
                    } else {
                        builder =
                            builder.with_variant(ctx, variant,
                                                 constant_mangling_prefix,
                                                 enum_rust_ty.clone(),
                                                 result);
                    }
                }
                Entry::Vacant(entry) => {
                    builder =
                        builder.with_variant(ctx, variant,
                                             constant_mangling_prefix,
                                             enum_rust_ty.clone(), result);
                    let variant_name = ctx.rust_mangle(variant.name());
                    if (is_rust_enum && enum_ty.name().is_none()) ||
                           variant.force_constification() {
                        let mangled_name =
                            if is_toplevel {
                                variant_name.clone()
                            } else {
                                let parent_name =
                                    parent_canonical_name.as_ref().unwrap();
                                Cow::Owned(format!("{}_{}" , parent_name ,
                                                   variant_name))
                            };
                        add_constant(enum_ty, &name, &mangled_name,
                                     &variant_name, enum_rust_ty.clone(),
                                     result);
                    }
                    entry.insert(variant_name.into_owned());
                }
            }
        }
        let enum_ = builder.build(ctx, enum_rust_ty, result);
        result.push(enum_);
    }
}
/// Fallible conversion to an opaque blob.
///
/// Implementors of this trait should provide the `try_get_layout` method to
/// fallibly get this thing's layout, which the provided `try_to_opaque` trait
/// method will use to convert the `Layout` into an opaque blob Rust type.
trait TryToOpaque {
    type
    Extra;
    /// Get the layout for this thing, if one is available.
    fn try_get_layout(&self, ctx: &BindgenContext, extra: &Self::Extra)
    -> error::Result<Layout>;
    /// Do not override this provided trait method.
    fn try_to_opaque(&self, ctx: &BindgenContext, extra: &Self::Extra)
     -> error::Result<P<ast::Ty>> {
        self.try_get_layout(ctx,
                            extra).map(|layout|
                                           BlobTyBuilder::new(layout).build())
    }
}
/// Infallible conversion of an IR thing to an opaque blob.
///
/// The resulting layout is best effort, and is unfortunately not guaranteed to
/// be correct. When all else fails, we fall back to a single byte layout as a
/// last resort, because C++ does not permit zero-sized types. See the note in
/// the `ToRustTyOrOpaque` doc comment about fallible versus infallible traits
/// and when each is appropriate.
///
/// Don't implement this directly. Instead implement `TryToOpaque`, and then
/// leverage the blanket impl for this trait.
trait ToOpaque: TryToOpaque {
    fn get_layout(&self, ctx: &BindgenContext, extra: &Self::Extra)
     -> Layout {
        self.try_get_layout(ctx,
                            extra).unwrap_or_else(|_| Layout::for_size(1))
    }
    fn to_opaque(&self, ctx: &BindgenContext, extra: &Self::Extra)
     -> P<ast::Ty> {
        let layout = self.get_layout(ctx, extra);
        BlobTyBuilder::new(layout).build()
    }
}
impl <T> ToOpaque for T where T: TryToOpaque { }
/// Fallible conversion from an IR thing to an *equivalent* Rust type.
///
/// If the C/C++ construct represented by the IR thing cannot (currently) be
/// represented in Rust (for example, instantiations of templates with
/// const-value generic parameters) then the impl should return an `Err`. It
/// should *not* attempt to return an opaque blob with the correct size and
/// alignment. That is the responsibility of the `TryToOpaque` trait.
trait TryToRustTy {
    type
    Extra;
    fn try_to_rust_ty(&self, ctx: &BindgenContext, extra: &Self::Extra)
    -> error::Result<P<ast::Ty>>;
}
/// Fallible conversion to a Rust type or an opaque blob with the correct size
/// and alignment.
///
/// Don't implement this directly. Instead implement `TryToRustTy` and
/// `TryToOpaque`, and then leverage the blanket impl for this trait below.
trait TryToRustTyOrOpaque: TryToRustTy + TryToOpaque {
    type
    Extra;
    fn try_to_rust_ty_or_opaque(&self, ctx: &BindgenContext,
                                extra: &<Self as TryToRustTyOrOpaque>::Extra)
    -> error::Result<P<ast::Ty>>;
}
impl <E, T> TryToRustTyOrOpaque for T where T: TryToRustTy<Extra = E> +
 TryToOpaque<Extra = E> {
    type
    Extra
    =
    E;
    fn try_to_rust_ty_or_opaque(&self, ctx: &BindgenContext, extra: &E)
     -> error::Result<P<ast::Ty>> {
        self.try_to_rust_ty(ctx,
                            extra).or_else(|_|
                                               {
                                                   if let Ok(layout) =
                                                          self.try_get_layout(ctx,
                                                                              extra)
                                                          {
                                                       Ok(BlobTyBuilder::new(layout).build())
                                                   } else {
                                                       Err(error::Error::NoLayoutForOpaqueBlob)
                                                   }
                                               })
    }
}
/// Infallible conversion to a Rust type, or an opaque blob with a best effort
/// of correct size and alignment.
///
/// Don't implement this directly. Instead implement `TryToRustTy` and
/// `TryToOpaque`, and then leverage the blanket impl for this trait below.
///
/// ### Fallible vs. Infallible Conversions to Rust Types
///
/// When should one use this infallible `ToRustTyOrOpaque` trait versus the
/// fallible `TryTo{RustTy, Opaque, RustTyOrOpaque}` triats? All fallible trait
/// implementations that need to convert another thing into a Rust type or
/// opaque blob in a nested manner should also use fallible trait methods and
/// propagate failure up the stack. Only infallible functions and methods like
/// CodeGenerator implementations should use the infallible
/// `ToRustTyOrOpaque`. The further out we push error recovery, the more likely
/// we are to get a usable `Layout` even if we can't generate an equivalent Rust
/// type for a C++ construct.
trait ToRustTyOrOpaque: TryToRustTy + ToOpaque {
    type
    Extra;
    fn to_rust_ty_or_opaque(&self, ctx: &BindgenContext,
                            extra: &<Self as ToRustTyOrOpaque>::Extra)
    -> P<ast::Ty>;
}
impl <E, T> ToRustTyOrOpaque for T where T: TryToRustTy<Extra = E> +
 ToOpaque<Extra = E> {
    type
    Extra
    =
    E;
    fn to_rust_ty_or_opaque(&self, ctx: &BindgenContext, extra: &E)
     -> P<ast::Ty> {
        self.try_to_rust_ty(ctx,
                            extra).unwrap_or_else(|_|
                                                      self.to_opaque(ctx,
                                                                     extra))
    }
}
impl TryToOpaque for ItemId {
    type
    Extra
    =
    ();
    fn try_get_layout(&self, ctx: &BindgenContext, _: &())
     -> error::Result<Layout> {
        ctx.resolve_item(*self).try_get_layout(ctx, &())
    }
}
impl TryToRustTy for ItemId {
    type
    Extra
    =
    ();
    fn try_to_rust_ty(&self, ctx: &BindgenContext, _: &())
     -> error::Result<P<ast::Ty>> {
        ctx.resolve_item(*self).try_to_rust_ty(ctx, &())
    }
}
impl TryToOpaque for Item {
    type
    Extra
    =
    ();
    fn try_get_layout(&self, ctx: &BindgenContext, _: &())
     -> error::Result<Layout> {
        self.kind().expect_type().try_get_layout(ctx, self)
    }
}
impl TryToRustTy for Item {
    type
    Extra
    =
    ();
    fn try_to_rust_ty(&self, ctx: &BindgenContext, _: &())
     -> error::Result<P<ast::Ty>> {
        self.kind().expect_type().try_to_rust_ty(ctx, self)
    }
}
impl TryToOpaque for Type {
    type
    Extra
    =
    Item;
    fn try_get_layout(&self, ctx: &BindgenContext, _: &Item)
     -> error::Result<Layout> {
        self.layout(ctx).ok_or(error::Error::NoLayoutForOpaqueBlob)
    }
}
impl TryToRustTy for Type {
    type
    Extra
    =
    Item;
    fn try_to_rust_ty(&self, ctx: &BindgenContext, item: &Item)
     -> error::Result<P<ast::Ty>> {
        use self::helpers::ast_ty::*;
        match *self.kind() {
            TypeKind::Void => Ok(raw_type(ctx, "c_void")),
            TypeKind::NullPtr => {
                Ok(raw_type(ctx, "c_void").to_ptr(true, ctx.span()))
            }
            TypeKind::Int(ik) => {
                match ik {
                    IntKind::Bool => Ok(aster::ty::TyBuilder::new().bool()),
                    IntKind::Char { .. } => Ok(raw_type(ctx, "c_char")),
                    IntKind::SChar => Ok(raw_type(ctx, "c_schar")),
                    IntKind::UChar => Ok(raw_type(ctx, "c_uchar")),
                    IntKind::Short => Ok(raw_type(ctx, "c_short")),
                    IntKind::UShort => Ok(raw_type(ctx, "c_ushort")),
                    IntKind::Int => Ok(raw_type(ctx, "c_int")),
                    IntKind::UInt => Ok(raw_type(ctx, "c_uint")),
                    IntKind::Long => Ok(raw_type(ctx, "c_long")),
                    IntKind::ULong => Ok(raw_type(ctx, "c_ulong")),
                    IntKind::LongLong => Ok(raw_type(ctx, "c_longlong")),
                    IntKind::ULongLong => Ok(raw_type(ctx, "c_ulonglong")),
                    IntKind::I8 => Ok(aster::ty::TyBuilder::new().i8()),
                    IntKind::U8 => Ok(aster::ty::TyBuilder::new().u8()),
                    IntKind::I16 => Ok(aster::ty::TyBuilder::new().i16()),
                    IntKind::U16 => Ok(aster::ty::TyBuilder::new().u16()),
                    IntKind::I32 => Ok(aster::ty::TyBuilder::new().i32()),
                    IntKind::U32 => Ok(aster::ty::TyBuilder::new().u32()),
                    IntKind::I64 => Ok(aster::ty::TyBuilder::new().i64()),
                    IntKind::U64 => Ok(aster::ty::TyBuilder::new().u64()),
                    IntKind::Custom { name, .. } => {
                        let ident = ctx.rust_ident_raw(name);
                        Ok({
                               let ext_cx = &*ctx.ext_cx();
                               ::quasi::parse_ty_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                                 {
                                                                                                     #[allow(unused_imports)]
                                                                                                     use quasi::IntoWrappedIterator;
                                                                                                     #[allow(unused_imports)]
                                                                                                     use quasi::IntoWrappedRepeat;
                                                                                                     let _sp =
                                                                                                         ext_cx.call_site();
                                                                                                     let mut tt =
                                                                                                         ::std::vec::Vec::new();
                                                                                                     tt.extend(::quasi::ToTokens::to_tokens(&ident,
                                                                                                                                            ext_cx).into_iter());
                                                                                                     tt
                                                                                                 }))
                           })
                    }
                    IntKind::U128 | IntKind::I128 => {
                        Ok(aster::ty::TyBuilder::new().array(2).u64())
                    }
                }
            }
            TypeKind::Float(fk) => Ok(float_kind_rust_type(ctx, fk)),
            TypeKind::Complex(fk) => {
                let float_path = float_kind_rust_type(ctx, fk);
                ctx.generated_bindegen_complex();
                Ok(if ctx.options().enable_cxx_namespaces {
                       {
                           let ext_cx = &*ctx.ext_cx();
                           ::quasi::parse_ty_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                             {
                                                                                                 #[allow(unused_imports)]
                                                                                                 use quasi::IntoWrappedIterator;
                                                                                                 #[allow(unused_imports)]
                                                                                                 use quasi::IntoWrappedRepeat;
                                                                                                 let _sp =
                                                                                                     ext_cx.call_site();
                                                                                                 let mut tt =
                                                                                                     ::std::vec::Vec::new();
                                                                                                 tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                 ::syntax::parse::token::Ident(ext_cx.ident_of("root"))));
                                                                                                 tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                 ::syntax::parse::token::ModSep));
                                                                                                 tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                 ::syntax::parse::token::Ident(ext_cx.ident_of("__BindgenComplex"))));
                                                                                                 tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                 ::syntax::parse::token::Lt));
                                                                                                 tt.extend(::quasi::ToTokens::to_tokens(&float_path,
                                                                                                                                        ext_cx).into_iter());
                                                                                                 tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                 ::syntax::parse::token::Gt));
                                                                                                 tt
                                                                                             }))
                       }
                   } else {
                       {
                           let ext_cx = &*ctx.ext_cx();
                           ::quasi::parse_ty_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                             {
                                                                                                 #[allow(unused_imports)]
                                                                                                 use quasi::IntoWrappedIterator;
                                                                                                 #[allow(unused_imports)]
                                                                                                 use quasi::IntoWrappedRepeat;
                                                                                                 let _sp =
                                                                                                     ext_cx.call_site();
                                                                                                 let mut tt =
                                                                                                     ::std::vec::Vec::new();
                                                                                                 tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                 ::syntax::parse::token::Ident(ext_cx.ident_of("__BindgenComplex"))));
                                                                                                 tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                 ::syntax::parse::token::Lt));
                                                                                                 tt.extend(::quasi::ToTokens::to_tokens(&float_path,
                                                                                                                                        ext_cx).into_iter());
                                                                                                 tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                 ::syntax::parse::token::Gt));
                                                                                                 tt
                                                                                             }))
                       }
                   })
            }
            TypeKind::Function(ref fs) => {
                let ty = fs.try_to_rust_ty(ctx, &())?;
                let prefix = ctx.trait_prefix();
                Ok({
                       let ext_cx = &*ctx.ext_cx();
                       ::quasi::parse_ty_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                         {
                                                                                             #[allow(unused_imports)]
                                                                                             use quasi::IntoWrappedIterator;
                                                                                             #[allow(unused_imports)]
                                                                                             use quasi::IntoWrappedRepeat;
                                                                                             let _sp =
                                                                                                 ext_cx.call_site();
                                                                                             let mut tt =
                                                                                                 ::std::vec::Vec::new();
                                                                                             tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                             ::syntax::parse::token::ModSep));
                                                                                             tt.extend(::quasi::ToTokens::to_tokens(&prefix,
                                                                                                                                    ext_cx).into_iter());
                                                                                             tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                             ::syntax::parse::token::ModSep));
                                                                                             tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                             ::syntax::parse::token::Ident(ext_cx.ident_of("option"))));
                                                                                             tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                             ::syntax::parse::token::ModSep));
                                                                                             tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                             ::syntax::parse::token::Ident(ext_cx.ident_of("Option"))));
                                                                                             tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                             ::syntax::parse::token::Lt));
                                                                                             tt.extend(::quasi::ToTokens::to_tokens(&ty,
                                                                                                                                    ext_cx).into_iter());
                                                                                             tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                             ::syntax::parse::token::Gt));
                                                                                             tt
                                                                                         }))
                   })
            }
            TypeKind::Array(item, len) => {
                let ty = item.try_to_rust_ty(ctx, &())?;
                Ok(aster::ty::TyBuilder::new().array(len).build(ty))
            }
            TypeKind::Enum(..) => {
                let path = item.namespace_aware_canonical_path(ctx);
                Ok(aster::AstBuilder::new().ty().path().ids(path).build())
            }
            TypeKind::TemplateInstantiation(ref inst) => {
                inst.try_to_rust_ty(ctx, self)
            }
            TypeKind::ResolvedTypeRef(inner) =>
            inner.try_to_rust_ty(ctx, &()),
            TypeKind::TemplateAlias(inner, _) | TypeKind::Alias(inner) => {
                let template_params =
                    item.used_template_params(ctx).unwrap_or(vec!()).into_iter().filter(|param|
                                                                                            param.is_named(ctx,
                                                                                                           &())).collect::<Vec<_>>();
                let spelling = self.name().expect("Unnamed alias?");
                if item.is_opaque(ctx) && !template_params.is_empty() {
                    self.try_to_opaque(ctx, item)
                } else if let Some(ty) =
                 utils::type_from_named(ctx, spelling, inner) {
                    Ok(ty)
                } else {
                    utils::build_templated_path(item, ctx, template_params)
                }
            }
            TypeKind::Comp(ref info) => {
                let template_params = item.used_template_params(ctx);
                if info.has_non_type_template_params() ||
                       (item.is_opaque(ctx) && template_params.is_some()) {
                    return self.try_to_opaque(ctx, item);
                }
                let template_params = template_params.unwrap_or(vec!());
                utils::build_templated_path(item, ctx, template_params)
            }
            TypeKind::Opaque => { self.try_to_opaque(ctx, item) }
            TypeKind::BlockPointer => {
                let void = raw_type(ctx, "c_void");
                Ok(void.to_ptr(false, ctx.span()))
            }
            TypeKind::Pointer(inner) | TypeKind::Reference(inner) => {
                let inner = ctx.resolve_item(inner);
                let inner_ty = inner.expect_type();
                let ty = inner.to_rust_ty_or_opaque(ctx, &());
                if inner_ty.canonical_type(ctx).is_function() {
                    Ok(ty)
                } else {
                    let is_const =
                        self.is_const() || inner.expect_type().is_const();
                    Ok(ty.to_ptr(is_const, ctx.span()))
                }
            }
            TypeKind::Named => {
                let name = item.canonical_name(ctx);
                let ident = ctx.rust_ident(&name);
                Ok({
                       let ext_cx = &*ctx.ext_cx();
                       ::quasi::parse_ty_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                         {
                                                                                             #[allow(unused_imports)]
                                                                                             use quasi::IntoWrappedIterator;
                                                                                             #[allow(unused_imports)]
                                                                                             use quasi::IntoWrappedRepeat;
                                                                                             let _sp =
                                                                                                 ext_cx.call_site();
                                                                                             let mut tt =
                                                                                                 ::std::vec::Vec::new();
                                                                                             tt.extend(::quasi::ToTokens::to_tokens(&ident,
                                                                                                                                    ext_cx).into_iter());
                                                                                             tt
                                                                                         }))
                   })
            }
            TypeKind::ObjCSel =>
            Ok({
                   let ext_cx = &*ctx.ext_cx();
                   ::quasi::parse_ty_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                     {
                                                                                         #[allow(unused_imports)]
                                                                                         use quasi::IntoWrappedIterator;
                                                                                         #[allow(unused_imports)]
                                                                                         use quasi::IntoWrappedRepeat;
                                                                                         let _sp =
                                                                                             ext_cx.call_site();
                                                                                         let mut tt =
                                                                                             ::std::vec::Vec::new();
                                                                                         tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                         ::syntax::parse::token::Ident(ext_cx.ident_of("objc"))));
                                                                                         tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                         ::syntax::parse::token::ModSep));
                                                                                         tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                         ::syntax::parse::token::Ident(ext_cx.ident_of("runtime"))));
                                                                                         tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                         ::syntax::parse::token::ModSep));
                                                                                         tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                         ::syntax::parse::token::Ident(ext_cx.ident_of("Sel"))));
                                                                                         tt
                                                                                     }))
               }),
            TypeKind::ObjCId | TypeKind::ObjCInterface(..) =>
            Ok({
                   let ext_cx = &*ctx.ext_cx();
                   ::quasi::parse_ty_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                     {
                                                                                         #[allow(unused_imports)]
                                                                                         use quasi::IntoWrappedIterator;
                                                                                         #[allow(unused_imports)]
                                                                                         use quasi::IntoWrappedRepeat;
                                                                                         let _sp =
                                                                                             ext_cx.call_site();
                                                                                         let mut tt =
                                                                                             ::std::vec::Vec::new();
                                                                                         tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                         ::syntax::parse::token::Ident(ext_cx.ident_of("id"))));
                                                                                         tt
                                                                                     }))
               }),
            ref u@TypeKind::UnresolvedTypeRef(..) => {
                unreachable!("Should have been resolved after parsing {:?}!" ,
                             u)
            }
        }
    }
}
impl TryToOpaque for TemplateInstantiation {
    type
    Extra
    =
    Type;
    fn try_get_layout(&self, ctx: &BindgenContext, self_ty: &Type)
     -> error::Result<Layout> {
        self_ty.layout(ctx).ok_or(error::Error::NoLayoutForOpaqueBlob)
    }
}
impl TryToRustTy for TemplateInstantiation {
    type
    Extra
    =
    Type;
    fn try_to_rust_ty(&self, ctx: &BindgenContext, _: &Type)
     -> error::Result<P<ast::Ty>> {
        let decl = self.template_definition();
        let mut ty = decl.try_to_rust_ty(ctx, &())?.unwrap();
        let decl_params =
            match decl.self_template_params(ctx) {
                Some(params) => params,
                None => {
                    extra_assert!(ctx . resolve_type_through_type_refs ( decl
                                  ) . is_opaque (  ));
                    return Err(error::Error::InstantiationOfOpaqueType);
                }
            };
        if let ast::TyKind::Path(_, ref mut path) = ty.node {
            let template_args =
                self.template_arguments().iter().zip(decl_params.iter()).filter(|&(_,
                                                                                   param)|
                                                                                    ctx.uses_template_parameter(decl,
                                                                                                                *param)).map(|(arg,
                                                                                                                               _)|
                                                                                                                                 arg.try_to_rust_ty(ctx,
                                                                                                                                                    &())).collect::<error::Result<Vec<_>>>()?;
            path.segments.last_mut().unwrap().parameters =
                if template_args.is_empty() {
                    None
                } else {
                    Some(P(ast::PathParameters::AngleBracketed(ast::AngleBracketedParameterData{lifetimes:
                                                                                                    vec!(),
                                                                                                types:
                                                                                                    template_args,
                                                                                                bindings:
                                                                                                    vec!(),})))
                }
        }
        Ok(P(ty))
    }
}
impl TryToRustTy for FunctionSig {
    type
    Extra
    =
    ();
    fn try_to_rust_ty(&self, ctx: &BindgenContext, _: &())
     -> error::Result<P<ast::Ty>> {
        let ret = utils::fnsig_return_ty(ctx, &self);
        let arguments = utils::fnsig_arguments(ctx, &self);
        let decl =
            P(ast::FnDecl{inputs: arguments,
                          output: ret,
                          variadic: self.is_variadic(),});
        let fnty =
            ast::TyKind::BareFn(P(ast::BareFnTy{unsafety:
                                                    ast::Unsafety::Unsafe,
                                                abi:
                                                    self.abi().expect("Invalid abi for function!"),
                                                lifetimes: vec!(),
                                                decl: decl,}));
        Ok(P(ast::Ty{id: ast::DUMMY_NODE_ID, node: fnty, span: ctx.span(),}))
    }
}
impl CodeGenerator for Function {
    type
    Extra
    =
    Item;
    fn codegen<'a>(&self, ctx: &BindgenContext,
                   result: &mut CodegenResult<'a>,
                   _whitelisted_items: &ItemSet, item: &Item) {
        debug!("<Function as CodeGenerator>::codegen: item = {:?}" , item);
        let name = self.name();
        let mut canonical_name = item.canonical_name(ctx);
        let mangled_name = self.mangled_name();
        {
            let seen_symbol_name = mangled_name.unwrap_or(&canonical_name);
            if result.seen_function(seen_symbol_name) { return; }
            result.saw_function(seen_symbol_name);
        }
        let signature_item = ctx.resolve_item(self.signature());
        let signature =
            signature_item.kind().expect_type().canonical_type(ctx);
        let signature =
            match *signature.kind() {
                TypeKind::Function(ref sig) => sig,
                _ =>
                panic!("Signature kind is not a Function: {:?}" , signature),
            };
        let fndecl = utils::rust_fndecl_from_signature(ctx, signature_item);
        let mut attributes = vec!();
        if ctx.options().generate_comments {
            if let Some(comment) = item.comment() {
                attributes.push(attributes::doc(comment));
            }
        }
        if let Some(mangled) = mangled_name {
            attributes.push(attributes::link_name(mangled));
        } else if name != canonical_name {
            attributes.push(attributes::link_name(name));
        }
        let foreign_item_kind =
            ast::ForeignItemKind::Fn(fndecl, ast::Generics::default());
        let times_seen = result.overload_number(&canonical_name);
        if times_seen > 0 {
            write!(& mut canonical_name , "{}" , times_seen).unwrap();
        }
        let foreign_item =
            ast::ForeignItem{ident: ctx.rust_ident_raw(&canonical_name),
                             attrs: attributes,
                             node: foreign_item_kind,
                             id: ast::DUMMY_NODE_ID,
                             span: ctx.span(),
                             vis: ast::Visibility::Public,};
        let item =
            ForeignModBuilder::new(signature.abi().expect("Invalid abi for function!")).with_foreign_item(foreign_item).build(ctx);
        result.push(item);
    }
}
fn objc_method_codegen(ctx: &BindgenContext, method: &ObjCMethod,
                       class_name: Option<&str>)
 -> (ast::ImplItem, ast::TraitItem) {
    let signature = method.signature();
    let fn_args = utils::fnsig_arguments(ctx, signature);
    let fn_ret = utils::fnsig_return_ty(ctx, signature);
    let sig =
        if method.is_class_method() {
            aster::AstBuilder::new().method_sig().unsafe_().fn_decl().with_args(fn_args.clone()).build(fn_ret)
        } else {
            aster::AstBuilder::new().method_sig().unsafe_().fn_decl().self_().build(ast::SelfKind::Value(ast::Mutability::Immutable)).with_args(fn_args.clone()).build(fn_ret)
        };
    let arg_names: Vec<_> =
        fn_args.iter().map(|ref arg|
                               match arg.pat.node {
                                   ast::PatKind::Ident(_, ref spanning, _) =>
                                   {
                                       spanning.node.name.as_str().to_string()
                                   }
                                   _ => { panic!("odd argument!"); }
                               }).collect();
    let methods_and_args =
        ctx.rust_ident(&method.format_method_call(&arg_names));
    let body =
        if method.is_class_method() {
            let class_name =
                class_name.expect("Generating a class method without class name?").to_owned();
            let expect_msg = format!("Couldn't find {}" , class_name);
            {
                let ext_cx = &*ctx.ext_cx();
                ::quasi::parse_stmt_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                    {
                                                                                        #[allow(unused_imports)]
                                                                                        use quasi::IntoWrappedIterator;
                                                                                        #[allow(unused_imports)]
                                                                                        use quasi::IntoWrappedRepeat;
                                                                                        let _sp =
                                                                                            ext_cx.call_site();
                                                                                        let mut tt =
                                                                                            ::std::vec::Vec::new();
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("msg_send"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Not));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("objc"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("runtime"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("Class"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("get"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&class_name,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Dot));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("expect"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&expect_msg,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Comma));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&methods_and_args,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Bracket)));
                                                                                        tt
                                                                                    }))
            }.unwrap()
        } else {
            {
                let ext_cx = &*ctx.ext_cx();
                ::quasi::parse_stmt_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                    {
                                                                                        #[allow(unused_imports)]
                                                                                        use quasi::IntoWrappedIterator;
                                                                                        #[allow(unused_imports)]
                                                                                        use quasi::IntoWrappedRepeat;
                                                                                        let _sp =
                                                                                            ext_cx.call_site();
                                                                                        let mut tt =
                                                                                            ::std::vec::Vec::new();
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("msg_send"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Not));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("self"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Comma));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&methods_and_args,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Bracket)));
                                                                                        tt
                                                                                    }))
            }.unwrap()
        };
    let block =
        ast::Block{stmts: vec!(body),
                   id: ast::DUMMY_NODE_ID,
                   rules: ast::BlockCheckMode::Default,
                   span: ctx.span(),};
    let attrs = vec!();
    let impl_item =
        ast::ImplItem{id: ast::DUMMY_NODE_ID,
                      ident: ctx.rust_ident(method.rust_name()),
                      vis: ast::Visibility::Inherited,
                      attrs: attrs.clone(),
                      node: ast::ImplItemKind::Method(sig.clone(), P(block)),
                      defaultness: ast::Defaultness::Final,
                      span: ctx.span(),};
    let trait_item =
        ast::TraitItem{id: ast::DUMMY_NODE_ID,
                       ident: ctx.rust_ident(method.rust_name()),
                       attrs: attrs,
                       node: ast::TraitItemKind::Method(sig, None),
                       span: ctx.span(),};
    (impl_item, trait_item)
}
impl CodeGenerator for ObjCInterface {
    type
    Extra
    =
    Item;
    fn codegen<'a>(&self, ctx: &BindgenContext,
                   result: &mut CodegenResult<'a>,
                   _whitelisted_items: &ItemSet, _: &Item) {
        let mut impl_items = vec!();
        let mut trait_items = vec!();
        for method in self.methods() {
            let (impl_item, trait_item) =
                objc_method_codegen(ctx, method, None);
            impl_items.push(impl_item);
            trait_items.push(trait_item)
        }
        for class_method in self.class_methods() {
            let (impl_item, trait_item) =
                objc_method_codegen(ctx, class_method, Some(self.name()));
            impl_items.push(impl_item);
            trait_items.push(trait_item)
        }
        let trait_name = self.rust_name();
        let trait_block =
            aster::AstBuilder::new().item().pub_().trait_(&trait_name).with_items(trait_items).build();
        let ty_for_impl =
            {
                let ext_cx = &*ctx.ext_cx();
                ::quasi::parse_ty_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                  {
                                                                                      #[allow(unused_imports)]
                                                                                      use quasi::IntoWrappedIterator;
                                                                                      #[allow(unused_imports)]
                                                                                      use quasi::IntoWrappedRepeat;
                                                                                      let _sp =
                                                                                          ext_cx.call_site();
                                                                                      let mut tt =
                                                                                          ::std::vec::Vec::new();
                                                                                      tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                      ::syntax::parse::token::Ident(ext_cx.ident_of("id"))));
                                                                                      tt
                                                                                  }))
            };
        let impl_block =
            aster::AstBuilder::new().item().impl_().trait_().id(&trait_name).build().with_items(impl_items).build_ty(ty_for_impl);
        result.push(trait_block);
        result.push(impl_block);
        result.saw_objc();
    }
}
pub fn codegen(context: &mut BindgenContext) -> Vec<P<ast::Item>> {
    context.gen(|context|
                    {
                        let counter = Cell::new(0);
                        let mut result = CodegenResult::new(&counter);
                        debug!("codegen: {:?}" , context . options (  ));
                        let whitelisted_items: ItemSet =
                            context.whitelisted_items().collect();
                        if context.options().emit_ir {
                            for &id in whitelisted_items.iter() {
                                let item = context.resolve_item(id);
                                println!("ir: {:?} = {:#?}" , id , item);
                            }
                        }
                        if let Some(path) =
                               context.options().emit_ir_graphviz.as_ref() {
                            match dot::write_dot_file(context, path) {
                                Ok(()) =>
                                info!("Your dot file was generated successfully into: {}"
                                      , path),
                                Err(e) => error!("{}" , e),
                            }
                        }
                        context.resolve_item(context.root_module()).codegen(context,
                                                                            &mut result,
                                                                            &whitelisted_items,
                                                                            &());
                        result.items
                    })
}
mod utils {
    use super::{error, TryToRustTy, ToRustTyOrOpaque};
    use aster;
    use ir::context::{BindgenContext, ItemId};
    use ir::function::FunctionSig;
    use ir::item::{Item, ItemCanonicalPath};
    use ir::ty::TypeKind;
    use std::mem;
    use syntax::ast;
    use syntax::ptr::P;
    pub fn prepend_objc_header(ctx: &BindgenContext,
                               result: &mut Vec<P<ast::Item>>) {
        let use_objc =
            if ctx.options().objc_extern_crate {
                {
                    let ext_cx = &*ctx.ext_cx();
                    ::quasi::parse_item_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                        {
                                                                                            #[allow(unused_imports)]
                                                                                            use quasi::IntoWrappedIterator;
                                                                                            #[allow(unused_imports)]
                                                                                            use quasi::IntoWrappedRepeat;
                                                                                            let _sp =
                                                                                                ext_cx.call_site();
                                                                                            let mut tt =
                                                                                                ::std::vec::Vec::new();
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("use"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("objc"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Semi));
                                                                                            tt
                                                                                        }))
                }.unwrap()
            } else {
                {
                    let ext_cx = &*ctx.ext_cx();
                    ::quasi::parse_item_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                        {
                                                                                            #[allow(unused_imports)]
                                                                                            use quasi::IntoWrappedIterator;
                                                                                            #[allow(unused_imports)]
                                                                                            use quasi::IntoWrappedRepeat;
                                                                                            let _sp =
                                                                                                ext_cx.call_site();
                                                                                            let mut tt =
                                                                                                ::std::vec::Vec::new();
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Pound));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::OpenDelim(::syntax::parse::token::Bracket)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("macro_use"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::CloseDelim(::syntax::parse::token::Bracket)));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("extern"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("crate"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Ident(ext_cx.ident_of("objc"))));
                                                                                            tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                            ::syntax::parse::token::Semi));
                                                                                            tt
                                                                                        }))
                }.unwrap()
            };
        let id_type =
            {
                let ext_cx = &*ctx.ext_cx();
                ::quasi::parse_item_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                    {
                                                                                        #[allow(unused_imports)]
                                                                                        use quasi::IntoWrappedIterator;
                                                                                        #[allow(unused_imports)]
                                                                                        use quasi::IntoWrappedRepeat;
                                                                                        let _sp =
                                                                                            ext_cx.call_site();
                                                                                        let mut tt =
                                                                                            ::std::vec::Vec::new();
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Pound));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("allow"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("non_camel_case_types"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("pub"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("type"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("id"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Eq));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::BinOp(::syntax::parse::token::Star)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("mut"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("objc"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("runtime"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("Object"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Semi));
                                                                                        tt
                                                                                    }))
            }.unwrap();
        let items = vec!(use_objc , id_type);
        let old_items = mem::replace(result, items);
        result.extend(old_items.into_iter());
    }
    pub fn prepend_union_types(ctx: &BindgenContext,
                               result: &mut Vec<P<ast::Item>>) {
        let prefix = ctx.trait_prefix();
        let union_field_decl =
            {
                let ext_cx = &*ctx.ext_cx();
                ::quasi::parse_item_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                    {
                                                                                        #[allow(unused_imports)]
                                                                                        use quasi::IntoWrappedIterator;
                                                                                        #[allow(unused_imports)]
                                                                                        use quasi::IntoWrappedRepeat;
                                                                                        let _sp =
                                                                                            ext_cx.call_site();
                                                                                        let mut tt =
                                                                                            ::std::vec::Vec::new();
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Pound));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("repr"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("C"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("pub"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("struct"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("__BindgenUnionField"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Lt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("T"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Gt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&prefix,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("marker"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("PhantomData"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Lt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("T"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Gt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Semi));
                                                                                        tt
                                                                                    }))
            }.unwrap();
        let union_field_impl =
            {
                let ext_cx = &*&ctx.ext_cx();
                ::quasi::parse_item_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                    {
                                                                                        #[allow(unused_imports)]
                                                                                        use quasi::IntoWrappedIterator;
                                                                                        #[allow(unused_imports)]
                                                                                        use quasi::IntoWrappedRepeat;
                                                                                        let _sp =
                                                                                            ext_cx.call_site();
                                                                                        let mut tt =
                                                                                            ::std::vec::Vec::new();
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("impl"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Lt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("T"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Gt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("__BindgenUnionField"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Lt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("T"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Gt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Pound));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("inline"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("pub"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("fn"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("new"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::RArrow));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("Self"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("__BindgenUnionField"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&prefix,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("marker"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("PhantomData"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Pound));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("inline"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("pub"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("unsafe"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("fn"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("as_ref"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("self"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::RArrow));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("T"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&prefix,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("mem"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("transmute"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("self"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Pound));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("inline"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("pub"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("unsafe"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("fn"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("as_mut"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("mut"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("self"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::RArrow));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("mut"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("T"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&prefix,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("mem"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("transmute"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("self"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                        tt
                                                                                    }))
            }.unwrap();
        let union_field_default_impl =
            {
                let ext_cx = &*&ctx.ext_cx();
                ::quasi::parse_item_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                    {
                                                                                        #[allow(unused_imports)]
                                                                                        use quasi::IntoWrappedIterator;
                                                                                        #[allow(unused_imports)]
                                                                                        use quasi::IntoWrappedRepeat;
                                                                                        let _sp =
                                                                                            ext_cx.call_site();
                                                                                        let mut tt =
                                                                                            ::std::vec::Vec::new();
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("impl"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Lt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("T"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Gt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&prefix,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("default"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("Default"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("for"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("__BindgenUnionField"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Lt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("T"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Gt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Pound));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("inline"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("fn"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("default"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::RArrow));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("Self"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("Self"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("new"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                        tt
                                                                                    }))
            }.unwrap();
        let union_field_clone_impl =
            {
                let ext_cx = &*&ctx.ext_cx();
                ::quasi::parse_item_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                    {
                                                                                        #[allow(unused_imports)]
                                                                                        use quasi::IntoWrappedIterator;
                                                                                        #[allow(unused_imports)]
                                                                                        use quasi::IntoWrappedRepeat;
                                                                                        let _sp =
                                                                                            ext_cx.call_site();
                                                                                        let mut tt =
                                                                                            ::std::vec::Vec::new();
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("impl"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Lt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("T"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Gt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&prefix,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("clone"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("Clone"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("for"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("__BindgenUnionField"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Lt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("T"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Gt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Pound));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("inline"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("fn"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("clone"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("self"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::RArrow));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("Self"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("Self"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("new"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                        tt
                                                                                    }))
            }.unwrap();
        let union_field_copy_impl =
            {
                let ext_cx = &*&ctx.ext_cx();
                ::quasi::parse_item_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                    {
                                                                                        #[allow(unused_imports)]
                                                                                        use quasi::IntoWrappedIterator;
                                                                                        #[allow(unused_imports)]
                                                                                        use quasi::IntoWrappedRepeat;
                                                                                        let _sp =
                                                                                            ext_cx.call_site();
                                                                                        let mut tt =
                                                                                            ::std::vec::Vec::new();
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("impl"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Lt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("T"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Gt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&prefix,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("marker"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("Copy"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("for"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("__BindgenUnionField"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Lt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("T"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Gt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                        tt
                                                                                    }))
            }.unwrap();
        let union_field_debug_impl =
            {
                let ext_cx = &*ctx.ext_cx();
                ::quasi::parse_item_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                    {
                                                                                        #[allow(unused_imports)]
                                                                                        use quasi::IntoWrappedIterator;
                                                                                        #[allow(unused_imports)]
                                                                                        use quasi::IntoWrappedRepeat;
                                                                                        let _sp =
                                                                                            ext_cx.call_site();
                                                                                        let mut tt =
                                                                                            ::std::vec::Vec::new();
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("impl"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Lt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("T"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Gt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&prefix,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("fmt"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("Debug"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("for"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("__BindgenUnionField"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Lt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("T"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Gt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("fn"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("fmt"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("self"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Comma));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("fmt"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Colon));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("mut"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&prefix,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("fmt"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("Formatter"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::RArrow));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&prefix,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("fmt"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("Result"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("fmt"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Dot));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("write_str"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Literal(::syntax::parse::token::Str_(ext_cx.name_of("__BindgenUnionField")),
                                                                                                                                                                        ::std::option::Option::None)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                        tt
                                                                                    }))
            }.unwrap();
        let items =
            vec!(union_field_decl , union_field_impl ,
                 union_field_default_impl , union_field_clone_impl ,
                 union_field_copy_impl , union_field_debug_impl);
        let old_items = mem::replace(result, items);
        result.extend(old_items.into_iter());
    }
    pub fn prepend_incomplete_array_types(ctx: &BindgenContext,
                                          result: &mut Vec<P<ast::Item>>) {
        let prefix = ctx.trait_prefix();
        let incomplete_array_decl =
            {
                let ext_cx = &*ctx.ext_cx();
                ::quasi::parse_item_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                    {
                                                                                        #[allow(unused_imports)]
                                                                                        use quasi::IntoWrappedIterator;
                                                                                        #[allow(unused_imports)]
                                                                                        use quasi::IntoWrappedRepeat;
                                                                                        let _sp =
                                                                                            ext_cx.call_site();
                                                                                        let mut tt =
                                                                                            ::std::vec::Vec::new();
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Pound));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("repr"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("C"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Pound));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("derive"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("Default"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("pub"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("struct"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("__IncompleteArrayField"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Lt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("T"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Gt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&prefix,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("marker"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("PhantomData"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Lt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("T"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Gt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Semi));
                                                                                        tt
                                                                                    }))
            }.unwrap();
        let incomplete_array_impl =
            {
                let ext_cx = &*&ctx.ext_cx();
                ::quasi::parse_item_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                    {
                                                                                        #[allow(unused_imports)]
                                                                                        use quasi::IntoWrappedIterator;
                                                                                        #[allow(unused_imports)]
                                                                                        use quasi::IntoWrappedRepeat;
                                                                                        let _sp =
                                                                                            ext_cx.call_site();
                                                                                        let mut tt =
                                                                                            ::std::vec::Vec::new();
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("impl"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Lt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("T"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Gt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("__IncompleteArrayField"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Lt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("T"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Gt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Pound));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("inline"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("pub"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("fn"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("new"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::RArrow));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("Self"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("__IncompleteArrayField"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&prefix,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("marker"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("PhantomData"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Pound));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("inline"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("pub"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("unsafe"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("fn"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("as_ptr"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("self"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::RArrow));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::BinOp(::syntax::parse::token::Star)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("const"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("T"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&prefix,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("mem"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("transmute"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("self"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Pound));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("inline"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("pub"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("unsafe"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("fn"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("as_mut_ptr"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("mut"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("self"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::RArrow));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::BinOp(::syntax::parse::token::Star)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("mut"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("T"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&prefix,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("mem"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("transmute"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("self"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Pound));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("inline"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("pub"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("unsafe"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("fn"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("as_slice"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("self"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Comma));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("len"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Colon));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("usize"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::RArrow));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("T"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&prefix,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("slice"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("from_raw_parts"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("self"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Dot));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("as_ptr"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Comma));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("len"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Pound));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("inline"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("pub"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("unsafe"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("fn"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("as_mut_slice"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("mut"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("self"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Comma));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("len"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Colon));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("usize"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::RArrow));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("mut"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("T"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&prefix,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("slice"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("from_raw_parts_mut"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("self"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Dot));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("as_mut_ptr"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Comma));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("len"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                        tt
                                                                                    }))
            }.unwrap();
        let incomplete_array_debug_impl =
            {
                let ext_cx = &*ctx.ext_cx();
                ::quasi::parse_item_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                    {
                                                                                        #[allow(unused_imports)]
                                                                                        use quasi::IntoWrappedIterator;
                                                                                        #[allow(unused_imports)]
                                                                                        use quasi::IntoWrappedRepeat;
                                                                                        let _sp =
                                                                                            ext_cx.call_site();
                                                                                        let mut tt =
                                                                                            ::std::vec::Vec::new();
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("impl"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Lt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("T"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Gt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&prefix,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("fmt"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("Debug"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("for"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("__IncompleteArrayField"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Lt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("T"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Gt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("fn"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("fmt"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("self"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Comma));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("fmt"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Colon));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("mut"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&prefix,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("fmt"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("Formatter"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::RArrow));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&prefix,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("fmt"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("Result"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("fmt"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Dot));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("write_str"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Literal(::syntax::parse::token::Str_(ext_cx.name_of("__IncompleteArrayField")),
                                                                                                                                                                        ::std::option::Option::None)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                        tt
                                                                                    }))
            }.unwrap();
        let incomplete_array_clone_impl =
            {
                let ext_cx = &*&ctx.ext_cx();
                ::quasi::parse_item_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                    {
                                                                                        #[allow(unused_imports)]
                                                                                        use quasi::IntoWrappedIterator;
                                                                                        #[allow(unused_imports)]
                                                                                        use quasi::IntoWrappedRepeat;
                                                                                        let _sp =
                                                                                            ext_cx.call_site();
                                                                                        let mut tt =
                                                                                            ::std::vec::Vec::new();
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("impl"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Lt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("T"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Gt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&prefix,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("clone"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("Clone"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("for"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("__IncompleteArrayField"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Lt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("T"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Gt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Pound));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("inline"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("fn"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("clone"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::BinOp(::syntax::parse::token::And)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("self"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::RArrow));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("Self"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("Self"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("new"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                        tt
                                                                                    }))
            }.unwrap();
        let incomplete_array_copy_impl =
            {
                let ext_cx = &*&ctx.ext_cx();
                ::quasi::parse_item_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                    {
                                                                                        #[allow(unused_imports)]
                                                                                        use quasi::IntoWrappedIterator;
                                                                                        #[allow(unused_imports)]
                                                                                        use quasi::IntoWrappedRepeat;
                                                                                        let _sp =
                                                                                            ext_cx.call_site();
                                                                                        let mut tt =
                                                                                            ::std::vec::Vec::new();
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("impl"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Lt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("T"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Gt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.extend(::quasi::ToTokens::to_tokens(&prefix,
                                                                                                                               ext_cx).into_iter());
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("marker"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::ModSep));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("Copy"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("for"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("__IncompleteArrayField"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Lt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("T"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Gt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                        tt
                                                                                    }))
            }.unwrap();
        let items =
            vec!(incomplete_array_decl , incomplete_array_impl ,
                 incomplete_array_debug_impl , incomplete_array_clone_impl ,
                 incomplete_array_copy_impl);
        let old_items = mem::replace(result, items);
        result.extend(old_items.into_iter());
    }
    pub fn prepend_complex_type(ctx: &BindgenContext,
                                result: &mut Vec<P<ast::Item>>) {
        let complex_type =
            {
                let ext_cx = &*ctx.ext_cx();
                ::quasi::parse_item_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                    {
                                                                                        #[allow(unused_imports)]
                                                                                        use quasi::IntoWrappedIterator;
                                                                                        #[allow(unused_imports)]
                                                                                        use quasi::IntoWrappedRepeat;
                                                                                        let _sp =
                                                                                            ext_cx.call_site();
                                                                                        let mut tt =
                                                                                            ::std::vec::Vec::new();
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Pound));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("derive"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("PartialEq"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Comma));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("Copy"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Comma));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("Clone"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Comma));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("Hash"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Comma));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("Debug"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Comma));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("Default"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Pound));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("repr"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("C"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Paren)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Bracket)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("pub"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("struct"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("__BindgenComplex"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Lt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("T"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Gt));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::OpenDelim(::syntax::parse::token::Brace)));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("pub"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("re"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Colon));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("T"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Comma));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("pub"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("im"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Colon));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::Ident(ext_cx.ident_of("T"))));
                                                                                        tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                        ::syntax::parse::token::CloseDelim(::syntax::parse::token::Brace)));
                                                                                        tt
                                                                                    }))
            }.unwrap();
        let items = vec!(complex_type);
        let old_items = mem::replace(result, items);
        result.extend(old_items.into_iter());
    }
    pub fn build_templated_path(item: &Item, ctx: &BindgenContext,
                                template_params: Vec<ItemId>)
     -> error::Result<P<ast::Ty>> {
        let path = item.namespace_aware_canonical_path(ctx);
        let builder = aster::AstBuilder::new().ty().path();
        let template_params =
            template_params.iter().map(|param|
                                           param.try_to_rust_ty(ctx,
                                                                &())).collect::<error::Result<Vec<_>>>()?;
        if path.len() == 1 {
            return Ok(builder.segment(&path[0]).with_tys(template_params).build().build());
        }
        let mut builder = builder.id(&path[0]);
        for (i, segment) in path.iter().skip(1).enumerate() {
            builder =
                if i == path.len() - 2 {
                    builder.segment(&segment).with_tys(template_params.clone()).build()
                } else { builder.segment(&segment).build() }
        }
        Ok(builder.build())
    }
    fn primitive_ty(ctx: &BindgenContext, name: &str) -> P<ast::Ty> {
        let ident = ctx.rust_ident_raw(&name);
        {
            let ext_cx = &*ctx.ext_cx();
            ::quasi::parse_ty_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                              {
                                                                                  #[allow(unused_imports)]
                                                                                  use quasi::IntoWrappedIterator;
                                                                                  #[allow(unused_imports)]
                                                                                  use quasi::IntoWrappedRepeat;
                                                                                  let _sp =
                                                                                      ext_cx.call_site();
                                                                                  let mut tt =
                                                                                      ::std::vec::Vec::new();
                                                                                  tt.extend(::quasi::ToTokens::to_tokens(&ident,
                                                                                                                         ext_cx).into_iter());
                                                                                  tt
                                                                              }))
        }
    }
    pub fn type_from_named(ctx: &BindgenContext, name: &str, _inner: ItemId)
     -> Option<P<ast::Ty>> {
        Some(match name {
                 "int8_t" => primitive_ty(ctx, "i8"),
                 "uint8_t" => primitive_ty(ctx, "u8"),
                 "int16_t" => primitive_ty(ctx, "i16"),
                 "uint16_t" => primitive_ty(ctx, "u16"),
                 "int32_t" => primitive_ty(ctx, "i32"),
                 "uint32_t" => primitive_ty(ctx, "u32"),
                 "int64_t" => primitive_ty(ctx, "i64"),
                 "uint64_t" => primitive_ty(ctx, "u64"),
                 "uintptr_t" | "size_t" => primitive_ty(ctx, "usize"),
                 "intptr_t" | "ptrdiff_t" | "ssize_t" => {
                     primitive_ty(ctx, "isize")
                 }
                 _ => return None,
             })
    }
    pub fn rust_fndecl_from_signature(ctx: &BindgenContext, sig: &Item)
     -> P<ast::FnDecl> {
        let signature = sig.kind().expect_type().canonical_type(ctx);
        let signature =
            match *signature.kind() {
                TypeKind::Function(ref sig) => sig,
                _ => panic!("How?"),
            };
        let decl_ty =
            signature.try_to_rust_ty(ctx,
                                     &()).expect("function signature to Rust type conversion is infallible");
        match decl_ty.unwrap().node {
            ast::TyKind::BareFn(bare_fn) => bare_fn.unwrap().decl,
            _ => panic!("How did this happen exactly?"),
        }
    }
    pub fn fnsig_return_ty(ctx: &BindgenContext, sig: &FunctionSig)
     -> ast::FunctionRetTy {
        let return_item = ctx.resolve_item(sig.return_type());
        if let TypeKind::Void = *return_item.kind().expect_type().kind() {
            ast::FunctionRetTy::Default(ctx.span())
        } else {
            ast::FunctionRetTy::Ty(return_item.to_rust_ty_or_opaque(ctx, &()))
        }
    }
    pub fn fnsig_arguments(ctx: &BindgenContext, sig: &FunctionSig)
     -> Vec<ast::Arg> {
        use super::ToPtr;
        let mut unnamed_arguments = 0;
        sig.argument_types().iter().map(|&(ref name, ty)|
                                            {
                                                let arg_item =
                                                    ctx.resolve_item(ty);
                                                let arg_ty =
                                                    arg_item.kind().expect_type();
                                                let arg_ty =
                                                    match *arg_ty.canonical_type(ctx).kind()
                                                        {
                                                        TypeKind::Array(t, _)
                                                        => {
                                                            t.to_rust_ty_or_opaque(ctx,
                                                                                   &()).to_ptr(ctx.resolve_type(t).is_const(),
                                                                                               ctx.span())
                                                        }
                                                        TypeKind::Pointer(inner)
                                                        => {
                                                            let inner =
                                                                ctx.resolve_item(inner);
                                                            let inner_ty =
                                                                inner.expect_type();
                                                            if let TypeKind::ObjCInterface(_)
                                                                   =
                                                                   *inner_ty.canonical_type(ctx).kind()
                                                                   {
                                                                {
                                                                    let ext_cx =
                                                                        &*ctx.ext_cx();
                                                                    ::quasi::parse_ty_panic(&mut ::syntax::parse::new_parser_from_tts(ext_cx.parse_sess(),
                                                                                                                                      {
                                                                                                                                          #[allow(unused_imports)]
                                                                                                                                          use quasi::IntoWrappedIterator;
                                                                                                                                          #[allow(unused_imports)]
                                                                                                                                          use quasi::IntoWrappedRepeat;
                                                                                                                                          let _sp =
                                                                                                                                              ext_cx.call_site();
                                                                                                                                          let mut tt =
                                                                                                                                              ::std::vec::Vec::new();
                                                                                                                                          tt.push(::syntax::tokenstream::TokenTree::Token(_sp,
                                                                                                                                                                                          ::syntax::parse::token::Ident(ext_cx.ident_of("id"))));
                                                                                                                                          tt
                                                                                                                                      }))
                                                                }
                                                            } else {
                                                                arg_item.to_rust_ty_or_opaque(ctx,
                                                                                              &())
                                                            }
                                                        }
                                                        _ => {
                                                            arg_item.to_rust_ty_or_opaque(ctx,
                                                                                          &())
                                                        }
                                                    };
                                                let arg_name =
                                                    match *name {
                                                        Some(ref name) =>
                                                        ctx.rust_mangle(name).into_owned(),
                                                        None => {
                                                            unnamed_arguments
                                                                += 1;
                                                            format!("arg{}" ,
                                                                    unnamed_arguments)
                                                        }
                                                    };
                                                assert!(! arg_name . is_empty
                                                        (  ));
                                                ast::Arg{ty: arg_ty,
                                                         pat:
                                                             aster::AstBuilder::new().pat().id(arg_name),
                                                         id:
                                                             ast::DUMMY_NODE_ID,}
                                            }).collect::<Vec<_>>()
    }
}
