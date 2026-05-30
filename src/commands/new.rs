use crate::cli::NewArgs;
use crate::core::{ContextBuilder, PluginRegistry, TemplateGenerator};
use crate::error::{DevoraError, Result};
use crate::utils::ensure_dir_exists;
use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;
use std::time::Instant;

pub async fn execute(args: NewArgs, json: bool) -> Result<()> {
    let start = Instant::now();

    // Machine mode can never prompt.
    let non_interactive = args.non_interactive || json;

    // Determine output directory.
    let output_dir = args.output_dir.clone().unwrap_or_else(|| args.name.clone());
    let output_path = PathBuf::from(&output_dir);

    if output_path.exists() {
        return Err(DevoraError::FileSystemError {
            path: output_dir,
            message: "Directory already exists".to_string(),
        });
    }

    // Discover the embedded plugins.
    let mut registry = PluginRegistry::new();
    registry.discover()?;

    let language = args
        .language
        .clone()
        .ok_or_else(|| DevoraError::ValidationError {
            field: "language".to_string(),
            message: "Language is required. Use 'devora list' to see available languages."
                .to_string(),
        })?;

    let language_manifest = registry.get_language(&language)?;
    let lang_version = language_manifest.language.version.clone();
    let framework = args.framework.clone().unwrap_or_else(|| {
        language_manifest
            .default_framework
            .clone()
            .unwrap_or_else(|| "base".to_string())
    });

    let framework_manifest = registry.get_framework(&language, &framework)?.clone();

    if !json {
        if args.dry_run {
            println!("Dry run mode — no files will be created");
        }
        println!("Resolving plugin: {}@{}", language, lang_version);
        println!("Rendering templates …");
    }

    // Build the context.
    let context_builder = ContextBuilder::new();
    let mut provided_vars: HashMap<String, String> = HashMap::new();
    for (key, value) in args.var.clone() {
        provided_vars.insert(key, value);
    }

    let variables = framework_manifest.variables.clone().unwrap_or_default();
    if !non_interactive {
        for (name, var_def) in &variables {
            if !provided_vars.contains_key(name) && var_def.required {
                let prompt = var_def.prompt.as_ref().unwrap_or(&var_def.description);
                let input = dialoguer::Input::new()
                    .with_prompt(format!("{} ({})", prompt, name))
                    .default(
                        var_def
                            .default
                            .as_ref()
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                    )
                    .interact_text()?;
                provided_vars.insert(name.clone(), input);
            }
        }
    }

    let mut context = context_builder.build_context(&args.name, &variables, &provided_vars)?;
    context.insert(
        "framework".to_string(),
        serde_json::Value::String(framework.clone()),
    );

    // Resolve the embedded templates directory for this framework.
    let templates_dir = format!("{}/frameworks/{}/templates", language, framework);
    let mut generator = TemplateGenerator::new(&templates_dir)?;

    let run_hooks = !args.no_hooks && !args.dry_run;

    if !args.dry_run {
        ensure_dir_exists(&output_path)?;
        if run_hooks {
            if let Some(hooks) = &framework_manifest.pre_hooks {
                generator.execute_hooks(hooks, &output_path, &context)?;
            }
        }
    }

    let created = generator.generate_project(
        &output_path,
        &context,
        framework_manifest
            .conditional_files
            .as_deref()
            .unwrap_or(&[]),
        args.dry_run,
    )?;

    if run_hooks {
        if let Some(hooks) = &framework_manifest.post_hooks {
            generator.execute_hooks(hooks, &output_path, &context)?;
        }
    }

    let elapsed = start.elapsed();

    if json {
        let out = serde_json::json!({
            "name": args.name,
            "language": language,
            "framework": framework,
            "path": output_dir,
            "files": created,
            "dry_run": args.dry_run,
            "hooks_skipped": args.no_hooks,
            "elapsed_ms": elapsed.as_millis() as u64,
        });
        println!("{}", serde_json::to_string_pretty(&out)?);
    } else if !args.dry_run {
        print_tree(&args.name, &created);
        println!("created {} in {:.2}s", args.name, elapsed.as_secs_f64());
    }

    Ok(())
}

/// Print a generated project's files as a tree, matching the website animation:
///
/// ```text
/// my-app
/// ├─ Cargo.toml
/// └─ src
///    └─ main.rs
/// ```
fn print_tree(root: &str, files: &[String]) {
    #[derive(Default)]
    struct Node {
        children: BTreeMap<String, Node>,
    }

    fn insert(node: &mut Node, parts: &[&str]) {
        if let Some((first, rest)) = parts.split_first() {
            let child = node.children.entry((*first).to_string()).or_default();
            insert(child, rest);
        }
    }

    fn print_node(node: &Node, prefix: &str) {
        let count = node.children.len();
        for (i, (name, child)) in node.children.iter().enumerate() {
            let last = i == count - 1;
            let connector = if last { "└─ " } else { "├─ " };
            println!("{}{}{}", prefix, connector, name);
            let child_prefix = format!("{}{}", prefix, if last { "   " } else { "│  " });
            print_node(child, &child_prefix);
        }
    }

    let mut tree = Node::default();
    for file in files {
        let parts: Vec<&str> = file.split('/').filter(|p| !p.is_empty()).collect();
        insert(&mut tree, &parts);
    }

    println!("{}", root);
    print_node(&tree, "");
}
