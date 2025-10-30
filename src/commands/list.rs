use crate::cli::ListArgs;
use crate::error::DevoraError;

pub async fn execute(args: ListArgs) -> Result<(), DevoraError> {
    println!("📋 Available Languages and Frameworks");
    println!("=====================================");

    // Initialize plugin registry
    let plugins_dir = std::path::PathBuf::from("plugins");
    let mut registry = crate::core::PluginRegistry::new(&plugins_dir);
    registry.discover()?;

    if let Some(language) = args.language {
        println!("Frameworks for language: {}", language);
        let frameworks = registry.list_frameworks(&language);
        if frameworks.is_empty() {
            println!("  No frameworks found for this language");
        } else {
            for framework in frameworks {
                println!("  {}", framework);
            }
        }
    } else {
        println!("Available languages:");
        let languages = registry.list_languages();
        if languages.is_empty() {
            println!("  No languages found. Make sure plugins are set up correctly.");
        } else {
            for language in languages {
                match registry.get_language(language) {
                    Ok(manifest) => println!("  {} - {}", language, manifest.language.description),
                    Err(_) => println!("  {}", language),
                }
            }
        }
    }

    Ok(())
}