// assembly-output: bpf-linker
// compile-flags: --crate-type bin -C link-arg=--emit=obj -C debuginfo=2 -C link-arg=--log-level=debug -C link-arg=--log-file=/tmp/linker.log
#![no_std]
#![no_main]

// aux-build: loop-panic-handler.rs
extern crate loop_panic_handler;

// aux-build: dep-exports.rs
extern crate dep_exports as dep;

pub use dep::dep_public_symbol as local_re_exported;

#[no_mangle]
fn local_no_mangle() -> u8 {
    local_public()
}

#[inline(never)]
pub fn local_public() -> u8 {
    dep::dep_public_symbol()
}

// CHECK: FUNC 'local_no_mangle' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC '{{.*}}local_public{{.*}}' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC '{{.*}}dep_public_symbol{{.*}}' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'dep_no_mangle' type_id={{[0-9]+}} linkage=global
