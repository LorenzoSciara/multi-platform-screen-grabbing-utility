<<<<<<< HEAD
use iced::{Element, Length, alignment, theme};
use iced::widget::{button, row, text, column, container};
=======
use std::fmt::format;
use iced::{Element, Length, alignment, theme, Event};
use iced::widget::{button, row, text, column, container, Row};
>>>>>>> thread_lorenzo
use crate::{Message, Choice};
use iced::widget::{horizontal_space, scrollable, toggler, vertical_space, Radio, Container};

fn select_container(select_value: String, select_type: String) -> Container<'static, Message> {
    let text_box = Container::new(text(select_value).size(20).horizontal_alignment(alignment::Horizontal::Center).vertical_alignment(alignment::Vertical::Center))
        .style(style::text_container)
        .height(30)
        .width(160)
        .padding([0,0,0,20]);
    let select_button = button(if select_type == "shortcut" { "Set Now!" } else { "Chose Path" } )
        .on_press( if select_type == "shortcut" { Message::Shortcut("Ctrl + s".to_string()) } else { Message::Path("C:/user/Desktop".to_string()) });
    let select_button_container = Container::new(select_button).padding([0,0,0,20]);
    let setting_input = row![text_box, select_button_container];
    let container = Container::new(setting_input);
    return container;

}
fn timer_container(timer_value: i32)-> Container<'static, Message> {
    let increment_button = button(text("+").width(Length::Fixed(25.0)).height(Length::Fixed(25.0)).size(25).horizontal_alignment(alignment::Horizontal::Center).vertical_alignment(alignment::Vertical::Center))
<<<<<<< HEAD
        .on_press( if timer_value < 10 { Message::TimerChange(timer_value+1) } else { Message::TimerChange(timer_value) })
        ;
    let timer_text = Container::new(text(timer_value).size(20)).padding([5,20,0,20]);
    let decrement_button = button(text("-").width(Length::Fixed(25.0)).height(Length::Fixed(25.0)).size(25).horizontal_alignment(alignment::Horizontal::Center).vertical_alignment(alignment::Vertical::Center))
        .on_press(if timer_value > 0 { Message::TimerChange(timer_value-1) } else { Message::TimerChange(timer_value)})
        ;
=======
        .on_press( if timer_value < 10 { Message::TimerChange(timer_value+1) } else { Message::TimerChange(timer_value) });
    let timer_text = Container::new(text(timer_value.clone()).size(20)).padding([5,20,0,20]);
    let decrement_button = button(text("-").width(Length::Fixed(25.0)).height(Length::Fixed(25.0)).size(25).horizontal_alignment(alignment::Horizontal::Center).vertical_alignment(alignment::Vertical::Center))
        .on_press(if timer_value > 0 { Message::TimerChange(timer_value.clone()-1) } else { Message::TimerChange(timer_value.clone())});
>>>>>>> thread_lorenzo
    let setting_input = row![decrement_button, timer_text, increment_button];
    let container = Container::new(setting_input);
    return container;
}

<<<<<<< HEAD

fn radio_container(radio_value: Choice, radio_type: String) -> Container<'static, Message> {
=======
fn radio_container_format(radio_value: Choice) -> Container<'static, Message> {
    let selected_choice = Some(radio_value);
    let a = Radio::new(".jpg", Choice::A, selected_choice, Message::RadioSelectedFormat);
    let container_a = Container::new(a).padding([0,10]);
    let b = Radio::new(".png", Choice::B, selected_choice, Message::RadioSelectedFormat);
    let container_b = Container::new(b).padding([0,10]);
    let c = Radio::new(".gif", Choice::C, selected_choice, Message::RadioSelectedFormat);
    let container_c = Container::new(c).padding([0,10]);
    let setting_input = row![container_a, container_b, container_c];
    let container = Container::new(setting_input);
    return container;
}

fn radio_container_monitor(radio_value: Choice, total_monitor_number: usize) -> Container<'static, Message> {
    let tmn = if total_monitor_number>6 {6}else{total_monitor_number};
    let choices = vec![("1", Choice::A), ("2", Choice::B), ("3", Choice::C), ("4", Choice::D), ("5", Choice::E), ("All", Choice::F), ];
    let b : Vec<Event> = Vec::new();
    let events = Row::with_children(
        b
            .iter()
            .map(|choice| text(format!("{choice:?}")).size(40))
            .map(Element::from),
    );


    return container;
}



fn radio_container_monitor1(radio_value: Choice, radio_type: String, total_monitor_number: usize) -> Container<'static, Message> {
>>>>>>> thread_lorenzo
    let selected_choice = Some(radio_value);
    let a = Radio::new(
        if radio_type=="monitor" {"Display 1"} else {".jpg"},
        Choice::A,
        selected_choice,
        if radio_type=="monitor" {Message::RadioSelectedMonitor} else {Message::RadioSelectedFormat});
    let A = Container::new(a).padding([0,10]);
    let b = Radio::new(
        if radio_type=="monitor" {"Display 2"} else {".png"},
        Choice::B,
        selected_choice,
        if radio_type=="monitor" {Message::RadioSelectedMonitor} else {Message::RadioSelectedFormat});
    let B = Container::new(b).padding([0,10]);
    let c = Radio::new(
        if radio_type=="monitor" {"All Display"} else {".gif"},
        Choice::C,
        selected_choice,
        if radio_type=="monitor" {Message::RadioSelectedMonitor} else {Message::RadioSelectedFormat});
    let C = Container::new(c).padding([0,10]);
    let setting_input = row![A, B, C];
    let container = Container::new(setting_input);
    return container;
}

