use super::{Client, View, ClientMessage};
use protocol::network::NetworkMessage;

use iced::{
    Application, Element, Row, Length, TextInput, Button, Text, Container,
    Column, Scrollable, Color,
};

impl Client {
    pub fn get_view(&mut self) -> Element<'_, <Self as Application>::Message> {
        match &mut self.view {
            View::Home { input, submit } => {
                let form: Row<_> = Row::new()
                    .width(Length::Units(256))
                    .spacing(7)
                    .push(
                        TextInput::new(input, "Username", &self.username, ClientMessage::UpdateUsername)
                            .on_submit(ClientMessage::SubmitUsername)
                            .style(style::TextInput)
                            .padding(5),
                    )
                    .push(
                        Button::new(submit, Text::new("Login"))
                            .on_press(ClientMessage::SubmitUsername)
                            .style(style::Button)
                            .padding(5),
                    );

                Container::new(form)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .into()
            }
            View::SelectServer { buttons, servers } => {
                while buttons.len() <= servers.len() {
                    buttons.push(iced::button::State::default());
                }

                let mut buttons = buttons.iter_mut();
                let col = Column::new()
                    .width(Length::Shrink)
                    .spacing(5)
                    .push(
                        Button::new(
                            buttons.next().unwrap(),
                            Text::new("Refresh")
                                .horizontal_alignment(iced::HorizontalAlignment::Center),
                        )
                        .on_press(ClientMessage::RefreshServerList)
                        .width(Length::Units(256))
                        .style(style::Button)
                        .padding(5),
                    )
                    .push(Row::new().height(Length::Units(15)));

                let list = match servers.len() {
                    0 => col.push(
                        Text::new("No server found")
                            .horizontal_alignment(iced::HorizontalAlignment::Center)
                            .width(Length::Units(256))
                            .color(Color::WHITE),
                    ),
                    _ => servers.iter().zip(buttons).fold(
                        col,
                        |container, ((entree, addr), state)| {
                            container.push(
                                Button::new(
                                    state,
                                    Text::new(entree.to_owned())
                                        .horizontal_alignment(iced::HorizontalAlignment::Center),
                                )
                                .on_press(ClientMessage::SelectServer(addr.to_owned()))
                                .width(Length::Units(256))
                                .style(style::Button)
                                .padding(5),
                            )
                        },
                    ),
                };

                Container::new(list)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .into()
            }
            View::Chat { messages, users, scroll_view, input, message, .. } => {
                let users_col = Column::new()
                    .width(Length::Units(180))
                    .height(Length::Fill)
                    .padding(5)
                    .spacing(5)
                    .push(Container::new(Text::new(&self.username))
                        .style(style::SelfContainer)
                        .width(Length::Fill)
                        .padding(7)
                    );

                let users_col = users.values().fold(users_col, |users, username| {
                    users.push(Container::new(Text::new(username))
                        .style(style::GuestContainer)
                        .width(Length::Fill)
                        .padding(7)
                    )
                });

                let scroll_view = messages.iter().fold(
                    Scrollable::new(scroll_view)
                        .width(Length::Fill)
                        .spacing(5),
                    |scroll, (msg, from)| {
                        match msg {
                            NetworkMessage::UserList(list) => {
                                match list.users().len() {
                                    0 => scroll,
                                    1 => scroll.push(
                                        Text::new(format!("Join a server with {}", list.users().first().unwrap().1))
                                            .color(Color::from_rgb(0.6, 0.6, 0.6))
                                    ),
                                    len => {
                                        let (_, last) = list.users().last().unwrap();
                                        let mut init = String::with_capacity(128);
                                        init.push_str(&list.users().first().unwrap().1);

                                        let msg = list.users()[1..len - 1].iter().fold(init, |mut msg, (_, user)| {
                                            msg.push_str(", ");
                                            msg.push_str(user);
                                            msg
                                        });

                                        scroll.push(
                                        Text::new(format!("Join a server with {} and {}.", msg, last))
                                            .color(Color::from_rgb(0.6, 0.6, 0.6))
                                        )
                                    }
                                }
                            }
                            NetworkMessage::UserJoin(join) => scroll.push(
                                Text::new(format!("{} joined the server", join.name()))
                                    .color(Color::from_rgb(0.6, 0.6, 0.6))
                            ),
                            NetworkMessage::UserLeave(_) => scroll.push(
                                Text::new(format!("{} left the server", from))
                                    .color(Color::from_rgb(0.6, 0.6, 0.6))
                            ),
                            NetworkMessage::Message(msg) => scroll.push(Row::new()
                                .push(Text::new(format!("{}: ", from)).color(Color::from_rgb(0.0, 3.0, 5.0)))
                                .push(Text::new(msg.content()).color(Color::WHITE))
                            ),
                            _ => scroll
                        }
                    }
                );

                let input = TextInput::new(input, "Envoyez un message", message, ClientMessage::UpdateMessage)
                    .on_submit(ClientMessage::SendMessage)
                    .style(style::TextInput)
                    .padding(7);

                let chat_col = Column::new()
                    .padding(5)
                    .push(iced::Space::new(
                        Length::Fill,
                        Length::Fill)
                    )
                    .push(scroll_view)
                    .push(iced::Space::new(
                        Length::Fill,
                        Length::Units(7))
                    )
                    .push(input);

                Row::new()
                    .push(users_col)
                    .push(iced::Rule::vertical(0).style(style::Rule))
                    .push(chat_col)
                    .into()
            }
        }
    }

    pub fn get_background_color(&self) -> Color {
        Color::from_rgb(0.0, 0.05, 0.1)
    }
}

