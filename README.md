# proc-macro ICE

Produces ICE as result of [std::thread::LocalKey](https://doc.rust-lang.org/std/thread/struct.LocalKey.html) usage during proc-macro code generation.

## Run:

```shell
RUST_BACKTRACE=full cargo run
```

## Output: 

```
thread 'rustc' panicked at 'use-after-free in `proc_macro` handle', src/libcore/option.rs:1166:5
stack backtrace:
   0:        0x10dec0c92 - <unknown>
   1:        0x10dec095d - <unknown>
   2:        0x10cc12183 - <unknown>
   3:        0x10dec1551 - <unknown>
   4:        0x10dec0fad - <unknown>
   5:        0x10dec0e99 - <unknown>
   6:        0x10deecc12 - <unknown>
   7:        0x10deecd2e - <unknown>
   8:        0x10cd91443 - <unknown>
   9:        0x10ce6c815 - <unknown>
  10:        0x10ded07cf - <unknown>
  11:        0x10cdb62f8 - <unknown>
  12:        0x10cdb3b13 - <unknown>
  13:        0x1154b4477 - proc_macro::bridge::closure::Closure<A,R>::call::h2a09b3facd4bf305
                               at src/libproc_macro/bridge/closure.rs:30
  14:        0x1154b4477 - proc_macro::bridge::client::Punct::with_span::{{closure}}::h0b5f924c6bd12c5e
                               at src/libproc_macro/bridge/client.rs:236
  15:        0x1154b4477 - proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::with::{{closure}}::hb805151e6e26e972
                               at src/libproc_macro/bridge/client.rs:320
  16:        0x1154b4477 - proc_macro::bridge::client::BridgeState::with::{{closure}}::{{closure}}::hfd794fa692de71cc
                               at src/libproc_macro/bridge/client.rs:285
  17:        0x1154b4477 - proc_macro::bridge::scoped_cell::ScopedCell<T>::replace::hdb8a9baf8428bfc7
                               at src/libproc_macro/bridge/scoped_cell.rs:74
  18:        0x1154b4477 - proc_macro::bridge::client::BridgeState::with::{{closure}}::hc33b9ff1c5262206
                               at src/libproc_macro/bridge/client.rs:283
  19:        0x1154b4477 - std::thread::local::LocalKey<T>::try_with::he43cb685e78f8194
                               at /rustc/625451e376bb2e5283fc4741caa0a3e8a2ca4d54/src/libstd/thread/local.rs:262
  20:        0x1154b4477 - std::thread::local::LocalKey<T>::with::ha9bc6c29706c90a4
                               at /rustc/625451e376bb2e5283fc4741caa0a3e8a2ca4d54/src/libstd/thread/local.rs:239
  21:        0x1154b4477 - proc_macro::bridge::client::BridgeState::with::h5d9a6aeedbeda3e5
                               at src/libproc_macro/bridge/client.rs:282
  22:        0x1154b4477 - proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::with::hf42730f34fa863ad
                               at src/libproc_macro/bridge/client.rs:313
  23:        0x1154b4477 - proc_macro::bridge::client::Punct::with_span::h4e78f045758e95e5
                               at src/libproc_macro/bridge/client.rs:229
  24:        0x1154b4477 - proc_macro::Punct::set_span::hc9fd3affed3b2201
                               at src/libproc_macro/lib.rs:760
  25:        0x11548179c - proc_macro2::imp::into_compiler_token::hbcdf440978836135
                               at /Users/oleg/.cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.6/src/wrapper.rs:200
  26:        0x115459e30 - core::ops::function::FnMut::call_mut::h7f79f18633109a42
                               at /rustc/625451e376bb2e5283fc4741caa0a3e8a2ca4d54/src/libcore/ops/function.rs:152
  27:        0x11545c95a - <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::fold::{{closure}}::h45ed7e68f5cc36dd
                               at /rustc/625451e376bb2e5283fc4741caa0a3e8a2ca4d54/src/libcore/iter/adapters/mod.rs:622
  28:        0x115462613 - core::iter::traits::iterator::Iterator::fold::{{closure}}::h40cd83f4f5758f32
                               at /rustc/625451e376bb2e5283fc4741caa0a3e8a2ca4d54/src/libcore/iter/traits/iterator.rs:1783
  29:        0x11545f6c9 - core::iter::traits::iterator::Iterator::try_fold::h49200a73c20d9729
                               at /rustc/625451e376bb2e5283fc4741caa0a3e8a2ca4d54/src/libcore/iter/traits/iterator.rs:1671
  30:        0x11545f40c - core::iter::traits::iterator::Iterator::fold::hb3953cf8ca6e4519
                               at /rustc/625451e376bb2e5283fc4741caa0a3e8a2ca4d54/src/libcore/iter/traits/iterator.rs:1783
  31:        0x11545c817 - <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::fold::h7e14cd96ccc835ee
                               at /rustc/625451e376bb2e5283fc4741caa0a3e8a2ca4d54/src/libcore/iter/adapters/mod.rs:622
  32:        0x11545c337 - core::iter::traits::iterator::Iterator::for_each::h5b67fadcec3d052b
                               at /rustc/625451e376bb2e5283fc4741caa0a3e8a2ca4d54/src/libcore/iter/traits/iterator.rs:602
  33:        0x115457992 - <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::spec_extend::hdc71ec2d14b95f9a
                               at /rustc/625451e376bb2e5283fc4741caa0a3e8a2ca4d54/src/liballoc/vec.rs:1961
  34:        0x115457b79 - <alloc::vec::Vec<T> as core::iter::traits::collect::Extend<T>>::extend::h28b999588789ede1
                               at /rustc/625451e376bb2e5283fc4741caa0a3e8a2ca4d54/src/liballoc/vec.rs:1899
  35:        0x115458b6e - <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::hc213b664cc656deb
                               at /Users/oleg/.cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.6/src/wrapper.rs:259
  36:        0x11545e589 - <proc_macro2::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::ha35481113c9cc233
                               at /Users/oleg/.cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.6/src/lib.rs:204
  37:        0x11545ecae - <proc_macro2::TokenStream as quote::ext::TokenStreamExt>::append::h021e46d4243ef034
                               at /Users/oleg/.cargo/registry/src/github.com-1ecc6299db9ec823/quote-1.0.2/src/ext.rs:66
  38:        0x1153788cb - syn::token::printing::punct::hd9d7f53cb67e52af
                               at /Users/oleg/.cargo/registry/src/github.com-1ecc6299db9ec823/syn-1.0.7/src/token.rs:926
  39:        0x115407f15 - <syn::token::Colon2 as quote::to_tokens::ToTokens>::to_tokens::h10509758c84e5d32
                               at /Users/oleg/.cargo/registry/src/github.com-1ecc6299db9ec823/syn-1.0.7/src/token.rs:406
  40:        0x115444d99 - <core::option::Option<T> as quote::to_tokens::ToTokens>::to_tokens::h07eab989b0097806
                               at /Users/oleg/.cargo/registry/src/github.com-1ecc6299db9ec823/quote-1.0.2/src/to_tokens.rs:112
  41:        0x1153fa5a6 - syn::path::printing::<impl quote::to_tokens::ToTokens for syn::path::Path>::to_tokens::h0d618cfaecc9b64b
                               at /Users/oleg/.cargo/registry/src/github.com-1ecc6299db9ec823/syn-1.0.7/src/path.rs:571
  42:        0x115363200 - <&T as quote::to_tokens::ToTokens>::to_tokens::hd58e40ac37faeb17
                               at /Users/oleg/.cargo/registry/src/github.com-1ecc6299db9ec823/quote-1.0.2/src/to_tokens.rs:81
  43:        0x11535ec0d - proc_macro_bug_impl::some_macro::{{closure}}::h1dd44483f10a11b1
                               at proc-macro-bug-impl/src/lib.rs:17
  44:        0x11536295b - std::thread::local::LocalKey<T>::try_with::h2fbc0c435b62453e
                               at /rustc/625451e376bb2e5283fc4741caa0a3e8a2ca4d54/src/libstd/thread/local.rs:262
  45:        0x115362847 - std::thread::local::LocalKey<T>::with::h363f3334befa7127
                               at /rustc/625451e376bb2e5283fc4741caa0a3e8a2ca4d54/src/libstd/thread/local.rs:239
  46:        0x11536300a - proc_macro_bug_impl::some_macro::h95ab1d18a4846ecb
                               at proc-macro-bug-impl/src/lib.rs:16
  47:        0x115496e2f - proc_macro::bridge::client::__run_expand2::{{closure}}::{{closure}}::h81965a9095ca117a
                               at src/libproc_macro/bridge/client.rs:412
  48:        0x115496e2f - proc_macro::bridge::scoped_cell::ScopedCell<T>::set::{{closure}}::h2fa15a9eb613d481
                               at src/libproc_macro/bridge/scoped_cell.rs:79
  49:        0x115496e2f - proc_macro::bridge::scoped_cell::ScopedCell<T>::replace::hda3ccab505338f1f
                               at src/libproc_macro/bridge/scoped_cell.rs:74
  50:        0x115496e2f - proc_macro::bridge::scoped_cell::ScopedCell<T>::set::he215425adea53b80
                               at src/libproc_macro/bridge/scoped_cell.rs:79
  51:        0x115496e2f - proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::{{closure}}::h287392c24c6cae23
                               at src/libproc_macro/bridge/client.rs:309
  52:        0x115496e2f - std::thread::local::LocalKey<T>::try_with::he1fcfde5827fd98b
                               at /rustc/625451e376bb2e5283fc4741caa0a3e8a2ca4d54/src/libstd/thread/local.rs:262
  53:        0x115496e2f - std::thread::local::LocalKey<T>::with::hf582851647ec70eb
                               at /rustc/625451e376bb2e5283fc4741caa0a3e8a2ca4d54/src/libstd/thread/local.rs:239
  54:        0x115496e2f - proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::h55ef49da95306a67
                               at src/libproc_macro/bridge/client.rs:309
  55:        0x115496e2f - proc_macro::bridge::client::__run_expand2::{{closure}}::h10af6c7898bf021d
                               at src/libproc_macro/bridge/client.rs:404
  56:        0x115496e2f - <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::h173f3e925c7c3807
                               at /rustc/625451e376bb2e5283fc4741caa0a3e8a2ca4d54/src/libstd/panic.rs:315
  57:        0x115496e2f - std::panicking::try::do_call::h27a4307415f1e29a
                               at /rustc/625451e376bb2e5283fc4741caa0a3e8a2ca4d54/src/libstd/panicking.rs:296
  58:        0x1154f0cef - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:80
  59:        0x11549baa7 - std::panicking::try::h2d9d28f402d5960a
                               at /rustc/625451e376bb2e5283fc4741caa0a3e8a2ca4d54/src/libstd/panicking.rs:275
  60:        0x11549baa7 - std::panic::catch_unwind::hbaaab21c3f42de29
                               at /rustc/625451e376bb2e5283fc4741caa0a3e8a2ca4d54/src/libstd/panic.rs:394
  61:        0x11549baa7 - proc_macro::bridge::client::__run_expand2::ha7b5f54eaddb04a8
                               at src/libproc_macro/bridge/client.rs:403
  62:        0x10cdb292e - <unknown>
  63:        0x10ceb983f - <unknown>
  64:        0x10ceafff1 - <unknown>
  65:        0x10ceacfa4 - <unknown>
  66:        0x10b5a1cad - <unknown>
  67:        0x10b59d8ad - <unknown>
  68:        0x10b55db16 - <unknown>
  69:        0x10b5d5058 - <unknown>
  70:        0x10b5aad47 - <unknown>
  71:        0x10b55acd9 - <unknown>
  72:        0x10b54fe6b - <unknown>
  73:        0x10b61508c - <unknown>
  74:        0x10b4832b2 - <unknown>
  75:        0x10b4b6d84 - <unknown>
  76:        0x10b4cc938 - <unknown>
  77:        0x10b45e807 - <unknown>
  78:        0x10ded07cf - <unknown>
  79:        0x10b4851d7 - <unknown>
  80:        0x10dea462e - <unknown>
  81:        0x10decf5ce - <unknown>
  82:     0x7fff5f4402eb - <unknown>
  83:     0x7fff5f443249 - <unknown>
query stack during panic:
end of query stack
error: custom attribute panicked
 --> proc-macro-bug/src/main.rs:7:1
  |
7 | #[some_macro(0)]
  | ^^^^^^^^^^^^^^^^
  |
  = help: message: use-after-free in `proc_macro` handle
```