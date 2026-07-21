use quote::{ToTokens, quote};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use syn::parse::Parser;
use syn::visit::{self, Visit};
use syn::visit_mut::{self, VisitMut};
use syn::{
    Attribute, Expr, File, Item, ItemMacro, ItemMod, ItemUse, Path as SynPath, Stmt, Token,
    UseTree, punctuated::Punctuated,
};

type ModulePath = Vec<String>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args_os().skip(1).collect();
    let (input, output, copy) = match args.as_slice() {
        [] => (
            PathBuf::from("src/main.rs"),
            Some(PathBuf::from("submission.rs")),
            false,
        ),
        [flag] if flag == "--clipboard" || flag == "-c" => {
            (PathBuf::from("src/main.rs"), None, true)
        }
        [input] => (
            PathBuf::from(input),
            Some(PathBuf::from("submission.rs")),
            false,
        ),
        [input, output] => (PathBuf::from(input), Some(PathBuf::from(output)), false),
        _ => return Err("usage: cargo bundle [INPUT [OUTPUT]] | cargo bundle --clipboard".into()),
    };

    let src_dir = input.parent().unwrap_or_else(|| Path::new("."));
    let crate_name = env!("CARGO_PKG_NAME").replace('-', "_");
    let modules = discover_modules(src_dir)?;
    let macro_modules = discover_exported_macros(&modules)?;

    let mut main_file = parse_file(&input)?;
    remove_test_items(&mut main_file.items);
    remove_dbg(&mut main_file.items);
    let mut references = References::new(&crate_name);
    references.visit_file(&main_file);

    let mut selected = BTreeSet::new();
    let mut queue = VecDeque::from(references.paths);
    while let Some(reference) = queue.pop_front() {
        let Some(module) = resolve_module(&reference, &modules, &macro_modules) else {
            continue;
        };
        if !selected.insert(module.clone()) {
            continue;
        }

        let mut file = parse_file(&modules[&module])?;
        remove_test_items(&mut file.items);
        let mut dependencies = References::new("crate");
        dependencies.visit_file(&file);
        queue.extend(dependencies.paths);
    }

    rewrite_crate_name(&mut main_file.items, &crate_name);
    let module_tokens = render_modules(&selected, &modules)?;
    let attrs = &main_file.attrs;
    let items = &main_file.items;
    let raw = quote!(#(#attrs)* #module_tokens #(#items)*).to_string();
    let formatted = rustfmt(&raw).unwrap_or(raw);
    let bundled = remove_doc_attributes(&formatted);

    if let Some(output) = &output {
        fs::write(&output, &bundled)?;
        eprintln!(
            "bundled {} module(s) into {}",
            selected.len(),
            output.display()
        );
    }
    if copy {
        if copy_to_clipboard(&bundled) {
            eprintln!("copied submission to clipboard");
        } else {
            eprintln!("warning: could not access a supported system clipboard");
        }
    } else if output.is_none() {
        print!("{bundled}");
    }
    Ok(())
}

fn copy_to_clipboard(source: &str) -> bool {
    let commands: &[(&str, &[&str])] = &[
        ("wl-copy", &[]),
        ("xclip", &["-selection", "clipboard"]),
        ("xsel", &["--clipboard", "--input"]),
        ("pbcopy", &[]),
    ];

    commands.iter().any(|(program, args)| {
        let Ok(mut child) = Command::new(program)
            .args(*args)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
        else {
            return false;
        };
        let written = child
            .stdin
            .take()
            .is_some_and(|mut stdin| stdin.write_all(source.as_bytes()).is_ok());
        written && child.wait().is_ok_and(|status| status.success())
    })
}

fn remove_doc_attributes(source: &str) -> String {
    source
        .lines()
        .filter(|line| {
            let trimmed = line.trim_start();
            !trimmed.starts_with("#[doc") && !trimmed.starts_with("#![doc")
        })
        .map(|line| format!("{line}\n"))
        .collect()
}

fn discover_modules(src_dir: &Path) -> io::Result<BTreeMap<ModulePath, PathBuf>> {
    fn walk(
        dir: &Path,
        src_dir: &Path,
        result: &mut BTreeMap<ModulePath, PathBuf>,
    ) -> io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() {
                if path.file_name().is_some_and(|name| name == "bin") {
                    continue;
                }
                walk(&path, src_dir, result)?;
            } else if path.extension().is_some_and(|ext| ext == "rs")
                && !matches!(
                    path.file_name().and_then(|x| x.to_str()),
                    Some("main.rs" | "lib.rs" | "mod.rs")
                )
            {
                let relative = path.strip_prefix(src_dir).expect("module is below src");
                let mut module: ModulePath = relative
                    .parent()
                    .into_iter()
                    .flat_map(Path::components)
                    .map(|part| part.as_os_str().to_string_lossy().into_owned())
                    .collect();
                module.push(relative.file_stem().unwrap().to_string_lossy().into_owned());
                result.insert(module, path);
            }
        }
        Ok(())
    }

    let mut result = BTreeMap::new();
    walk(src_dir, src_dir, &mut result)?;
    Ok(result)
}

