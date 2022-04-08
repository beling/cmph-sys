use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=cmph");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_type("CMPH_HASH")
        .allowlist_type("CMPH_ALGO")
        .allowlist_var("cmph_hash_names")
        .allowlist_var("cmph_names")
        .allowlist_type("cmph_io_adapter_t")
        .allowlist_type("cmph_int8")
        .allowlist_type("cmph_uint8")
        .allowlist_type("cmph_int16")
        .allowlist_type("cmph_uint16")
        .allowlist_type("cmph_int32")
        .allowlist_type("cmph_uint32")
        .allowlist_function("cmph_io_nlfile_adapter")
        .allowlist_function("cmph_io_nlfile_adapter_destroy")
        .allowlist_function("cmph_io_nlnkfile_adapter")
        .allowlist_function("cmph_io_nlnkfile_adapter_destroy")
        .allowlist_function("cmph_io_vector_adapter")
        .allowlist_function("cmph_io_vector_adapter_destroy")
        .allowlist_function("cmph_io_byte_vector_adapter")
        .allowlist_function("cmph_io_byte_vector_adapter_destroy")
        .allowlist_function("cmph_io_struct_vector_adapter")
        .allowlist_function("cmph_io_struct_vector_adapter_destroy")
        .allowlist_type("cmph_config_t")
        .allowlist_function("cmph_config_new")
        .allowlist_function("cmph_config_set_hashfuncs")
        .allowlist_function("cmph_config_set_verbosity")
        .allowlist_function("cmph_config_set_graphsize")
        .allowlist_function("cmph_config_set_algo")
        .allowlist_function("cmph_config_set_tmp_dir")
        .allowlist_function("cmph_config_set_mphf_fd")
        .allowlist_function("cmph_config_set_b")
        .allowlist_function("cmph_config_set_keys_per_bin")
        .allowlist_function("cmph_config_set_memory_availability")
        .allowlist_function("cmph_config_destroy")
        .allowlist_type("cmph_t")
        .allowlist_function("cmph_new")
        .allowlist_function("cmph_search")
        .allowlist_function("cmph_size")
        .allowlist_function("cmph_destroy")
        .allowlist_function("cmph_dump")
        .allowlist_function("cmph_load")
        .allowlist_type("packed_mphf")
        .allowlist_function("cmph_pack")
        .allowlist_function("cmph_packed_size")
        .allowlist_function("cmph_search_packed")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}