use crate::cli::ListArgs;
use crate::core::roadmap::ROADMAP;
use crate::core::PluginRegistry;
use crate::error::Result;

pub async fn execute(args: ListArgs, json: bool) -> Result<()> {
    let mut registry = PluginRegistry::new();
    registry.discover()?;

    // `devora list <language>` → frameworks for that language.
    if let Some(language) = args.language {
        let mut frameworks = registry.list_frameworks(&language);
        frameworks.sort();

        if json {
            let out = serde_json::json!({
                "language": language,
                "frameworks": frameworks,
            });
            println!("{}", serde_json::to_string_pretty(&out)?);
        } else if frameworks.is_empty() {
            println!("No frameworks available for '{}'.", language);
        } else {
            for framework in frameworks {
                println!("{}", framework);
            }
        }

        return Ok(());
    }

    // `devora list` → the language status board.
    if json {
        let langs: Vec<_> = ROADMAP
            .iter()
            .map(|l| {
                serde_json::json!({
                    "name": l.name,
                    "state": l.state,
                    "note": l.note,
                })
            })
            .collect();
        let out = serde_json::json!({ "languages": langs });
        println!("{}", serde_json::to_string_pretty(&out)?);
    } else {
        for lang in ROADMAP {
            println!("{:<10} {:<13} {}", lang.state, lang.name, lang.note);
        }
    }

    Ok(())
}