fn parse_file(path: &Path) -> Result<File, Box<dyn std::error::Error>> {
    let source = fs::read_to_string(path)?;
    syn::parse_file(&source).map_err(|error| format!("{}: {error}", path.display()).into())
}

fn is_test_attr(attr: &Attribute) -> bool {
    if attr.path().is_ident("test") {
        return true;
    }
    if !attr.path().is_ident("cfg") {
        return false;
    }
    attr.meta.to_token_stream().to_string().contains("test")
}

fn remove_test_items(items: &mut Vec<Item>) {
    items.retain(|item| {
        let attrs: &[Attribute] = match item {
            Item::Const(item) => &item.attrs,
            Item::Enum(item) => &item.attrs,
            Item::ExternCrate(item) => &item.attrs,
            Item::Fn(item) => &item.attrs,
            Item::ForeignMod(item) => &item.attrs,
            Item::Impl(item) => &item.attrs,
            Item::Macro(item) => &item.attrs,
            Item::Mod(item) => &item.attrs,
            Item::Static(item) => &item.attrs,
            Item::Struct(item) => &item.attrs,
            Item::Trait(item) => &item.attrs,
            Item::TraitAlias(item) => &item.attrs,
            Item::Type(item) => &item.attrs,
            Item::Union(item) => &item.attrs,
            Item::Use(item) => &item.attrs,
            _ => &[],
        };
        !attrs.iter().any(is_test_attr)
    });
    for item in items {
        if let Item::Mod(module) = item
            && let Some((_, nested)) = &mut module.content
        {
            remove_test_items(nested);
        }
    }
}

fn remove_dbg(items: &mut Vec<Item>) {
    items.retain_mut(|item| {
        if let Item::Use(item_use) = item {
            return !remove_dbg_from_use(&mut item_use.tree);
        }
        true
    });
    let mut remover = DbgRemover;
    for item in items {
        remover.visit_item_mut(item);
    }
}

// Returns true when the entire use tree became empty.
fn remove_dbg_from_use(tree: &mut UseTree) -> bool {
    match tree {
        UseTree::Path(path) => {
            if path.ident == "dbg" {
                true
            } else {
                remove_dbg_from_use(&mut path.tree)
            }
        }
        UseTree::Name(name) => name.ident == "dbg",
        UseTree::Rename(rename) => rename.ident == "dbg",
        UseTree::Group(group) => {
            let mut retained = Punctuated::new();
            for mut item in group.items.clone() {
                if !remove_dbg_from_use(&mut item) {
                    retained.push(item);
                }
            }
            group.items = retained;
            group.items.is_empty()
        }
        UseTree::Glob(_) => false,
    }
}

struct DbgRemover;

impl VisitMut for DbgRemover {
    fn visit_block_mut(&mut self, block: &mut syn::Block) {
        block.stmts.retain(|statement| {
            !matches!(
                statement,
                Stmt::Expr(Expr::Macro(call), Some(_)) if call.mac.path.is_ident("dbg")
            ) && !matches!(
                statement,
                Stmt::Macro(call) if call.mac.path.is_ident("dbg")
            )
        });
        visit_mut::visit_block_mut(self, block);
    }

    fn visit_expr_mut(&mut self, expression: &mut Expr) {
        let Expr::Macro(call) = expression else {
            visit_mut::visit_expr_mut(self, expression);
            return;
        };
        if !call.mac.path.is_ident("dbg") {
            visit_mut::visit_expr_mut(self, expression);
            return;
        }

        let parser = Punctuated::<Expr, Token![,]>::parse_terminated;
        let Ok(mut arguments) = parser.parse2(call.mac.tokens.clone()) else {
            return;
        };
        *expression = match arguments.len() {
            0 => syn::parse_quote!(()),
            1 => arguments.pop().unwrap().into_value(),
            _ => syn::parse_quote!((#arguments)),
        };
        visit_mut::visit_expr_mut(self, expression);
    }
}

fn discover_exported_macros(
    modules: &BTreeMap<ModulePath, PathBuf>,
) -> Result<BTreeMap<String, ModulePath>, Box<dyn std::error::Error>> {
    let mut result = BTreeMap::new();
    for (module, path) in modules {
        // An unrelated scratch module should not prevent bundling the modules
        // used by the current solution. A selected module is parsed strictly
        // later, where its error is relevant and reported to the user.
        let Ok(file) = parse_file(path) else {
            continue;
        };
        for item in file.items {
            if let Item::Macro(ItemMacro {
                attrs,
                ident: Some(ident),
                ..
            }) = item
                && attrs
                    .iter()
                    .any(|attr| attr.path().is_ident("macro_export"))
            {
                result.insert(ident.to_string(), module.clone());
            }
        }
    }
    Ok(result)
}

fn resolve_module(
    reference: &ModulePath,
    modules: &BTreeMap<ModulePath, PathBuf>,
    macros: &BTreeMap<String, ModulePath>,
) -> Option<ModulePath> {
    (1..=reference.len())
        .rev()
        .map(|length| reference[..length].to_vec())
        .find(|candidate| modules.contains_key(candidate))
        .or_else(|| reference.first().and_then(|name| macros.get(name)).cloned())
}

struct References<'a> {
    root: &'a str,
    paths: Vec<ModulePath>,
}

impl<'a> References<'a> {
    fn new(root: &'a str) -> Self {
        Self {
            root,
            paths: Vec::new(),
        }
    }

