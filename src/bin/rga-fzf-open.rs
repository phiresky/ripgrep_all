use anyhow::Context;

use std::process::Command;

// TODO: add --rg-params=..., --rg-preview-params=... and --fzf-params=... params
// TODO: remove passthrough_args
fn main() -> anyhow::Result<()> {
    env_logger::init();
    let mut args = std::env::args().skip(1);
    let query = args.next().context("no query")?;
    let fname = args.next().context("no filename")?;
    // let instance_id = std::env::var("RGA_FZF_INSTANCE").unwrap_or("unk".to_string());
    use std::env;

    let (cmd, pdf_cmd) = match env::consts::OS {
        "macos" => ("open", "open -a Preview.app"), // use native Preview for macOs
        "linux" => ("xdg-open", "evince"),          // use evince for linux
        &_ => ("", ""),
    };
    if fname.ends_with(".pdf") {
        use std::io::ErrorKind::*;
        let worked = Command::new(pdf_cmd)
            .arg("--find")
            .arg(&query)
            .arg(&fname)
            .spawn()
            .map_or_else(
                |err| match err.kind() {
                    NotFound => Ok(false),
                    _ => Err(err),
                },
                |_| Ok(true),
            )?;
        if worked {
            return Ok(());
        }
    }
    Command::new(cmd).arg(fname).spawn()?;
    Ok(())
}
