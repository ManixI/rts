use syn::Meta;

// see https://github.com/jbaublitz/getset/blob/main/src/generate.rs

pub struct GenParams {
    pub mode: GenMode,
    pub global_attr: Option<Meta>
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum GenMode {
    Get,
    Set,
}

impl GenMode {

}