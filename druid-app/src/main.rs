use druid::{AppLauncher, WindowDesc, Widget, PlatformError};
use druid::widget::Label;

fn build_ui() -> impl Widget<()> + Clone {
    Label::new("Hello world")
}

fn main() -> Result<(), PlatformError> {
    println!("In Rust: main()");
    let window = WindowDesc::new(build_ui);
    let launcher = AppLauncher::with_window(window);
    launcher.launch(())?;
    ////AppLauncher::with_window(WindowDesc::new(build_ui)).launch(())?;
    Ok(())
}