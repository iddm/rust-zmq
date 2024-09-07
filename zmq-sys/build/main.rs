#[cfg(not(feature = "use_system_library"))]
pub fn configure() {
    // Note that by default `libzmq` builds without `libsodium` by instead
    // relying on `tweetnacl`. However since this `tweetnacl` [has never been
    // audited nor is ready for production](https://github.com/zeromq/libzmq/issues/3006),
    // we link against `libsodium` to enable `ZMQ_CURVE`.
    zeromq_src::Build::new().with_libsodium(None).build();
}

#[cfg(feature = "use_system_library")]
fn use_system_library() {
    // println!("cargo:rustc-flags=-L/usr/lib -llibzmq.a");
    println!("cargo:rustc-flags=-L/usr/lib -lzmq");

    // println!("cargo:rustc-flags=-L/usr/lib -lzmq -l:libzmq.a");
    // println!("cargo:rustc-flags=-L/usr/lib -l:libzmq.a");
    // println!("cargo:rustc-flags=-L/usr/lib -lzmq");
    // println!("cargo:rustc-flags=-l/usr/lib/libzmq.a");

    // println!("cargo:rustc-link-search=native=/usr/lib/");
    // println!("cargo:rustc-link-lib=static:+whole-archive=zmq");
}

fn main() {
    println!("cargo:rerun-if-changed=build/main.rs");
    println!("cargo:rerun-if-env-changed=PROFILE");

    #[cfg(not(feature = "use_system_library"))]
    configure();

    #[cfg(feature = "use_system_library")]
    use_system_library();
}
