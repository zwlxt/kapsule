use druid::{
    commands::NEW_FILE,
    im::{vector, Vector},
    widget::{Controller, Flex, Label, List, Scroll},
    AppLauncher, Data, Env, Event, Lens, MenuItem, Widget, WidgetExt, WindowDesc, WindowId, Menu, ArcStr,
};

#[derive(Debug, Clone, Data)]
pub struct Entry {
    pub name: ArcStr,
}

#[derive(Data, Lens, Clone)]
struct KapsuleAppState {
    entries: Vector<Entry>,
}

impl KapsuleAppState {
    fn new() -> Self {
        Self {
            entries: vector![Entry { name: "a".into() },],
        }
    }
}

fn browser_window() -> impl Widget<KapsuleAppState> {
    Flex::column().with_child(file_browser().expand())
}

fn file_browser() -> impl Widget<KapsuleAppState> {
    let file_list = List::new(browser_item).controller(KapsuleController);
    Flex::column()
        .with_child(Scroll::new(file_list).vertical())
        .padding(8.0)
        .lens(KapsuleAppState::entries)
}

fn browser_item() -> impl Widget<Entry> {
    Flex::row()
        .with_child(Label::dynamic(|entry: &Entry, _| entry.name.to_string()))
        .align_left()
}

#[allow(unused_assignments)]
fn browser_menu(_: Option<WindowId>, _: &KapsuleAppState, _: &Env) -> Menu<KapsuleAppState> {
    let mut base = Menu::empty();
    #[cfg(target_os = "macos")]
    {
        base = druid::platform_menus::mac::menu_bar();
    }
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    {
        base = base.entry(druid::platform_menus::win::file::default());
    }
    base.entry(Menu::new("Window").entry(MenuItem::new("New Window")))
}

struct KapsuleController;

impl<W: Widget<Vector<Entry>>> Controller<Vector<Entry>, W> for KapsuleController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut Vector<Entry>,
        env: &Env,
    ) {
        match event {
            Event::Command(cmd) if cmd.is(NEW_FILE) => println!("new file"),
            _ => child.event(ctx, event, data, env),
        }
    }
}

fn main() {
    let state = KapsuleAppState::new();
    let window = WindowDesc::new(browser_window())
        .title("Kapsule")
        .menu(browser_menu);
    AppLauncher::with_window(window)
        .launch(state)
        .expect("failed to launch Kapsule gui")
}
