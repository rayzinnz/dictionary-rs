//check the lazy example

use std::ops::Deref;

use iced::{Fill, Font, Length::{self, FillPortion}, Size, Task, font, widget::{self, Column, Grid, Row, button, column, container, grid, lazy, operation, row, scrollable, text, text_input}};

use crate::sql::{Word, WordShort};

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
    selected_word: Option<Word>,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    WordsReceived(Result<Vec<WordShort>, String>),
    WordReceived(Result<Option<Word>, String>),
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
                    Task::future(async move {
                            sql::get_words(&input).await
                        })
                        .map_err(|e| e.to_string())
                        .map(Message::WordsReceived)
                } else {
                    Task::done(Message::WordsReceived(Ok(Vec::new())))
                }
            },
            Message::WordsReceived(Ok(words)) => {
                self.version = self.version.wrapping_add(1);
                self.items = words.clone();
                match words.first() {
                    Some(word) => {
                        let word = word.clone();
                        Task::future(async move {
                                sql::get_word(word.words_rid).await
                            })
                            .map_err(|e| e.to_string())
                            .map(Message::WordReceived)
                    },
                    None => Task::done(Message::WordReceived(Ok(None)))
                }
            },
            Message::WordReceived(Ok(word_op)) => {
                self.selected_word = word_op;
                Task::none()
            },
            //error handling
            Message::WordsReceived(Err(e)) => {
                panic!("Error receiving words from db: {}", e)
            },
            Message::WordReceived(Err(e)) => {
                panic!("Error receiving a word from db: {}", e)
            },
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
        
        let selected_word_word = match &self.selected_word {
            Some(word) => word.word.clone(),
            None => String::new()
        };
        let synonyms = match &self.selected_word {
            Some(word) => word.synonyms.clone(),
            None => Vec::new()
        };
        let synonym_list = column(
                synonyms.into_iter().map(|item|
                    text(item).into()
                )
            );
        let antonyms = match &self.selected_word {
            Some(word) => word.antonyms.clone(),
            None => Vec::new()
        };
        let antonym_list = column(
                antonyms.into_iter().map(|item|
                    text(item).into()
                )
            );
        let definitions = match &self.selected_word {
            Some(word) => word.definitions.clone(),
            None => Vec::new()
        };
        let definition_list = column(
                definitions.into_iter().map(|item|
                    text(item.definition)
                    .font(Font{style: font::Style::Italic, ..Font::DEFAULT})
                    .into()
                )
            ).spacing(10);

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
                    column![
                        text(selected_word_word).font(Font {weight: font::Weight::Bold, ..Font::DEFAULT}),
                        row![
                            column![
                                text("synonyms").font(Font{style: font::Style::Italic, ..Font::DEFAULT}),
                                scrollable(synonym_list)
                            ],
                            column![
                                text("antonyms").font(Font{style: font::Style::Italic, ..Font::DEFAULT}),
                                scrollable(antonym_list)
                            ],
                            column![
                                text("definitions"),
                                scrollable(definition_list)
                            ].width(FillPortion(1)),
                        ]
                        .padding(20)
                        .spacing(20)
                    ]
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

