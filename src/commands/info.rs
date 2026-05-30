use crate::cli::InfoArgs;
use crate::core::PluginRegistry;
use crate::error::Result;

pub async fn execute(args: InfoArgs, json: bool) -> Result<()> {
    let mut registry = PluginRegistry::new();
    registry.discover()?;

    if let Some(framework_name) = args.framework {
        if json {
            let manifest = registry.get_framework(&args.language, &framework_name)?;
            println!("{}", serde_json::to_string_pretty(manifest)?);
        } else {
            show_framework_info(&registry, &args.language, &framework_name)?;
        }
    } else if json {
        let manifest = registry.get_language(&args.language)?;
        println!("{}", serde_json::to_string_pretty(manifest)?);
    } else {
        show_language_info(&registry, &args.language)?;
    }

    Ok(())
}

fn show_language_info(registry: &PluginRegistry, language_name: &str) -> Result<()> {
    let language_manifest = registry.get_language(language_name)?;

    println!("Language Information");
    println!("═══════════════════════════════════════════════════════════════");
    println!("Name:        {}", language_manifest.language.name);
    println!("ID:          {}", language_manifest.language.id);
    println!("Version:     {}", language_manifest.language.version);
    println!("Description: {}", language_manifest.language.description);

    if let Some(default_framework) = &language_manifest.default_framework {
        println!("Default framework: {}", default_framework);
    }

    println!();

    // Show available frameworks
    println!("Available Frameworks");
    println!("═══════════════════════════════════════════════════════════════");

    if let Some(frameworks) = &language_manifest.frameworks {
        if frameworks.is_empty() {
            println!("No frameworks available for this language.");
        } else {
            for (i, framework_id) in frameworks.iter().enumerate() {
                if let Ok(framework_manifest) = registry.get_framework(language_name, framework_id)
                {
                    println!(
                        "{}. {} ({})",
                        i + 1,
                        framework_manifest.framework.name,
                        framework_manifest.framework.id
                    );
                    println!("   {}", framework_manifest.framework.description);
                    println!("   v{}", framework_manifest.framework.version);
                    println!();
                }
            }
        }
    } else {
        println!("No frameworks available for this language.");
    }

    // Show usage example
    println!("Usage Examples");
    println!("═══════════════════════════════════════════════════════════════");
    if let Some(default_framework) = &language_manifest.default_framework {
        println!(
            "devora new <project-name> {}                    # Use default framework",
            language_name
        );
        println!(
            "devora new <project-name> {} --framework {}     # Use specific framework",
            language_name, default_framework
        );
    }

    if let Some(frameworks) = &language_manifest.frameworks {
        for framework_id in frameworks {
            println!(
                "devora new <project-name> {} --framework {}    # Use {} framework",
                language_name, framework_id, framework_id
            );
        }
    }

    println!(
        "devora info {} <framework>                        # Get framework details",
        language_name
    );

    Ok(())
}

