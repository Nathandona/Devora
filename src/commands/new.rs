use crate::cli::NewArgs;
use crate::core::{PluginRegistry, TemplateGenerator, ContextBuilder};
use crate::error::{DevoraError, Result};
use crate::utils::ensure_dir_exists;
use std::collections::HashMap;
use std::path::PathBuf;

pub async fn execute(args: NewArgs) -> Result<()> {
    // Determine output directory
    let output_dir = args.output_dir.unwrap_or_else(|| args.name.clone());
    let output_path = PathBuf::from(&output_dir);

    // Check if output directory already exists
    if output_path.exists() {
        return Err(DevoraError::FileSystemError {
            path: output_dir,
            message: "Directory already exists".to_string(),
        });
    }

    // Initialize plugin registry
    let plugins_dir = PathBuf::from("plugins");
    ensure_dir_exists(&plugins_dir)?;
    let mut registry = PluginRegistry::new(&plugins_dir);
    registry.discover()?;

    // Determine language and framework
    let language = args.language.ok_or_else(|| {
        DevoraError::ValidationError {
            field: "language".to_string(),
            message: "Language is required. Use 'devora list' to see available languages.".to_string(),
        }
    })?;

    let language_manifest = registry.get_language(&language)?;
    let framework = args.framework.unwrap_or_else(|| {
        language_manifest.default_framework.clone().unwrap_or_else(|| "base".to_string())
    });

    let framework_manifest = registry.get_framework(&language, &framework)?;

    println!("🚀 Creating new {} project: {}", language, args.name);
    println!("📁 Framework: {}", framework);
    if args.dry_run {
        println!("🔍 Dry run mode - no files will be created");
    }

    
    // Build context
    let context_builder = ContextBuilder::new();
    let mut provided_vars = HashMap::new();
    for (key, value) in args.var {
        provided_vars.insert(key, value);
    }

    // Interactive prompts for missing required variables
    let variables = framework_manifest.variables.clone().unwrap_or_default();
    if !args.non_interactive {
        for (name, var_def) in &variables {
            if !provided_vars.contains_key(name) && var_def.required {
                let prompt = var_def.prompt.as_ref().unwrap_or(&var_def.description);
                let input = dialoguer::Input::new()
                    .with_prompt(format!("{} ({})", prompt, name))
                    .default(var_def.default.as_ref()
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string())
                    .interact_text()?;
                provided_vars.insert(name.clone(), input);
            }
        }
    }

    let mut context = context_builder.build_context(
        &args.name,
        &variables,
        &provided_vars,
    )?;

    // Add framework name to context
    context.insert("framework".to_string(), serde_json::Value::String(framework.clone()));

    // Generate templates
    let templates_dir = plugins_dir.join(language).join("frameworks").join(framework).join("templates");
    let mut generator = TemplateGenerator::new(&templates_dir)?;

    if !args.dry_run {
        ensure_dir_exists(&output_path)?;

        // Execute pre-generation hooks
        if let Some(hooks) = &framework_manifest.pre_hooks {
            println!("🔧 Running pre-generation hooks...");
            generator.execute_hooks(hooks, &output_path, &context)?;
        }
    }

    generator.generate_project(
        &templates_dir,
        &output_path,
        &context,
        &framework_manifest.conditional_files.as_deref().unwrap_or(&[]),
        args.dry_run,
    )?;

    // Execute post-generation hooks
    if !args.dry_run {
        if let Some(hooks) = &framework_manifest.post_hooks {
            generator.execute_hooks(hooks, &output_path, &context)?;
        }
    }

    if !args.dry_run {
        println!("✅ Project '{}' created successfully at: {}", args.name, output_dir);
        println!();
        println!("Next steps:");
        println!("  cd {}", output_dir);

        if let Some(commands) = &framework_manifest.init_commands {
            for command in commands {
                println!("  {}", command);
            }
        } else {
            println!("  # Add your build/run commands here");
        }
    }

    Ok(())
}