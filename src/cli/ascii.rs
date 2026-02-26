use colored::Colorize;

pub fn splash() -> String {
    let hxn_version = env!("CARGO_PKG_VERSION");

    let logo = format!(
        r#"
    ┓┏    ┓      
    ┣┫┏┓┓┏┃┓┏┏┓┏┓
    ┛┗┗┻┗┫┗┛┗┗┛┛┗ v{}
         ┛       
    Shoot before the blink
                @pwnwriter/haylxon
 "#,
        hxn_version
    )
    .purple();

    format!("{logo}")
}
