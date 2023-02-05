use iced::widget::{ Space, slider, checkbox, container, button, text, row};
use iced::widget::column as col;
use iced::{Application, Length, Color, alignment, theme};
use iced_native::widget::menu::{MenuBar, MenuTree};
use iced_native::widget::quad;

pub fn main() -> iced::Result{
    App::run(iced::Settings{
        default_text_size: 15,
        window: iced::window::Settings{
            size: (800, 500),
            // position: iced::window::Position::Default,
            ..Default::default()
        },
        ..Default::default()
    })
}

#[derive(Debug, Clone)]
enum Message{
    Hello,
    Debug(String),
    ValueChange(u8),
    CheckChange(bool),
    ColorChange(Color),
    Flip,
}

struct App{
    title: String,
    value: u8,
    check: bool,
    theme: iced::Theme,
    flip: bool,
}
impl Application for App{
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let theme = iced::Theme::custom(theme::Palette{
            background: Color::from([0.8;3]),
            primary: Color::from([0.82, 0.38, 0.55]),
            ..iced::Theme::Light.palette()
        });

        (
            Self{
                title: "Menu Test".to_string(),
                value: 0,
                check: false,
                theme,
                flip: false,
            },
            iced::Command::none()
        )
    }

    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message{
            Message::Debug(s) => {
                println!("dbg msg: {}", s);
                self.title = s.clone();

                let plt = self.theme.palette();
                let primary = match s.as_str() {
                    "Twilight"   => Color::from([0.45, 0.25, 0.57]),
                    "Pinkie"     => Color::from([0.82, 0.26, 0.41]),
                    "Fluttershy" => Color::from([0.83, 0.75, 0.43]),
                    "Rarity"     => Color::from([0.33, 0.24, 0.60]),
                    "Applejack"  => Color::from([0.84, 0.45, 0.14]),
                    "Rainbow"    => Color::from([0.08, 0.57, 0.81]),
                    _ => plt.primary,
                };
                self.theme = iced::Theme::custom(theme::Palette{
                    primary,
                    ..plt
                })
                /*
                [0.45, 0.25, 0.57]
                [0.82, 0.26, 0.41]
                [0.83, 0.75, 0.43]
                [0.33, 0.24, 0.60]
                [0.84, 0.45, 0.14]
                [0.08, 0.57, 0.81]
                */
            },
            Message::ValueChange(v) => {
                self.value = v;
                self.title = v.to_string();
            },
            Message::CheckChange(c) => {
                self.check = c;
                self.title = c.to_string();
            }
            Message::ColorChange(c) => {
                self.theme = iced::Theme::custom(theme::Palette{
                    primary: c,
                    ..self.theme.palette()
                });
                self.title = format!("[{:.2}, {:.2}, {:.2}]", c.r, c.g, c.b);
                
            },
            Message::Flip => self.flip = !self.flip,
            _ => ()
        }
        iced::Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        
        let mb = MenuBar::new(vec![
            menu_1(self),
            menu_2(self),
            menu_3(self),
        ])
            .spacing(4)
            .item_size([180.0, 25.0])
            ;
        
        let r = row!(
            Space::new(Length::Units(4), Length::Shrink),
            mb,
        ).padding([2,0]);

        let top_bar_style:fn(&iced::Theme)->container::Appearance = |_theme|{
            container::Appearance{
                background: Some(Color::from([0.8;3]).into()),
                ..Default::default()
            }
        };
        let top_bar = container(r)
            .width(Length::Fill)
            .style(top_bar_style)
            ;

        let back_style:fn(&iced::Theme)->container::Appearance = |theme|{
            container::Appearance{
                background: Some(theme.palette().primary.into()),
                ..Default::default()
            }
        };
        let back = container(col![])
            .width(Length::Fill)
            .height(Length::Fill)
            .style(back_style)
            ;

        let c = if self.flip{
            col![
                back,
                top_bar,
            ]
        }else{
            col![
                top_bar,
                back,
            ]
        };

        c.into()
    }
}


