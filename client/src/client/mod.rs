mod incoming_messages;
mod events;
mod ui;

use std::net::{SocketAddr, TcpStream};
use std::collections::HashMap;

use iced::{
    Element, Application, Command, Clipboard, Subscription, Color, executor,
};

use protocol::network::NetworkMessage;

#[derive(Default)]
pub struct Client {
    view: View,
    username: String,
}

enum View {
    Home {
        input: iced::text_input::State,
        submit: iced::button::State,
    },
    SelectServer {
        buttons: Vec<iced::button::State>,
        servers: Vec<(String, SocketAddr)>,
    },
    Chat {
        messages: Vec<(NetworkMessage, String)>,
        users: HashMap<u32, String>,
        scroll_view: iced::scrollable::State,
        input: iced::text_input::State,
        socket: TcpStream,
        personal_id: u32,
        message: String,
    },
}

#[derive(Debug, Clone)]
pub enum ClientMessage {
    UpdateUsername(String),
    SubmitUsername,
    RefreshServerList,
    SelectServer(SocketAddr),
    UpdateMessage(String),
    SendMessage,
    IncomingMessages(NetworkMessage),
}

impl Application for Client {
    type Executor = executor::Default;
    type Message = ClientMessage;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Default::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("messaging")
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        self.get_subscription()
    }

    fn update(&mut self, message: Self::Message, clipboard: &mut Clipboard) -> Command<Self::Message> {
        self.update_ui(message, clipboard)
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        self.get_view()
    }

    fn background_color(&self) -> Color {
        self.get_background_color()
    }
}

impl Default for View {
    fn default() -> Self {
        Self::Home {
            input: iced::text_input::State::default(),
            submit: iced::button::State::default(),
        }
    }
}
