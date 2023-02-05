use iced::widget::{ Space, slider, checkbox, container, button, text, row, toggler, horizontal_space};
use iced::widget::column as col;
use iced::{Application, Length, Color, alignment, theme, Element};

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
    ToggleChange(bool),
    ColorChange(Color),
    Flip,
    ThemeChange(bool),
}

struct App{
    title: String,
    value: u8,
    check: bool,
    toggle: bool,
    theme: iced::Theme,
    flip: bool,
    dark_mode: bool,
}
impl Application for App{
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let theme = iced::Theme::custom(theme::Palette{
            background: Color::from([0.8;3]),
            // primary: Color::from([0.82, 0.38, 0.55]),
            primary: Color::from([0.45, 0.25, 0.57]),
            ..iced::Theme::Light.palette()
        });
        // let theme = iced::Theme::Light;

        (
            Self{
                title: "Menu Test".to_string(),
                value: 0,
                check: false,
                toggle: false,
                theme,
                flip: false,
                dark_mode:false,
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
                // println!("dbg msg: {}", s);
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
            Message::ToggleChange(t) => {
                self.toggle = t;
                self.title = t.to_string();
            }
            Message::ColorChange(c) => {
                self.theme = iced::Theme::custom(theme::Palette{
                    primary: c,
                    ..self.theme.palette()
                });
                self.title = format!("[{:.2}, {:.2}, {:.2}]", c.r, c.g, c.b);
                
            },
            Message::Flip => self.flip = !self.flip,
            Message::ThemeChange(b) => {
                self.dark_mode = b;
                if b {
                    self.theme = iced::Theme::Dark;
                }else{
                    self.theme = iced::Theme::Light;
                }
            }
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
            .bounds_expand(30)
            ;
        
        let r = row!(
            Space::new(Length::Units(4), Length::Shrink),
            mb,
        ).padding([2,0]);

        let top_bar_style:fn(&iced::Theme)->container::Appearance = |_theme|{
            container::Appearance{
                // background: Some(Color::from([0.8;3]).into()),
                background: Some(Color::TRANSPARENT.into()),
                ..Default::default()
            }
        };
        let top_bar = container(r)
            .width(Length::Fill)
            .style(top_bar_style)
            ;

        let back_style:fn(&iced::Theme)->container::Appearance = |theme|{
            container::Appearance{
                background: Some(theme.extended_palette().primary.base.color.into()),
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

        fn active(&self, style: &Self::Style) -> button::Appearance {
            button::Appearance{
                // text_color: Color::BLACK,
                text_color: style.extended_palette().background.base.text,
                border_radius: 4.0.into(),
                background: Some(Color::TRANSPARENT.into()),
                // background: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.2).into()),
                ..Default::default()
            }
        }

        fn hovered(&self, style: &Self::Style) -> button::Appearance {
            let plt = style.extended_palette();
            
            button::Appearance{
                // background: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.8).into()),
                // background: Some(Color::TRANSPARENT.into()),
                border_radius: 4.0.into(),
                background: Some(plt.primary.weak.color.into()),
                text_color: plt.primary.weak.text,
                ..self.active(style)
            }
        }
    }



fn base_button<'a>(
    content: impl Into<Element<'a, Message, iced::Renderer>>, 
    msg: Message
) -> button::Button<'a, Message, iced::Renderer>{
    button(content)
        .padding([4, 8])
        .style(iced::theme::Button::Custom(Box::new(ButtonStyle{})))
        .on_press(msg)
}

fn labeled_button<'a>(
    label: &str,
    msg: Message
) -> button::Button<'a, Message, iced::Renderer>{
    base_button(
        text(label)
            .width(Length::Fill)
            .height(Length::Fill)
            .vertical_alignment(alignment::Vertical::Center),
        msg
    )
}

fn debug_button<'a>(
    label: &str,
) -> button::Button<'a, Message, iced::Renderer>{
    labeled_button(label, Message::Debug(label.into()))
}

fn debug_item<'a>(
    label: &str,
) -> MenuTree<'a, Message, iced::Renderer>{
    MenuTree::new(
        debug_button(label)
            .width(Length::Fill)
            .height(Length::Fill)
    )
}


fn sub_menu<'a>(
    label: &str, 
    msg: Message, 
    children: Vec<MenuTree<'a, Message, iced::Renderer>>
) -> MenuTree<'a, Message, iced::Renderer>{
    MenuTree::with_children(
        base_button(
            row![
                text(label)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .vertical_alignment(alignment::Vertical::Center), 
                text(" > ")
                    .height(Length::Fill)
                    .vertical_alignment(alignment::Vertical::Center), 
            ],
            msg
        )
        .width(Length::Fill)
        .height(Length::Fill),
        children
    )
}

fn debug_sub_menu<'a>(
    label: &str, 
    children: Vec<MenuTree<'a, Message, iced::Renderer>>
) -> MenuTree<'a, Message, iced::Renderer>{
    sub_menu(label, Message::Debug(label.into()), children)
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
    
    let sub_5 = debug_sub_menu(
        "SUB",
        vec![
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
            
        ]
    );
    let sub_4 = debug_sub_menu(
        "SUB",
        vec![
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
            
        ]
    );
    let sub_3 = debug_sub_menu(
        "More sub menus",
        vec![
            debug_item("You can"),
            debug_item("nest menus"),
            sub_4,
            debug_item("how ever"),
            debug_item("You like"),
            sub_5,
        ]
    );
    let sub_2 = debug_sub_menu(
        "Another sub menu",
        vec![
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
            sub_3,
            debug_item("Item"),
        ]
    );
    let sub_1 = debug_sub_menu(
        "A sub menu",
        vec![
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
            sub_2,
            debug_item("Item"),
        ]
    );

    let root = MenuTree::with_children(
        debug_button("Nested Menus"),
        vec![
            debug_item("Item"),
            debug_item("Item"),
            sub_1,
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
        ]
    );

    root
}

fn menu_2<'a>(app: &App) -> MenuTree<'a, Message, iced::Renderer>{
    
    let bt = MenuTree::new(
        button(
            text("Button")
            .width(Length::Fill)
            .height(Length::Fill)
            .vertical_alignment(alignment::Vertical::Center)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .on_press(Message::Debug("Button".into()))
            
    );

    let cb = MenuTree::new(
        checkbox("Checkbox", app.check, Message::CheckChange)
    );

    let sld = MenuTree::new(row![
        "Slider",
        horizontal_space(Length::Units(8)),
        slider(0..=255, app.value, Message::ValueChange)
    ]);

    let tx = MenuTree::new(text("Text"));
    
    
    let root = MenuTree::with_children(
        debug_button("Widgets"),
        vec![
            debug_item("You can use any widget"),
            debug_item("as a menu item"),
            bt,
            cb,
            sld,
            tx,
            MenuTree::with_children(
                row![
                    toggler(Some("Or a sub menu item".to_string()), app.toggle, Message::ToggleChange),
                ].padding([0, 8]),
                vec![
                    debug_item("Item"),
                    debug_item("Item"),
                    debug_item("Item"),
                    debug_item("Item"),
                ]
            )
        ]
    );

    root
}

fn menu_3<'a>(app: &App) -> MenuTree<'a, Message, iced::Renderer>{
    
    let root = MenuTree::with_children(
        labeled_button("Click Me", Message::Debug("Click Me".into())),
        vec![
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
        ]
    );

    root
}
