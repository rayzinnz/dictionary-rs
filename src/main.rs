//check the lazy example

use iced::{Fill, Length, Size, Task, widget::{self, Column, Grid, Row, button, column, container, grid, lazy, operation, row, scrollable, text, text_input}};

use crate::sql::WordShort;

mod sql;

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
    .window_size(Size::new(1200.0, 600.0))
    .run()
}

// derive default here, so defaults are auto made
#[derive(Default)]
struct App {
    version: u8,
    search_string: String,
    items: Vec<WordShort>,
    selected_word: String,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    WordsReceived(Result<Vec<WordShort>, String>),
}

impl App {
    fn new() -> (Self, Task<Message>) {
        (Self {
            ..Default::default()
            },
            operation::focus("search")
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::InputChanged(input) => {
                self.search_string = input.clone();
                if input.len()>=3 {
                    println!("input: {}", input);
                    Task::future(async move {sql::get_words(&input).await}).map_err(|e| e.to_string()).map(Message::WordsReceived)
                } else {
                    Task::done(Message::WordsReceived(Ok(Vec::new())))
                }
            },
            Message::WordsReceived(Ok(words)) => {
                self.version = self.version.wrapping_add(1);
                self.selected_word = match words.first() {
                    Some(word) => word.word,
                    None => String::new()
                };
                self.items = words;
                Task::none()
            },

            Message::WordsReceived(Err(e)) => {
                panic!("Error receiving words from db: {}", e)
            }
        }
    }

    fn view(&self) -> Column<'_, Message> {
        let word_list = lazy(self.version, |_| {
            let items: Vec<_> = self.items.iter().cloned().collect();

            column(items.into_iter().map(|item| {
                row![
                    text(item.word.clone()),
                ]
                .spacing(20)
                .into()
            }))
            .spacing(10)
            
        });

        column![
            text_input("Search", &self.search_string)
                .id("search")
                .on_input(Message::InputChanged)
                .style(|theme, status| text_input::default(theme, status))
            ,
            row![
                container(
                    scrollable(word_list)
                    .height(Fill)
                    .width(Fill)
                ,)
                .width(Length::FillPortion(1))
                .style(|theme| container::bordered_box(theme)),
                container(
                    text(&self.selected_word)
                )
                .width(Length::FillPortion(3))
                
            ]
            .padding(20)
            .spacing(10)
            ,
        ]
        .padding(20)
        .spacing(10)
    }
}

