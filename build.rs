fn main() -> std::io::Result<()> {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("alex-com.ico");
        res.compile()?

    }

    Ok(())
}