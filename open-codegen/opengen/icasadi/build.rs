use cc;
use std::path::Path;

fn main() {
    // Sanity checks to get better error messages
    assert!(
        Path::new("extern/auto_casadi_constraints_type_penalty.c").exists(),
        "extern/auto_casadi_cost.c is missing"
    );
    assert!(
        Path::new("extern/auto_casadi_cost.c").exists(),
        "extern/auto_casadi_cost.c is missing"
    );
    assert!(
        Path::new("extern/auto_casadi_grad.c").exists(),
        "extern/auto_casadi_grad.c is missing"
    );

    cc::Build::new()
        .flag_if_supported("-Wall")
        .flag_if_supported("-Wpedantic")
        .flag_if_supported("-Wno-long-long")
        .flag_if_supported("-Wno-unused-parameter")
        .pic(true)
        .include("src")
        .file("extern/auto_casadi_cost.c")
        .file("extern/auto_casadi_grad.c")
        .file("extern/auto_casadi_constraints_type_penalty.c")
        .compile("icasadi");

    // Rerun if these autogenerated files change
    println!("cargo:rerun-if-changed=extern/auto_casadi_cost.c");
    println!("cargo:rerun-if-changed=extern/auto_casadi_grad.c");
    println!("cargo:rerun-if-changed=extern/auto_casadi_constraints_type_penalty.c");
}
