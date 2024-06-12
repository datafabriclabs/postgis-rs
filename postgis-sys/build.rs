use std::{
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    assert!(Command::new("git")
        .args(&["submodule", "update", "--init", "--recursive"])
        .status()
        .is_ok());

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=postgis/postgis/mvt.c");
    println!("cargo:rerun-if-changed=foo.c");

    let src = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());

    assert!(Command::new("perl")
        .current_dir(src.join("postgis"))
        .arg("utils/repo_revision.pl")
        .status()
        .expect("should run ok")
        .success());

    // run autogen
    assert!(Command::new("sh")
        .current_dir(src.join("postgis"))
        .arg("autogen.sh")
        .status()
        .expect("should run ok")
        .success());

    assert!(Command::new("sh")
        .current_dir(src.join("postgis"))
        .arg("configure")
        .args([
            "--without-libintl-prefix",
            "--without-json",
            "--without-address-standardizer",
            "--without-topology",
            "--without-raster",
        ])
        .status()
        .expect("should run ok")
        .success());

    assert!(Command::new("sh")
        .current_dir(src.join("postgis"))
        .arg("config.status")
        .status()
        .expect("should run ok")
        .success());
    let mut c = Command::new("/usr/bin/make");
    c.current_dir(src.join("postgis/postgis"));
    c.arg("vector_tile.pb-c.h");

    assert!(c.status().unwrap().success());

    // for build c files
    let mut builder = cc::Build::new();

    let add_include = |builder: &mut cc::Build| {
        // yuyang:very stupid way to locate a header file
        let postgres_headers = vec![
            "/usr/include/postgresql/16/server",
            "/usr/include/postgresql/15/server",
            "/usr/include/postgressql",
        ];

        let mut postgres_header = Option::None;
        for h in postgres_headers.iter() {
            if Path::new(*h).exists() {
                postgres_header = Some(Path::new(*h));
                break;
            }
        }
        let postgres_header = postgres_header.expect("postgres dev must installed");

        builder.include(postgres_header);
        builder.include(src.join("postgis/liblwgeom"));
        builder.include(src.join("postgis/libpgcommon"));
        builder.include(src.join("postgis/deps/wagyu"));
        builder.include(src.join("postgis/deps/uthash/include"));
        builder.include(src.join("postgis/deps/wagyu/include/"));
    };

    add_include(&mut builder);

    builder
        .file(src.join("postgis/postgis/mvt.c"))
        .file(src.join("foo.c"))
        .compile("xxxyyy");
    // for build c++ files
    let mut builder2 = cc::Build::new();

    add_include(&mut builder2);
    builder2.cpp(true);
    builder2.file(src.join("postgis/deps/wagyu/lwgeom_wagyu.cpp"));

    builder2.compile("xxxyyy2");
}