struct ButtonStyle;
    impl button::StyleSheet for ButtonStyle{
        type Style = iced::Theme;

        fn active(&self, _style: &Self::Style) -> button::Appearance {
            button::Appearance{
                text_color: Color::BLACK,
                border_radius: 4.0.into(),
                background: Some(Color::TRANSPARENT.into()),
                // background: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.2).into()),
                ..Default::default()
            }
        }

        // fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        //     button::Appearance{
        //         // background: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.8).into()),
        //         background: Some(Color::TRANSPARENT.into()),
        //         ..Default::default()
        //     }
        // }
    }

fn dbb_m(label: &str, msg: Message) -> button::Button<'_, Message, iced::Renderer>{
    button(
        text(label)
            .width(Length::Fill)
            .height(Length::Fill)
            // .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center)
    )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding([4, 8])
        .style(iced::theme::Button::Custom(Box::new(ButtonStyle{})))
        .on_press(msg)
}

fn dbb(label: &str) -> button::Button<'_, Message, iced::Renderer>{
    button(
        text(label)
            .width(Length::Fill)
            .height(Length::Fill)
            // .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center)
    )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding([4, 8])
        .style(iced::theme::Button::Custom(Box::new(ButtonStyle{})))
        .on_press(Message::Debug(label.to_string()))
}

fn hb(label: &str) -> button::Button<'_, Message, iced::Renderer>{
    button(label)
        // .width(Length::Fill)
        .padding([4, 8])
        .style(iced::theme::Button::Custom(Box::new(ButtonStyle{})))
        .on_press(Message::Debug(label.to_string()))
}

fn dbb_mt(label: &str) -> MenuTree<'_, Message, iced::Renderer>{
    MenuTree::new(dbb(label))
}

fn subm<'a>(
    label: &str, 
    children: Vec<MenuTree<'a, Message, iced::Renderer>>
) -> MenuTree<'a, Message, iced::Renderer>{
    MenuTree::<Message, iced::Renderer>::with_children(
        button(
            row![
                text(label)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .vertical_alignment(alignment::Vertical::Center),
                text(" > ")
                    .width(Length::Shrink)
                    .height(Length::Fill)
                    .size(20)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            ]
        )
            .width(Length::Fill)
            .height(Length::Fill)
            .padding([4, 8])
            .style(iced::theme::Button::Custom(Box::new(ButtonStyle{})))
            .on_press(Message::Debug(label.to_string())),
        children
    )
}

fn separator<'a>() -> MenuTree<'a, Message, iced::Renderer>{
    MenuTree::new(quad::Quad{
        color: [0.5;3].into(),
        border_radius: 4.0.into(),
        inner_bounds: quad::InnerBounds::Ratio(0.98, 0.1),
        ..Default::default()
    })
}



fn menu_1<'a>(app: &App) -> MenuTree<'a, Message, iced::Renderer>{
    let crack_it = subm(
        "crack it",
        vec![
            dbb_mt("Name it"),
            dbb_mt("read it"),
            dbb_mt("tune it"),
            dbb_mt("print it"),
            dbb_mt("scan it"),
            dbb_mt("send it"),
            dbb_mt("fax"),
            dbb_mt("rename it"),
        ]
    );
    let find_it = subm(
        "find it",
        vec![
            dbb_mt("Surf it"),
            dbb_mt("scroll it"),
            dbb_mt("pause it"),
            dbb_mt("click it"),
            dbb_mt("cross it"),
            crack_it,
            dbb_mt("switch"),
            dbb_mt("update it"),
        ]
    );
    let play_it = subm(
        "play it",
        vec![
            dbb_mt("Lock it"),
            dbb_mt("fill it"),
            dbb_mt("call it"),
            dbb_mt("view it"),
            dbb_mt("code it"),
            dbb_mt("jam"),
            find_it,
            dbb_mt("unlock it"),
        ]
    );
    let cut_it = subm(
        "cut it",
        vec![
            dbb_mt("Plug it"),
            play_it,
            dbb_mt("burn it"),
            dbb_mt("rip it"),
            dbb_mt("rip it"),
        ]
    );
    let upgrade_it = subm(
        "upgrade it",
        vec![
            dbb_mt("Write it"),
            cut_it,
            dbb_mt("paste it"),
            dbb_mt("save it"),
            separator(),
            dbb_mt("load it"),
            dbb_mt("check it"),
            dbb_mt("quick rewrite it"),
        ]
    );

    let break_it = subm(
        "break it",
        vec![
            dbb_mt("Charge it"),
            dbb_mt("point it"),
            dbb_mt("zoom it"),
            dbb_mt("press it"),
            dbb_mt("snap it"),
            dbb_mt("work it"),
            dbb_mt("quick erase it"),
        ]
    );

    let root = MenuTree::with_children(
        hb("Technologic"),
        vec![
            dbb_mt("Buy it"),
            dbb_mt("use it"),
            break_it,
            separator(),
            dbb_mt("fix it"),
            dbb_mt("trash it"),
            dbb_mt("change it"),
            dbb_mt("mail"),
            upgrade_it,
        ]
    );

    root
}

