use std::path::Path;

pub struct Template {
    pub name: &'static str,
    pub create_fn: fn(&Path, &str)
}

pub const TEMPLATES: &[Template] = &[
    Template {
        name: "fastapi",
        create_fn: |path, _name| fastapi::create_files(path)
    },
    Template {
        name: "fiber",
        create_fn: |path, name| fiber::create_files(path, name)
    },
    Template {
        name: "gcp-terra-go",
        create_fn: |path, name| gcp_terra_go::create_files(path, name)
    },
    Template {
        name: "gcp-terra-python",
        create_fn: |path, name| gcp_terra_python::create_files(path, name)
    },
    Template{
        name: "gcp-terra-js",
        create_fn: |path, name| gcp_terra_js::create_files(path, name)
    },
    Template {
        name: "express-ts",
        create_fn: |path, name| express_ts::create_files(path, name)
    },
    Template {
        name: "express-js",
        create_fn: |path, name| express_js::create_files(path, name)
    }
];


pub mod fastapi;
pub mod fiber;
pub mod gcp_terra_go;
pub mod gcp_terra_python;
pub mod gcp_terra_js;
pub mod express_ts;
pub mod express_js;