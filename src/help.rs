use crate::{state::UserFunction, ParserState};
use std::{collections::HashMap, fmt};

fn get_divider(title: &str) -> String {
    (0..title.len()).map(|_| "=").collect::<String>()
}

fn noun_case(text: &str) -> String {
    let mut c = text.chars();
    c.next().unwrap_or(' ').to_uppercase().chain(c).collect()
}

pub struct HelpBlock {
    title: String,
    entries: Vec<String>,
    order: usize,
}

impl HelpBlock {
    pub fn new(title: &str, i: usize) -> Self {
        Self {
            title: title.to_string(),
            entries: Vec::new(),
            order: i,
        }
    }

    pub fn add_entry(&mut self, entry: &str) -> &mut Self {
        self.entries.push(entry.to_string());
        self
    }

    pub fn order(&self) -> usize {
        self.order
    }

    pub fn title(&self) -> &str {
        &self.title
    }
}

impl fmt::Display for HelpBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.entries.join("\n"))
    }
}

pub struct Help {
    blocks: HashMap<String, HelpBlock>,
}

impl Help {
    pub fn new() -> Self {
        Self {
            blocks: HashMap::new(),
        }
    }

    pub fn add_std(&mut self, state: &mut ParserState) {
        self.add_std_functions(state);
        self.add_std_decorators(state);

        #[cfg(feature = "extensions")]
        self.add_extensions(state);

        self.add_user_functions(state);
        self.add_variables(state);
    }

    /// Add the built-in functions to the help instance
    pub fn add_std_functions(&mut self, state: &ParserState) {
        for (category, functions) in state.functions.all_by_category() {
            let block = self.add_block(&format!("{} Functions", &noun_case(category)));
            for f in functions {
                block.add_entry(&f.help());
            }
        }
    }

    /// Add the built-in decorations to the help instance
    pub fn add_std_decorators(&mut self, state: &ParserState) {
        let block = self.add_block("Built-in Decorators");
        for decorator in state.decorators.all() {
            block.add_entry(&decorator.help());
        }
    }

    /// Add loaded extensions to the help instance
    #[cfg(feature = "extensions")]
    pub fn add_extensions(&mut self, state: &mut ParserState) {
        for extension in state.extensions.all() {
            let title = format!("{} v{}", extension.name(), extension.version());
            self.add_block(&title)
                .add_entry(&format!("Author: {}", extension.author()))
                .add_entry(&format!(
                    "Functions:\n {}",
                    extension.function_signatures().join("\n ")
                ))
                .add_entry(&format!(
                    "Decorators:\n {}",
                    extension.decorator_signatures().join("\n ")
                ));
        }
    }

    pub fn add_user_functions(&mut self, state: &ParserState) {
        let block = self.add_block("User-defined Functions");
        let mut functions: Vec<&UserFunction> = state.user_functions.values().collect();
        functions.sort_by(|f1, f2| f1.name().cmp(f2.name()));

        if functions.is_empty() {
            block.add_entry(" -- None --");
        }

        for f in functions {
            block.add_entry(&f.signature());
        }
    }

    pub fn add_variables(&mut self, state: &ParserState) {
        let block = self.add_block("Defined Variables");
        for (name, value) in &state.constants {
            block.add_entry(&format!("{} = {} [constant]", name, value));
        }
        for (name, value) in &state.variables {
            block.add_entry(&format!("{} = {}", name, value));
        }
    }

    pub fn add_block(&mut self, title: &str) -> &mut HelpBlock {
        self.blocks
            .insert(title.to_string(), HelpBlock::new(title, self.blocks.len()));
        self.get_block(title).unwrap()
    }

    pub fn get_block(&mut self, title: &str) -> Option<&mut HelpBlock> {
        self.blocks.get_mut(title)
    }
}

impl fmt::Display for Help {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut blocks: Vec<&HelpBlock> = self.blocks.values().collect();
        blocks.sort_by_key(|f| f.order());

        let text = blocks
            .iter()
            .map(|b| format!("{}\n{}\n{}\n", b.title(), get_divider(b.title()), b))
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", text)
    }
}
