use core::time::Duration;
use js_playground::deno_core::extension;
use js_playground::{json_args, FunctionArguments, Module, ModuleHandle, Runtime, RuntimeOptions};
use once_cell::sync::OnceCell;
use std::cell::RefCell;

use super::extension::Extension;

// Create a thread-local version of the runtime
// This should allow the following to be enforced:
// - Runtime is not sent between threads
// - Runtime is only initialized once
// - Runtime is never accessed concurrently
thread_local! {
    static RUNTIME_CELL: OnceCell<RefCell<ExtensionsRuntime>> = OnceCell::new();
}

extension!(
    lavendeux,
    esm_entry_point = "ext:lavendeux/lavendeux.js",
    esm = [
        dir "src/extensions/js", "extension.js", "function.js", "value.js", "lavendeux.js",
    ],
);

const SCRIPT_TIMEOUT: u64 = 1000;
pub struct ExtensionsRuntime(Runtime);
impl ExtensionsRuntime {
    fn new() -> Self {
        Self(
            Runtime::new(RuntimeOptions {
                timeout: Duration::from_millis(SCRIPT_TIMEOUT),
                default_entrypoint: Some("extension".to_string()),
                extensions: vec![lavendeux::init_ops_and_esm()],
            })
            .expect("could not create a JS runtime for extensions"),
        )
    }

    /// Perform an operation on the runtime instance
    /// Will return T if we can get access to the runtime
    /// or panic went wrong
    pub fn with<T, F: FnMut(&mut ExtensionsRuntime) -> T>(mut callback: F) -> T {
        RUNTIME_CELL.with(|once_lock| {
            let rt_mut = once_lock.get_or_init(|| RefCell::new(ExtensionsRuntime::new()));
            let mut runtime = rt_mut.borrow_mut();
            runtime.reset();
            callback(&mut runtime)
        })
    }

    pub fn reset(&mut self) {
        self.0.reset()
    }

    pub fn load_module(&mut self, module: &Module) -> Result<ModuleHandle, js_playground::Error> {
        self.0.load_module(module)
    }

    pub fn call_function<T>(
        &mut self,
        context: &ModuleHandle,
        function: &str,
        args: &FunctionArguments,
    ) -> Result<T, js_playground::Error>
    where
        T: serde::de::DeserializeOwned,
    {
        self.0.call_function(&context, function, args)
    }

    pub fn load_extension(path: &str) -> Result<Extension, js_playground::Error> {
        let module = Module::load(path)?;
        ExtensionsRuntime::with(|runtime| runtime.get_extension_from_module(&module))
    }

    pub fn load_extensions(dir: &str) -> Vec<Result<Extension, js_playground::Error>> {
        match Module::load_dir(dir) {
            Ok(modules) => {
                let mut results: Vec<Result<Extension, js_playground::Error>> = Vec::new();
                for module in modules {
                    let extension = ExtensionsRuntime::with(|runtime| {
                        runtime.get_extension_from_module(&module)
                    });
                    results.push(extension);
                }

                results
            }
            Err(e) => vec![Err(e.into())],
        }
    }

    fn get_extension_from_module(
        &mut self,
        module: &Module,
    ) -> Result<Extension, js_playground::Error> {
        let context = self.0.load_module(module)?;
        let mut extension: Extension = self.0.call_entrypoint(&context, json_args!())?;
        extension.module = module.clone();
        Ok(extension)
    }
}
