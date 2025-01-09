fn main() -> anyhow::Result<()> {
    AppShared::write()?;

    Ok(())
}

struct AppShared;

impl AppShared {
    const PORT: &'static str = std::env!("PORT");

    fn write() -> anyhow::Result<()> {
        use std::io::Write;

        let mut file = std::fs::File::create("src/shared/app.rs")?;

        writeln!(file, "pub const PORT: u16 = {};", Self::PORT)?;

        Ok(())
    }
}
