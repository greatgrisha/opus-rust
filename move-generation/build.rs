fn main() {
    // Required for PyO3 to build a native extension module
    pyo3_build_config::add_extension_module_link_args();
}
