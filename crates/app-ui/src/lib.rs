//! Iced presentation layer. Keep domain logic out of this crate.

use iced::widget::{button, column, container, row, text, text_editor};
use iced::{Alignment, Element, Font, Length, Task, Theme};

/// Start the desktop UI.
pub fn run() -> iced::Result {
    iced::application(App::default, update, view)
        .title("Rust IDE")
        .theme(|_: &App| Theme::Dark)
        .window_size(iced::Size::new(1440.0, 900.0))
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    ToggleTerminal,
    SelectFile(&'static str),
    EditorAction(text_editor::Action),
}

struct App {
    selected_file: &'static str,
    show_terminal: bool,
    editor: text_editor::Content,
}

impl Default for App {
    fn default() -> Self {
        Self {
            selected_file: "src/main.rs",
            show_terminal: true,
            editor: text_editor::Content::with_text(
                "fn main() {\n    println!(\"Hello from Rust IDE\");\n}\n",
            ),
        }
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::ToggleTerminal => app.show_terminal = !app.show_terminal,
        Message::SelectFile(path) => {
            app.selected_file = path;
            let preview = match path {
                "Cargo.toml" => "[workspace]\nresolver = \"2\"\nmembers = []\n",
                "README.md" => "# Rust IDE\n\nIced UI + Rust backend.\n",
                _ => "// Select a source file.\n",
            };
            app.editor = text_editor::Content::with_text(preview);
        }
        Message::EditorAction(action) => app.editor.perform(action),
    }
    Task::none()
}

fn view(app: &App) -> Element<'_, Message> {
    let toolbar = row![
        text("RUST IDE").size(16),
        text("  Workspace: de-pinna-template").size(13),
        iced::widget::Space::new().width(Length::Fill),
        button(if app.show_terminal { "Hide Terminal" } else { "Show Terminal" })
            .on_press(Message::ToggleTerminal),
    ]
    .align_y(Alignment::Center)
    .spacing(12)
    .padding(10);

    let explorer = container(
        column![
            text("EXPLORER").size(12),
            button("src/main.rs")
                .width(Length::Fill)
                .on_press(Message::SelectFile("src/main.rs")),
            button("crates/")
                .width(Length::Fill)
                .on_press(Message::SelectFile("crates/")),
            button("Cargo.toml")
                .width(Length::Fill)
                .on_press(Message::SelectFile("Cargo.toml")),
            button("README.md")
                .width(Length::Fill)
                .on_press(Message::SelectFile("README.md")),
        ]
        .spacing(8)
        .padding(10),
    )
    .width(Length::Fixed(230.0))
    .height(Length::Fill);

    // Iced's TextEditor is used only as an initial editable prototype.
    // Replace it with a custom virtualized widget backed by de-pinna-editor-core.
    let editor = container(
        column![
            row![
                text(app.selected_file).size(13),
                iced::widget::Space::new().width(Length::Fill),
                text("Rust").size(12),
            ]
            .spacing(8),
            text_editor(&app.editor)
                .on_action(Message::EditorAction)
                .font(Font::MONOSPACE)
                .size(15)
                .height(Length::Fill),
        ]
        .spacing(12)
        .padding(14),
    )
    .width(Length::Fill)
    .height(Length::Fill);

    let mut center = column![editor].height(Length::Fill).spacing(8);
    if app.show_terminal {
        center = center.push(
            container(
                column![
                    text("TERMINAL").size(12),
                    text("PTY sessions will be hosted by de-pinna-terminal.")
                        .font(Font::MONOSPACE)
                        .size(13),
                ]
                .spacing(8)
                .padding(12),
            )
            .height(Length::Fixed(150.0))
            .width(Length::Fill),
        );
    }

    let body = row![explorer, container(center).width(Length::Fill).height(Length::Fill)]
        .height(Length::Fill)
        .spacing(1);

    container(column![toolbar, body].spacing(1))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
