use std::process::Command;
use std::ops::Add;

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn main() {
    // macOS deps
    // brew install gettext automake autoconf libtool

    // Get current dir since build scripts are located in /target/debug|release
    let pwd = std::env::current_dir()
        .expect("Error getting current directory")
        .to_str()
        .expect("Error converting current directory to &str")
        .to_string()
        + "/vendor";

    // OpenSSL building and linking
    Command::new("sh")
        .current_dir(&pwd.clone().add("/openssl"))
        .args(&[
            "./config --prefix=$PWD/dist no-shared no-dso no-zlib",
            "make depend",
            "make",
            "make install"])
        .status()
        .expect("Error building OpenSSL");
    println!("cargo:rustc-link-search=native=vendor/openssl/dist/lib");
    println!("cargo:rustc-link-lib=static=crypto");
    println!("cargo:rustc-link-lib=static=ssl");

    // Libevent building and linking
    Command::new("sh")
        .current_dir(&pwd.clone().add("/libevent"))
        .args(&[
            "./autogen.sh",
            "./configure --prefix=$PWD/dist --disable-shared --enable-static --with-pic",
            "make",
            "make install"])
        .status()
        .expect("Error building Libevent");
    println!("cargo:rustc-link-search=native=vendor/libevent/dist/lib");
    println!("cargo:rustc-link-lib=static=event");

    // zlib building and linking
    Command::new("sh")
        .current_dir(&pwd.clone().add("/zlib"))
        .args(&[
            "./configure --prefix=$PWD/dist",
            "make",
            "make install"])
        .status()
        .expect("Error building zlib");
    println!("cargo:rustc-link-search=native=vendor/zlib/dist/lib");
    println!("cargo:rustc-link-lib=static=z");

    // xz building and linking
    Command::new("sh")
        .current_dir(&pwd.clone().add("/xz"))
        .args(&[
            "./autogen.sh",
            "./configure --prefix=$PWD/dist \
                --disable-shared \
                --enable-static \
                --disable-doc \
                --disable-scripts \
                --disable-xz \
                --disable-xzdec \
                --disable-lzmadec \
                --disable-lzmainfo \
                --disable-lzma-links",
            "make",
            "make install"])
        .status()
        .expect("Error building xz");
    println!("cargo:rustc-link-search=native=vendor/xz/dist/lib");
    println!("cargo:rustc-link-lib=static=lzma");

    // Tor building and linking
    Command::new("sh")
        .current_dir(&pwd.clone().add("/tor"))
        .args(&[
            "./autogen.sh",
            "LIBS=-lcrypt32 ./configure --prefix=$PWD/dist \
                --disable-gcc-hardening \
                --enable-static-tor \
                --enable-static-libevent \
                --with-libevent-dir=$PWD/../libevent/dist \
                --enable-static-openssl \
                --with-openssl-dir=$PWD/../openssl/dist \
                --enable-static-zlib \
                --with-zlib-dir=$PWD/../openssl/dist \
                --disable-system-torrc \
                --disable-asciidoc",
            "ln -s $PWD/../zlib/dist/lib/libz.a $PWD/../openssl/dist/lib/libz.a",
            "make",
            "make install"])
        .status()
        .expect("Error building Tor");
    println!("cargo:rustc-link-search=native=vendor/tor/src/ext/ed25519/ref10");
    println!("cargo:rustc-link-lib=static=ed25519_ref10");

    println!("cargo:rustc-link-search=native=vendor/tor/src/ext/ed25519/donna");
    println!("cargo:rustc-link-lib=static=ed25519_donna");

    println!("cargo:rustc-link-search=native=vendor/tor/src/trunnel");
    println!("cargo:rustc-link-lib=static=or-trunnel");

    println!("cargo:rustc-link-search=native=vendor/tor/src/ext/keccak-tiny");
    println!("cargo:rustc-link-lib=static=keccak-tiny");

    println!("cargo:rustc-link-search=native=vendor/tor/src/common");
    println!("cargo:rustc-link-lib=static=curve25519_donna");
    println!("cargo:rustc-link-lib=static=or");
    println!("cargo:rustc-link-lib=static=or-crypto");
    println!("cargo:rustc-link-lib=static=or-ctime");
    println!("cargo:rustc-link-lib=static=or-event");

    println!("cargo:rustc-link-search=native=vendor/tor/src/or");
    println!("cargo:rustc-link-lib=static=tor");
    
    println!("cargo:rustc-link-lib=ws2_32");
    println!("cargo:rustc-link-lib=crypt32");
    println!("cargo:rustc-link-lib=gdi32");
}