<<<<<<< HEAD

=======
>>>>>>> thread_lorenzo
fn toggler_container(toggler_value: bool, toggler_type: String) -> Container<'static, Message> {
    let setting_input = toggler(
        String::from(""),
        toggler_value,
        move |b|{if toggler_type=="autosave" { Message::TogglerToggledAutosave(b) } else { Message::TogglerToggledClipboard(b) }}
    )
        .width(Length::Shrink)
        .spacing(10);
    let container = Container::new(setting_input);
    return container;

}
fn settings_box(settings_text: String, settings_container: Container<'static, Message>) -> Container<'static, Message> {
    let settingtext = text(settings_text)
        .size(18)
        .vertical_alignment(alignment::Vertical::Center);
    let text_container = Container::new(settingtext).padding([0,0,0,10]);
    let space = horizontal_space(Length::Fill);
    let setting = row![text_container, space, settings_container];
    let container = Container::new(setting)
        .style(style::settings_container)
        .height(80)
        .width(Length::Fill)
        .padding(10)
        .align_y(alignment::Vertical::Center);
    return container;
}
<<<<<<< HEAD
pub fn settings(toggler_value_autosave: bool, toggler_value_clipboard: bool, radio_value_monitor: Choice, radio_value_format: Choice, timer_value:i32, shortcut_value:String, path_value:String) -> Element<'static, Message> {
=======
pub fn settings(toggler_value_autosave: bool, toggler_value_clipboard: bool, radio_value_monitor: Choice, radio_value_format: Choice, timer_value:i32, shortcut_value:String, path_value:String, total_monitor_number: usize) -> Element<'static, Message> {
>>>>>>> thread_lorenzo
    let undobutton = button(text("← Home").width(Length::Fill).size(20))
        .on_press(Message::HomeButton)
        .style(theme::Button::Destructive)
        .width(Length::Fixed(100.0))
        .height(Length::Fixed(50.0))
        .padding(10);

<<<<<<< HEAD
    //let space = horizontal_space(Length::Fill);

    // let savebutton = button(row![delete_icon(), text("Save").width(Length::Fill).size(20) ]
    //     .spacing(10)
    //     .align_items(Alignment::Center))
    //     .on_press(Message::PagesState(PagesState::Home))
    //     .style(theme::Button::Positive)
    //     .width(Length::Fixed(100.0))
    //     .height(Length::Fixed(50.0))
    //     .padding(10);

    let controls = row![undobutton];
    let spacev = vertical_space(Length::Fixed(20.0));

    let container1 = settings_box("Save the screenshot to the default location".to_string(), toggler_container(toggler_value_autosave, "autosave".to_string()));
    let container2 = settings_box("Copy the screenshot into the clipdoard".to_string(), toggler_container(toggler_value_clipboard, "clipboard".to_string()));
    let container3 = settings_box("Select the monitor in which to screenshot".to_string(), radio_container(radio_value_monitor, "monitor".to_string()));
    let container4 = settings_box("Set a shortcut to make the screenshots".to_string(), select_container(shortcut_value, "shortcut".to_string()));
    let container5 = settings_box("Select the screenshot format".to_string(), radio_container(radio_value_format, "format".to_string()));
    let container6 = settings_box("Set a timer before the screenshot".to_string(), timer_container(timer_value));
    let container7 = settings_box("Change the path where you save the screenshot".to_string(), select_container(path_value, "path".to_string()));






=======
    let controls = row![undobutton];
    let spacev = vertical_space(Length::Fixed(20.0));

    let container1 = settings_box("Save the screenshot automatically".to_string(), toggler_container(toggler_value_autosave, "autosave".to_string()));
    let container2 = settings_box("Copy the screenshot into the clipdoard automatically".to_string(), toggler_container(toggler_value_clipboard, "clipboard".to_string()));
    let container3 = settings_box("Select the monitor in which to screenshot".to_string(), radio_container_monitor(radio_value_monitor,  total_monitor_number));
    let container4 = settings_box("Set a shortcut to make the screenshots".to_string(), select_container(shortcut_value, "shortcut".to_string()));
    let container5 = settings_box("Select the screenshot format".to_string(), radio_container_format(radio_value_format));
    let container6 = settings_box("Set a timer before the screenshot".to_string(), timer_container(timer_value));
    let container7 = settings_box("Change the path where you save the screenshot".to_string(), select_container(path_value, "path".to_string()));

>>>>>>> thread_lorenzo
    let content: Element<_> = column![ controls, spacev, container1, container2, container3, container4, container5, container6, container7 ]
        .spacing(20)
        .padding(20)
        .into();

    let scrollable = scrollable(
        container(content)
            .width(Length::Fill)
            .center_x(),
    );

    return container(scrollable).height(Length::Fill).center_y().into();

}

mod style {
    use iced::widget::container;
    use iced::{BorderRadius, Color, Theme};

    pub fn settings_container(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();

        container::Appearance {
            text_color: Some(palette.background.strong.text),
            background: Some(Color::from_rgb(87.0 / 255.0, 115.0 / 255.0, 240.0 / 255.0).into()),
            border_radius: BorderRadius::from(15.0),
            border_width: 3.0,
            border_color: Color::BLACK,
            ..Default::default()
        }
    }

    pub fn text_container(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();

        container::Appearance {
            text_color: Some(palette.background.strong.text),
            background: Some(Color::from_rgb(87.0 / 255.0, 115.0 / 255.0, 240.0 / 255.0).into()),
            border_radius: BorderRadius::from(5.0),
            border_width: 2.0,
            border_color: Color::BLACK,
            ..Default::default()
        }
    }
}