fn show_framework_info(
    registry: &PluginRegistry,
    language_name: &str,
    framework_name: &str,
) -> Result<()> {
    let framework_manifest = registry.get_framework(language_name, framework_name)?;

    println!("Framework Information");
    println!("═══════════════════════════════════════════════════════════════");
    println!("Name:        {}", framework_manifest.framework.name);
    println!("ID:          {}", framework_manifest.framework.id);
    println!("Version:     {}", framework_manifest.framework.version);
    println!("Description: {}", framework_manifest.framework.description);
    println!();

    // Show variables
    if let Some(variables) = &framework_manifest.variables {
        if !variables.is_empty() {
            println!("Configuration Variables");
            println!("═══════════════════════════════════════════════════════════════");

            for (name, var) in variables {
                println!("{}", name);
                println!("   Description: {}", var.description);

                if let Some(prompt) = &var.prompt {
                    println!("   Prompt: \"{}\"", prompt);
                }

                if let Some(default) = &var.default {
                    println!("   Default: {}", default);
                }

                if var.required {
                    println!("   Required: yes");
                } else {
                    println!("   Required: no");
                }

                if let Some(condition) = &var.condition {
                    println!("   Condition: {}", condition);
                }

                if let Some(validation) = &var.validation {
                    if let Some(regex) = &validation.regex {
                        println!("   Validation: Regex pattern \"{}\"", regex);
                    }
                    if let Some(min_length) = validation.min_length {
                        println!("   Validation: Min length {}", min_length);
                    }
                    if let Some(max_length) = validation.max_length {
                        println!("   Validation: Max length {}", max_length);
                    }
                    if let Some(allowed) = &validation.allowed_values {
                        println!("   Validation: Allowed values: {}", allowed.join(", "));
                    }
                }

                println!();
            }
        }
    }

    // Show hooks
    if let Some(pre_hooks) = &framework_manifest.pre_hooks {
        if !pre_hooks.is_empty() {
            println!("Pre-Generation Hooks");
            println!("═══════════════════════════════════════════════════════════════");

            for (i, hook) in pre_hooks.iter().enumerate() {
                print_hook_info(i + 1, hook)?;
            }
            println!();
        }
    }

    if let Some(post_hooks) = &framework_manifest.post_hooks {
        if !post_hooks.is_empty() {
            println!("Post-Generation Hooks");
            println!("═══════════════════════════════════════════════════════════════");

            for (i, hook) in post_hooks.iter().enumerate() {
                print_hook_info(i + 1, hook)?;
            }
            println!();
        }
    }

    // Show conditional files
    if let Some(conditional_files) = &framework_manifest.conditional_files {
        if !conditional_files.is_empty() {
            println!("Conditional Files");
            println!("═══════════════════════════════════════════════════════════════");

            for (i, file) in conditional_files.iter().enumerate() {
                println!("{}. {}", i + 1, file.path);
                println!("   Condition: {}", file.condition);
                if let Some(description) = &file.description {
                    println!("   Description: {}", description);
                }
                println!();
            }
        }
    }

    // Show requirements
    if let Some(requirements) = &framework_manifest.requirements {
        if !requirements.is_empty() {
            println!("Requirements");
            println!("═══════════════════════════════════════════════════════════════");

            for (i, requirement) in requirements.iter().enumerate() {
                println!("{}. {}", i + 1, requirement);
            }
            println!();
        }
    }

    // Show usage example
    println!("Usage Example");
    println!("═══════════════════════════════════════════════════════════════");
    println!(
        "devora new <project-name> {} --framework {}",
        language_name, framework_name
    );

    // Add example variables if available
    if let Some(variables) = &framework_manifest.variables {
        let required_vars: Vec<_> = variables
            .iter()
            .filter(|(_, var)| var.required)
            .map(|(name, _)| format!("--var {}=<value>", name))
            .collect();

        if !required_vars.is_empty() {
            println!("Required variables: {}", required_vars.join(" "));
        }

        let optional_vars: Vec<_> = variables
            .iter()
            .filter(|(_, var)| !var.required && var.default.is_none())
            .map(|(name, var)| {
                if let Some(prompt) = &var.prompt {
                    format!("--var {}=<value> # {}", name, prompt)
                } else {
                    format!("--var {}=<value>", name)
                }
            })
            .collect();

        if !optional_vars.is_empty() {
            println!("Optional variables:");
            for var in optional_vars {
                println!("  {}", var);
            }
        }
    }

    Ok(())
}

fn print_hook_info(index: usize, hook: &crate::models::Hook) -> Result<()> {
    println!("{}. {}", index, hook.command);

    if let Some(description) = &hook.description {
        println!("   Description: {}", description);
    }

    if let Some(working_dir) = &hook.working_directory {
        println!("   Working Directory: {}", working_dir);
    }

    if let Some(condition) = &hook.condition {
        println!("   Condition: {}", condition);
    }

    if let Some(env_vars) = &hook.environment {
        if !env_vars.is_empty() {
            println!("   Environment Variables:");
            for (key, value) in env_vars {
                println!("     {}={}", key, value);
            }
        }
    }

    println!();
    Ok(())
}