    fn record_path(&mut self, path: &SynPath) {
        let mut segments = path.segments.iter().map(|part| part.ident.to_string());
        if segments.next().as_deref() == Some(self.root) {
            let rest = segments.collect::<Vec<_>>();
            if !rest.is_empty() {
                self.paths.push(rest);
            }
        }
    }

    fn record_use(&mut self, tree: &UseTree) {
        fn flatten(tree: &UseTree, prefix: &mut ModulePath, output: &mut Vec<ModulePath>) {
            match tree {
                UseTree::Path(path) => {
                    prefix.push(path.ident.to_string());
                    flatten(&path.tree, prefix, output);
                    prefix.pop();
                }
                UseTree::Name(name) => {
                    prefix.push(name.ident.to_string());
                    output.push(prefix.clone());
                    prefix.pop();
                }
                UseTree::Rename(rename) => {
                    prefix.push(rename.ident.to_string());
                    output.push(prefix.clone());
                    prefix.pop();
                }
                UseTree::Glob(_) => output.push(prefix.clone()),
                UseTree::Group(group) => {
                    for item in &group.items {
                        flatten(item, prefix, output);
                    }
                }
            }
        }

        let mut paths = Vec::new();
        flatten(tree, &mut Vec::new(), &mut paths);
        self.paths.extend(paths.into_iter().filter_map(|path| {
            (path.first().map(String::as_str) == Some(self.root)).then(|| path[1..].to_vec())
        }));
    }
}

impl<'ast> Visit<'ast> for References<'_> {
    fn visit_item_use(&mut self, item: &'ast ItemUse) {
        self.record_use(&item.tree);
        visit::visit_item_use(self, item);
    }

    fn visit_path(&mut self, path: &'ast SynPath) {
        self.record_path(path);
        visit::visit_path(self, path);
    }
}

fn rewrite_crate_name(items: &mut [Item], crate_name: &str) {
    fn rewrite_use(tree: &mut UseTree, crate_name: &str) {
        if let UseTree::Path(path) = tree
            && path.ident == crate_name
        {
            path.ident = syn::Ident::new("crate", path.ident.span());
        }
    }

    for item in items {
        if let Item::Use(item_use) = item {
            rewrite_use(&mut item_use.tree, crate_name);
        }
    }
}

#[derive(Default)]
struct ModuleNode {
    source: Option<ModulePath>,
    children: BTreeMap<String, ModuleNode>,
}

fn render_modules(
    selected: &BTreeSet<ModulePath>,
    modules: &BTreeMap<ModulePath, PathBuf>,
) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    fn render(
        node: &ModuleNode,
        modules: &BTreeMap<ModulePath, PathBuf>,
    ) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
        let own_items = if let Some(module) = &node.source {
            let mut file = parse_file(&modules[module])?;
            remove_test_items(&mut file.items);
            file.items
                .retain(|item| !matches!(item, Item::Mod(ItemMod { content: None, .. })));
            let items = file.items;
            quote!(#(#items)*)
        } else {
            quote!()
        };

        let mut children = proc_macro2::TokenStream::new();
        for (name, child) in &node.children {
            let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
            let body = render(child, modules)?;
            children.extend(quote!(pub mod #ident { #body }));
        }
        Ok(quote!(#own_items #children))
    }

    let mut root = ModuleNode::default();
    for module in selected {
        let mut node = &mut root;
        for part in module {
            node = node.children.entry(part.clone()).or_default();
        }
        node.source = Some(module.clone());
    }
    render(&root, modules)
}

fn rustfmt(source: &str) -> Option<String> {
    let mut child = Command::new("rustfmt")
        .args(["--edition", "2024"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;
    let mut stdin = child.stdin.take()?;
    stdin.write_all(source.as_bytes()).ok()?;
    drop(stdin);
    let output = child.wait_with_output().ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8(output.stdout).ok())
        .flatten()
}
