use crate::cli::InfoArgs;
use crate::error::DevoraError;

pub async fn execute(args: InfoArgs) -> Result<(), DevoraError> {
    println!("ℹ️  Information for: {}", args.language);

    if let Some(framework) = args.framework {
        println!("Framework: {}", framework);
    } else {
        println!("Language: {}", args.language);
        println!("This command will show detailed information about the language/framework.");
    }

    Ok(())
}