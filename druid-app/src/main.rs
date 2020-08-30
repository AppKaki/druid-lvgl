use druid::{AppLauncher, WindowDesc, Widget, PlatformError};
use druid::widget::Label;

fn build_ui() -> impl Widget<()> {
    Label::new("Hello world")
}

fn main() -> Result<(), PlatformError> {
    println!("In Rust: main()");
    AppLauncher::with_window(WindowDesc::new(build_ui)).launch(())?;
    Ok(())
}