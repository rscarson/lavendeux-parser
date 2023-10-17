use crate::Value;

use rustyscript::{json_args, Module};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::runtime::ExtensionsRuntime;

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct ExtensionFunction {
    pub returns: String,
    pub argument_types: Vec<String>,
    pub fname: String,
    pub ftype: String,
}

impl ExtensionFunction {
    fn decorator_signature(&self) -> String {
        format!(
            "[{}] @{}",
            self.argument_types
                .get(0)
                .unwrap_or(&"Any".to_string().to_lowercase()),
            self.fname
        )
    }

    fn function_signature(&self) -> String {
        format!(
            "{}({}) -> {}",
            self.fname,
            self.argument_types
                .iter()
                .map(|a| format!("[{}]", a.to_lowercase()))
                .collect::<Vec<String>>()
                .join(", "),
            self.returns.to_lowercase()
        )
    }

    pub fn signature(&self) -> String {
        if self.ftype == "decorator" {
            self.decorator_signature()
        } else {
            self.function_signature()
        }
    }

    pub fn call_legacy(
        name: &str,
        module: &Module,
        args: &[Value],
    ) -> Result<Value, rustyscript::Error> {
        ExtensionsRuntime::with(|runtime| match runtime.load_module(module) {
            Ok(module_context) => {
                let mut _args = serde_json::to_value(args)?;
                runtime.call_function::<Value>(&module_context, name, &[_args])
            }
            Err(e) => Err(e),
        })
    }

    pub fn call_legacy_decorator(
        name: &str,
        module: &Module,
        arg: Value,
    ) -> Result<String, rustyscript::Error> {
        ExtensionsRuntime::with(|runtime| match runtime.load_module(module) {
            Ok(module_context) => {
                let mut _arg = serde_json::to_value(arg.clone())?;
                runtime.call_function::<String>(&module_context, name, &[_arg])
            }
            Err(e) => Err(e),
        })
    }

    pub fn call(
        &self,
        module: &Module,
        args: &[Value],
        variables: &mut HashMap<String, Value>,
    ) -> Result<Value, rustyscript::Error> {
        ExtensionsRuntime::with(|runtime| {
            match runtime.load_module(module) {
                Ok(module_context) => {
                    // Inject parser state
                    let json_variables = serde_json::to_value(variables.clone())?;
                    runtime.call_function(
                        &module_context,
                        "setLavendeuxState",
                        json_args!(json_variables),
                    )?;

                    // Decode arguments
                    let mut _args: Vec<serde_json::Value> = vec![serde_json::to_value(self)?];
                    for arg in args {
                        _args.push(serde_json::to_value(arg)?);
                    }

                    // Call the function
                    let result: Value = runtime.call_function(
                        &module_context,
                        "callLavendeuxFunction",
                        _args.as_slice(),
                    )?;

                    // Pull out modified state
                    let state: HashMap<String, Value> = runtime.call_function(
                        &module_context,
                        "getLavendeuxState",
                        json_args!(),
                    )?;
                    variables.clear();
                    for k in state.keys() {
                        variables.insert(k.to_string(), state.get(k).unwrap().clone());
                    }

                    Ok(result)
                }
                Err(e) => Err(e),
            }
        })
    }
}
