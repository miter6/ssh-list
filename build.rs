#[cfg(windows)]
use winres;

fn main() {
    #[cfg(windows)]
    {
        if matches!(std::env::var("CARGO_CFG_TARGET_OS").as_deref(), Ok("windows")) {
            let mut res = winres::WindowsResource::new();
            res.set_icon("images/icon.ico");
            res.set("ProductName", env!("CARGO_PKG_NAME"));
            res.set("FileDescription", env!("CARGO_PKG_DESCRIPTION"));
            res.set("LegalCopyright", env!("CARGO_PKG_AUTHORS"));
            if let Err(e) = res.compile() {
                eprintln!("Failed to compile Windows resources: {}", e);
            }
        }
    }
}