fn menu_2<'a>(app: &App) -> MenuTree<'a, Message, iced::Renderer>{
    let twilight = subm(
        "Twilight",
        vec![
            dbb_mt("Books"),
            dbb_mt("Pudding"),
            dbb_mt("Magic"),
            dbb_mt("Princess"),
            dbb_mt("Neeeerrrrrd"),
        ]
    );

    let pinkie = subm(
        "Pinkie",
        vec![
            dbb_mt("My name is Pinkie Pie"),
            dbb_mt("Hello"),
            dbb_mt("And I am here to say:"),
            dbb_mt("You better get ready to die!"),
        ]
    );

    let ms = subm(
        "M6",
        vec![
            twilight,
            pinkie,
            dbb_mt("Fluttershy"),
            dbb_mt("Rarity"),
            dbb_mt("Applejack"),
            dbb_mt("Rainbow"),
        ]
    );

    let starlight = subm(
        "Starlight",
        vec![
            dbb_mt("Magic"),
            dbb_mt("Trixie"),
            dbb_mt("Trixie"),
            dbb_mt("Trixie"),
        ]
    );

    let trixie = subm(
        "Trixie",
        vec![
            dbb_mt("Magic"),
            dbb_mt("Starlight"),
            dbb_mt("Starlight"),
            dbb_mt("Starlight"),
        ]
    );

    let vil = subm(
        "V8",
        vec![
            dbb_mt("Sunset"),
            starlight,
            trixie,
            separator(),
            dbb_mt("Discord"),
            dbb_mt("Trek"),
            dbb_mt("Chrysalis"),
            dbb_mt("Sambra"),
            dbb_mt("Cozy Glow"),
        ]
    );
    
    let root = MenuTree::with_children(
        hb("FIM"),
        vec![
            dbb_mt("Lyra"),
            dbb_mt("Bon Bon"),
            ms,
            separator(),
            dbb_mt("Apple Bloom"),
            dbb_mt("Sweetie Belle"),
            dbb_mt("Scootaloo"),
            vil,
            separator(),
            dbb_mt("Doctor Wh0000ves"),
            dbb_mt("Derpyyyy"),

            MenuTree::new( slider(0..=255, app.value, Message::ValueChange) ),
            MenuTree::new( checkbox("Check Me", app.check, Message::CheckChange)
                .size(17)
                .spacing(10)
                .width(Length::Fill)
            ),
        ]
    );

    root
}

fn menu_3<'a>(app: &App) -> MenuTree<'a, Message, iced::Renderer>{

    let [r,g,b,_] = app.theme.palette().primary.into_rgba8();
    
    let primary = subm(
        "Primary",
        vec![
            MenuTree::new(slider(0..=255, r, move |x| Message::ColorChange( Color::from_rgb8(x, g, b) ) )),
            MenuTree::new(slider(0..=255, g, move |x| Message::ColorChange( Color::from_rgb8(r, x, b) ) )),
            MenuTree::new(slider(0..=255, b, move |x| Message::ColorChange( Color::from_rgb8(r, g, x) ) )),
        ]
    );

    let root = MenuTree::with_children(
        hb("Controls"),
        vec![
            primary,
            MenuTree::new(dbb_m("Flip", Message::Flip)),
        ]
    );

    root
}


