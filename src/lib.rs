use std::path::Path;
use std::{collections::HashMap, io::Read};

use json_comments::StripComments;
use regex::Regex;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

use thiserror::Error;

pub type Result<T, E = ConfigError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Could not parse configuration file")]
    ParseError(#[from] serde_json::Error),
    #[error("Could not read file")]
    CouldNotFindFile(#[from] std::io::Error),
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TsConfig {
    exclude: Option<Vec<String>>,
    extends: Option<String>,
    files: Option<Vec<String>>,
    include: Option<Vec<String>>,
    references: Option<References>,
    type_acquisition: Option<TypeAcquisition>,
    compiler_options: Option<CompilerOptions>,
}

impl TsConfig {
    pub fn parse_file<P: AsRef<Path>>(path: &P) -> Result<TsConfig> {
        let values = parse_file_to_value(path)?;
        let cfg = serde_json::from_value(values)?;
        Ok(cfg)
    }

    pub fn parse_str(json: &str) -> Result<TsConfig> {
        // Remove trailing commas from objects.
        let re = Regex::new(r",(?P<valid>\s*})").unwrap();
        let mut stripped = String::with_capacity(json.len());
        StripComments::new(json.as_bytes()).read_to_string(&mut stripped)?;
        let stripped = re.replace_all(&stripped, "$valid");
        let r: TsConfig = serde_json::from_str(&stripped)?;
        Ok(r)
    }
}

fn merge(a: &mut Value, b: Value) {
    match (a, b) {
        (&mut Value::Object(ref mut a), Value::Object(b)) => {
            for (k, v) in b {
                merge(a.entry(k).or_insert(Value::Null), v);
            }
        }
        (a, b) => {
            if let Value::Null = a {
                *a = b;
            }
        }
    }
}

pub fn parse_file_to_value<P: AsRef<Path>>(path: &P) -> Result<Value> {
    let s = std::fs::read_to_string(path)?;
    let mut value = parse_to_value(&s)?;

    if let Value::String(s) = &value["extends"] {
        let extends_path = path
            .as_ref()
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .join(s);
        let extends_value = parse_file_to_value(&extends_path)?;
        merge(&mut value, extends_value);
    }

    Ok(value)
}

pub fn parse_to_value(json: &str) -> Result<Value> {
    // Remove trailing commas from objects.
    let re = Regex::new(r",(?P<valid>\s*})").unwrap();
    let mut stripped = String::with_capacity(json.len());
    StripComments::new(json.as_bytes()).read_to_string(&mut stripped)?;
    let stripped = re.replace_all(&stripped, "$valid");
    let r: Value = serde_json::from_str(&stripped)?;
    Ok(r)
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum References {
    Bool(bool),
    References(Vec<Reference>),
}

#[derive(Deserialize, Debug, Clone)]
pub struct Reference {
    path: String,
    prepend: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub enum TypeAcquisition {
    Bool(bool),
    Object {
        enable: bool,
        include: Option<Vec<String>>,
        exclude: Option<Vec<String>>,
        disable_filename_based_type_acquisition: Option<bool>,
    },
}

/// These options make up the bulk of TypeScript’s configuration and it covers how the language should work.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CompilerOptions {
    allow_js: Option<bool>,
    check_js: Option<bool>,
    composite: Option<bool>,
    declaration: Option<bool>,
    declaration_map: Option<bool>,
    downlevel_iteration: Option<bool>,
    import_helpers: Option<bool>,
    incremental: Option<bool>,
    isolated_modules: Option<bool>,
    jsx: Option<Jsx>,
    lib: Option<Vec<Lib>>,
    module: Option<Module>,
    no_emit: Option<bool>,
    out_dir: Option<String>,
    out_file: Option<String>,
    remove_comments: Option<bool>,
    root_dir: Option<String>,
    source_map: Option<bool>,
    target: Option<Target>,
    ts_build_info_file: Option<String>,
    always_strict: Option<bool>,
    no_implicit_any: Option<bool>,
    no_implicit_this: Option<bool>,
    strict: Option<bool>,
    strict_bind_call_apply: Option<bool>,
    strict_function_types: Option<bool>,
    strict_null_checks: Option<bool>,
    strict_property_initialization: Option<bool>,
    allow_synthetic_default_imports: Option<bool>,
    allow_umd_global_access: Option<bool>,
    base_url: Option<String>,
    es_module_interop: Option<bool>,
    module_resolution: Option<ModuleResolutionMode>,
    paths: Option<HashMap<String, Vec<String>>>,
    preserve_symlinks: Option<bool>,
    root_dirs: Option<Vec<String>>,
    type_roots: Option<Vec<String>>,
    types: Option<Vec<String>>,
    inline_source_map: Option<bool>,
    inline_sources: Option<bool>,
    map_root: Option<String>,
    source_root: Option<String>,
    no_fallthrough_cases_in_switch: Option<bool>,
    no_implicit_returns: Option<bool>,
    no_property_access_from_index_signature: Option<bool>,
    no_unchecked_indexed_access: Option<bool>,
    no_unused_locals: Option<bool>,
    emit_decorator_metadata: Option<bool>,
    experimental_decorators: Option<bool>,
    allow_unreachable_code: Option<bool>,
    allow_unused_labels: Option<bool>,
    assume_changes_only_affect_direct_dependencies: Option<bool>,
    #[deprecated]
    charset: Option<String>,
    declaration_dir: Option<String>,
    #[deprecated]
    diagnostics: Option<bool>,
    disable_referenced_project_load: Option<bool>,
    disable_size_limit: Option<bool>,
    disable_solution_searching: Option<bool>,
    disable_source_of_project_reference_redirect: Option<bool>,
    #[serde(rename = "emitBOM")]
    emit_bom: Option<bool>,
    emit_declaration_only: Option<bool>,
    explain_files: Option<bool>,
    extended_diagnostics: Option<bool>,
    force_consistent_casing_in_file_names: Option<bool>,
    // XXX: Is generateCpuProfile available from tsconfig? Or just the CLI?
    generate_cpu_profile: Option<bool>,

    imports_not_used_as_values: Option<String>,
    jsx_factory: Option<String>,
    jsx_fragment_factory: Option<String>,
    jsx_import_source: Option<String>,

    keyof_strings_only: Option<bool>,
    list_emitted_files: Option<bool>,
    list_files: Option<bool>,
    max_node_module_js_depth: Option<u32>,
    no_emit_helpers: Option<bool>,
    no_emit_on_error: Option<bool>,
    no_error_truncation: Option<bool>,
    no_implicit_use_strict: Option<bool>,
    no_lib: Option<bool>,
    no_resolve: Option<bool>,
    no_strict_generic_checks: Option<bool>,
    #[deprecated]
    out: Option<String>,
    preserve_const_enums: Option<bool>,
    react_namespace: Option<String>,
    resolve_json_module: Option<bool>,
    skip_default_lib_check: Option<bool>,
    skip_lib_check: Option<bool>,
    strip_internal: Option<bool>,
    suppress_excess_property_errors: Option<bool>,
    suppress_implicit_any_index_errors: Option<bool>,
    trace_resolution: Option<bool>,
    use_define_for_class_fields: Option<bool>,
    preserve_watch_output: Option<bool>,
    pretty: Option<bool>,
    fallback_polling: Option<String>,
    watch_directory: Option<String>,
    watch_file: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum ModuleResolutionMode {
    #[serde(rename = "node")]
    Node,
    #[serde(rename = "classic")]
    Classic,
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum Jsx {
    React,
    ReactJsx,
    ReactJsxdev,
    ReactNative,
    Preserve,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Target {
    Es3,
    Es5,
    Es2015,
    Es6,
    Es2016,
    Es7,
    Es2017,
    Es2018,
    Es2019,
    Es2020,
    EsNext,
    Other(String),
}
impl<'de> Deserialize<'de> for Target {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let s = s.to_uppercase();

        let d = match s.as_str() {
            "ES5" => Target::Es5,
            "ES2015" => Target::Es2015,
            "ES6" => Target::Es6,
            "ES2016" => Target::Es2016,
            "ES7" => Target::Es7,
            "ES2017" => Target::Es2017,
            "ES2018" => Target::Es2018,
            "ES2019" => Target::Es2019,
            "ES2020" => Target::Es2020,
            "ESNEXT" => Target::EsNext,
            other => Target::Other(other.to_string()),
        };

        Ok(d)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Lib {
    Es5,
    Es2015,
    Es6,
    Es2016,
    Es7,
    Es2017,
    Es2018,
    Es2019,
    Es2020,
    EsNext,
    Dom,
    WebWorker,
    ScriptHost,
    DomIterable,
    Es2015Core,
    Es2015Generator,
    Es2015Iterable,
    Es2015Promise,
    Es2015Proxy,
    Es2015Reflect,
    Es2015Symbol,
    Es2015SymbolWellKnown,
    Es2016ArrayInclude,
    Es2017Object,
    Es2017Intl,
    Es2017SharedMemory,
    Es2017String,
    Es2017TypedArrays,
    Es2018Intl,
    Es2018Promise,
    Es2018RegExp,
    Es2019Array,
    Es2019Object,
    Es2019String,
    Es2019Symbol,
    Es2020String,
    Es2020SymbolWellknown,
    EsNextAsyncIterable,
    EsNextArray,
    EsNextIntl,
    EsNextSymbol,
    Other(String),
}

impl<'de> Deserialize<'de> for Lib {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let s = s.to_uppercase();

        let d = match s.as_str() {
            "ES5" => Lib::Es5,
            "ES2015" => Lib::Es2015,
            "ES6" => Lib::Es6,
            "ES2016" => Lib::Es2016,
            "ES7" => Lib::Es7,
            "ES2017" => Lib::Es2017,
            "ES2018" => Lib::Es2018,
            "ES2019" => Lib::Es2019,
            "ES2020" => Lib::Es2020,
            "ESNext" => Lib::EsNext,
            "DOM" => Lib::Dom,
            "WEBWORKER" => Lib::WebWorker,
            "SCRIPTHOST" => Lib::ScriptHost,
            "DOM.ITERABLE" => Lib::DomIterable,
            "ES2015.CORE" => Lib::Es2015Core,
            "ES2015.GENERATOR" => Lib::Es2015Generator,
            "ES2015.ITERABLE" => Lib::Es2015Iterable,
            "ES2015.PROMISE" => Lib::Es2015Promise,
            "ES2015.PROXY" => Lib::Es2015Proxy,
            "ES2015.REFLECT" => Lib::Es2015Reflect,
            "ES2015.SYMBOL" => Lib::Es2015Symbol,
            "ES2015.SYMBOL.WELLKNOWN" => Lib::Es2015SymbolWellKnown,
            "ES2015.ARRAY.INCLUDE" => Lib::Es2016ArrayInclude,
            "ES2015.OBJECT" => Lib::Es2017Object,
            "ES2017INTL" => Lib::Es2017Intl,
            "ES2015.SHAREDMEMORY" => Lib::Es2017SharedMemory,
            "ES2017.STRING" => Lib::Es2017String,
            "ES2017.TYPEDARRAYS" => Lib::Es2017TypedArrays,
            "ES2018.INTL" => Lib::Es2018Intl,
            "ES2018.PROMISE" => Lib::Es2018Promise,
            "ES2018.REGEXP" => Lib::Es2018RegExp,
            "ES2019.ARRAY" => Lib::Es2019Array,
            "ES2019.OBJECT" => Lib::Es2019Object,
            "ES2019.STRING" => Lib::Es2019String,
            "ES2019.SYMBOL" => Lib::Es2019Symbol,
            "ES2020.STRING" => Lib::Es2020String,
            "ES2020.SYMBOL.WELLKNOWN" => Lib::Es2020SymbolWellknown,
            "ESNEXT.ASYNCITERABLE" => Lib::EsNextAsyncIterable,
            "ESNEXT.ARRAY" => Lib::EsNextArray,
            "ESNEXT.INTL" => Lib::EsNextIntl,
            "ESNEXT.SYMBOL" => Lib::EsNextSymbol,
            other => Lib::Other(other.to_string()),
        };

        Ok(d)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Module {
    CommonJs,
    Es6,
    Es2015,
    Es2020,
    None,
    Umd,
    Amd,
    System,
    EsNext,
    Other(String),
}

impl<'de> Deserialize<'de> for Module {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let s = s.to_uppercase();

        let r = match s.as_str() {
            "COMMONJS" => Module::CommonJs,
            "ESNEXT" => Module::EsNext,
            "ES6" => Module::Es6,
            "ES2015" => Module::Es2015,
            "ES2020" => Module::Es2020,
            "NONE" => Module::None,
            "UMD" => Module::Umd,
            "AMD" => Module::Amd,
            "SYSTEM" => Module::System,
            other => Module::Other(other.to_string()),
        };

        Ok(r)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn parse_jsx() {
        let json = r#"{"compilerOptions": {"jsx": "react-jsx"}}"#;

        let config = TsConfig::parse_str(json).unwrap();
        assert_eq!(config.compiler_options.unwrap().jsx, Some(Jsx::ReactJsx));
    }

    #[test]
    fn parse_paths() {
        let json = r#"{
        "compilerOptions": {
            "baseUrl": "src",
            "paths": {
                "tests/*": ["tests/*"],
                "blah": ["bloop"]
            }
        }
    }
        
        "#;

        let config = TsConfig::parse_str(json).unwrap();
        assert_eq!(
            config
                .compiler_options
                .unwrap()
                .paths
                .unwrap()
                .get("tests/*"),
            Some(&vec!["tests/*".to_string()])
        );
    }

    #[test]
    fn parse_empty() {
        TsConfig::parse_str("{}").unwrap();
        TsConfig::parse_str(r#"{"compilerOptions": {}}"#).unwrap();
    }

    #[test]
    fn parse_default() {
        let json = include_str!("../test/tsconfig.default.json");
        TsConfig::parse_str(json).unwrap();
    }

    #[test]
    fn parse_common_tsconfig() {
        let json = include_str!("../test/tsconfig.common.json");
        TsConfig::parse_str(json).unwrap();
    }

    #[test]
    fn parse_complete_tsconfig() {
        let json = include_str!("../test/tsconfig.complete.json");
        TsConfig::parse_str(json).unwrap();
    }

    #[test]
    fn ignores_invalid_fields() {
        let json = r#"{"bleep": true, "compilerOptions": {"someNewUnsupportedProperty": false}}"#;
        TsConfig::parse_str(json).unwrap();
    }

    #[test]
    fn ignores_dangling_commas() {
        let json = r#"{"compilerOptions": {"noImplicitAny": false,"explainFiles": true,}}"#;
        let cfg = TsConfig::parse_str(json).unwrap();
        assert_eq!(cfg.compiler_options.unwrap().explain_files.unwrap(), true);

        let json = r#"{"compilerOptions": {"noImplicitAny": false,"explainFiles": true, }}"#;
        let cfg = TsConfig::parse_str(json).unwrap();
        assert_eq!(cfg.compiler_options.unwrap().explain_files.unwrap(), true);

        let json = r#"{"compilerOptions": {"noImplicitAny": false,"explainFiles": true,
    }}"#;
        let cfg = TsConfig::parse_str(json).unwrap();
        assert_eq!(cfg.compiler_options.unwrap().explain_files.unwrap(), true);
    }

    #[test]
    fn merge_two_configs() {
        let json_1 = r#"{"compilerOptions": {"jsx": "react", "noEmit": true,}}"#;
        let json_2 = r#"{"compilerOptions": {"jsx": "preserve", "removeComments": true}}"#;

        let mut value1: Value = parse_to_value(json_1).unwrap();
        let value2: Value = parse_to_value(json_2).unwrap();

        merge(&mut value1, value2);

        let value: TsConfig = serde_json::from_value(value1).unwrap();

        assert_eq!(
            value.clone().compiler_options.unwrap().jsx,
            Some(Jsx::React)
        );
        assert_eq!(value.clone().compiler_options.unwrap().no_emit, Some(true));
        assert_eq!(value.compiler_options.unwrap().remove_comments, Some(true));
    }

    #[test]
    fn parse_basic_file() {
        let path = Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("test/tsconfig.default.json");
        let config = TsConfig::parse_file(&path).unwrap();

        assert_eq!(
            config.compiler_options.clone().unwrap().target,
            Some(Target::Es5)
        );
        assert_eq!(
            config.compiler_options.clone().unwrap().module,
            Some(Module::CommonJs)
        );
        assert_eq!(config.compiler_options.unwrap().strict, Some(true));
    }

    #[test]
    fn parse_inheriting_file() {
        let path = Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("test/tsconfig.inherits.json");
        let config = TsConfig::parse_file(&path).unwrap();

        assert_eq!(
            config
                .compiler_options
                .clone()
                .unwrap()
                .use_define_for_class_fields,
            Some(false)
        );

        assert_eq!(
            config.compiler_options.clone().unwrap().declaration,
            Some(true)
        );

        assert_eq!(
            config.compiler_options.unwrap().trace_resolution,
            Some(false)
        );
    }

    #[test]
    fn parse_inheritance_chain() {
        let path = Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("test/a/tsconfig.inherits_again.json");
        let config = TsConfig::parse_file(&path).unwrap();

        assert_eq!(
            config
                .compiler_options
                .clone()
                .unwrap()
                .use_define_for_class_fields,
            Some(false)
        );

        assert_eq!(
            config.compiler_options.clone().unwrap().declaration,
            Some(true)
        );

        assert_eq!(
            config.compiler_options.clone().unwrap().trace_resolution,
            Some(false)
        );

        assert_eq!(config.compiler_options.unwrap().jsx, Some(Jsx::ReactNative));
    }
}
