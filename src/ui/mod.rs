use crate::recorder::Recorder;
use crate::openai::{self, OpenAiSpeechToTextRes, OpenAiCompletionRes};
use iced::widget::{button, column, text, row, container, vertical_space, horizontal_space, text_input};
use iced::{Alignment,alignment,  Element, Sandbox, Settings, Length, Color, theme, Theme};

pub fn render() -> iced::Result {
    App::run(Settings {
        default_font: super::get_font(),
    ..Settings::default()
    })
}

struct App {
    api_key: String,
    items: Vec<Item>,
    input: String,
    recorder: Recorder,
}

#[derive(Debug, Clone)]
enum Message {
    AudioStartRecord,
    AudioStopRecord,
    AudioSpeechToText,

    ApiKeyInputChanged(String),
    ApiKeyInputSubmit(String),

    TextInputChanged(String),
    TextInputSubmit(String)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ChatContainer {
    #[default]
    ContentStyle
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Item {
    name: String,
    text: String,
}

impl container::StyleSheet for ChatContainer {
    type Style = Theme;

    fn appearance(&self, _theme: &Theme) -> container::Appearance {
        container::Appearance {
            border_color: Color::from_rgb8(200, 200, 200),
            border_width: 2.0,
            border_radius: 3.0,
            ..Default::default()
        }
    }
}

impl App {
    pub fn handle_completion(&mut self, input: String) {
        let res: OpenAiCompletionRes = openai::send_completion(input.clone(), self.api_key.clone()).unwrap();    

        match res.error {
            Some(error) => { self.items.push(Item {name: String::from("ChatGPT (Error):"), text: error.message.unwrap()}); }
            None => {
                res.choices.unwrap().iter().for_each(|choice| {
                    // chatgpt answer output
                    self.items.push(Item {name: String::from("ChatGPT:"), text: choice.text.clone().unwrap()});
                });
            }
        };
    }
}

impl<'a> Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self { api_key: String::from(""), items: Vec::new(), input: String::from(""), recorder: Recorder::new().unwrap() }
    }

    fn title(&self) -> String {
        String::from("OpenAI ChatGPT by Voice")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::AudioStartRecord => {
                let _ = self.recorder.begin_record().unwrap();
            }
            Message::AudioStopRecord => {
                let _ = self.recorder.stop_record().unwrap();
            }
            Message::ApiKeyInputChanged(api_key) => {
                self.api_key = api_key;
            }
            Message::ApiKeyInputSubmit(api_key) => {
                self.api_key = api_key;
            },
            Message::AudioSpeechToText => {
                let res: OpenAiSpeechToTextRes = openai::send_speech_to_text(super::get_file_path(), self.api_key.clone()).unwrap();

                match res.error {
                    Some(error) => { self.items.push(Item {name: String::from("ChatGPT (Error):"), text: error.message.unwrap()}); }
                    None => {
                        self.items.push(Item {name: String::from("You (by voice):"), text: res.text.clone().unwrap()});
                        self.handle_completion(res.text.clone().unwrap());
                    }
                }
            },
            Message::TextInputChanged(input) => {
                self.input = input;
            },
            Message::TextInputSubmit(input) => {
                self.input = input;
                self.items.push(Item {name: String::from("You (by text):"), text: self.input.clone()});
                self.handle_completion(self.input.clone());
                self.input = String::from("");
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let top_buttons = container(row![
                text("Record Your Voice").size(24).vertical_alignment(alignment::Vertical::Bottom),
                horizontal_space(Length::Fixed(10.0)),
                button("Start Record").on_press(Message::AudioStartRecord).style(theme::Button::Destructive),
                horizontal_space(Length::Fixed(10.0)),
                button("Stop Record").on_press(Message::AudioStopRecord).style(theme::Button::Destructive),
                horizontal_space(Length::Fixed(10.0)),
                button("Speech To Text").on_press(Message::AudioSpeechToText).style(theme::Button::Primary),
                horizontal_space(Length::Fixed(10.0)),
                text_input("OpenAI Api Key", &self.api_key)
                    .on_input(Message::ApiKeyInputChanged)
                    .on_submit(Message::ApiKeyInputSubmit(self.api_key.clone())),
            ].align_items(Alignment::Start))
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Left)
            .align_y(alignment::Vertical::Bottom);

        // add chat messages
        let chat_elements: Element<Message> = column(
            self.items
                .iter()
                .enumerate()
                .map(|(_i, task)| {
                    row![
                        text(task.name.clone()).size(14).vertical_alignment(alignment::Vertical::Bottom),
                        text(task.text.clone()).size(24).vertical_alignment(alignment::Vertical::Bottom),
                    ]
                    .spacing(10)
                    .align_items(Alignment::Center)
                    .into()
                })
                .collect(),
        )
        .spacing(10)
        .into();

        let chat_content = container(row![
            chat_elements,
        ])
        .height(Length::Fill)
        .width(Length::Fill)
        .style(theme::Container::Custom(Box::new(ChatContainer::ContentStyle)));

        let bottom_content = container(
            row![
                text_input("Your Message", &self.input)
                    .on_input(Message::TextInputChanged)
                    .on_submit(Message::TextInputSubmit(self.input.clone())),
                button("Send")
                    .on_press(Message::TextInputSubmit(self.input.clone())),
            ]
            .spacing(10)
        );

        column![
            top_buttons,
            vertical_space(Length::Fixed(10.0)),
            chat_content,
            vertical_space(Length::Fixed(10.0)),
            bottom_content,
        ]
        .height(Length::Fill)
        .width(Length::Fill)
        .padding(20)
        .into()
    }
}