mod style {
    use iced::{Color, button, container, rule, text_input};

    const SURFACE: Color = Color::from_rgb(0.05, 0.1, 0.2);

    const ACCENT: Color = Color::from_rgb(
        0x6F as f32 / 255.0,
        0xFF as f32 / 255.0,
        0xE9 as f32 / 255.0,
    );

    const ACTIVE: Color = Color::from_rgb(
        0x64 as f32 / 255.0,
        0x95 as f32 / 255.0,
        0xED as f32 / 255.0,
    );

    const HOVERED: Color = Color::from_rgb(
        0x41 as f32 / 255.0,
        0x69 as f32 / 255.0,
        0xE1 as f32 / 255.0,
    );

    pub struct TextInput;
    impl text_input::StyleSheet for TextInput {
        fn active(&self) -> text_input::Style {
            text_input::Style {
                background: SURFACE.into(),
                border_radius: 2.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            }
        }

        fn focused(&self) -> text_input::Style {
            text_input::Style {
                border_width: 1.0,
                border_color: ACCENT,
                ..self.active()
            }
        }

        fn placeholder_color(&self) -> Color {
            Color::from_rgb(0.5, 0.5, 0.5)
        }

        fn value_color(&self) -> Color {
            Color::WHITE
        }

        fn selection_color(&self) -> Color {
            ACTIVE
        }
    }

    pub struct Button;
    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: ACTIVE.into(),
                text_color: Color::WHITE,
                border_radius: 3.0,
                ..button::Style::default()
            }
        }

        fn hovered(&self) -> button::Style {
            button::Style {
                background: HOVERED.into(),
                ..self.active()
            }
        }

        fn pressed(&self) -> button::Style {
            button::Style {
                border_color: Color::WHITE,
                border_width: 1.0,
                ..self.hovered()
            }
        }
    }

    pub struct SelfContainer;
    impl container::StyleSheet for SelfContainer {
        fn style(&self) -> container::Style {
            container::Style {
                text_color: Some(Color::WHITE),
                background: Some(iced::Background::Color(
                    Color::from_rgb8(60, 180, 110)
                )),
                border_radius: 3.0,
                ..Default::default()
            }
        }
    }

    pub struct GuestContainer;
    impl container::StyleSheet for GuestContainer {
        fn style(&self) -> container::Style {
            container::Style {
                text_color: Some(Color::WHITE),
                border_radius: 3.0,
                border_width: 1.0,
                border_color: Color::from_rgb(0.5, 0.5, 0.5),
                ..Default::default()
            }
        }
    }

    pub struct Rule;
    impl rule::StyleSheet for Rule {
        fn style(&self) -> rule::Style {
            rule::Style {
                color: Color::from_rgb(0.5, 0.5, 0.5),
                width: 1,
                radius: 0.0,
                fill_mode: rule::FillMode::Full,
            }
        }
    }
}
