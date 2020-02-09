pub mod channel;
pub mod edge;
pub mod mode;

pub use self::channel::Channel;
pub use self::edge::Edge;
pub use self::mode::Mode;

use crate::color::Colorable;
use crate::widget::radio::Signal::*;
use gtk::prelude::*;

type ChannelWidget = crate::widget::RadioGroup<Channel>;
type EdgeWidget = crate::widget::RadioGroup<Edge>;
type ModeWidget = crate::widget::RadioGroup<Mode>;

#[derive(relm_derive::Msg, Clone)]
pub enum Signal {
    Auto,
    Normal,
    Single,
    Mode(Mode),
    Channel(Channel),
    Source(redpitaya_scpi::trigger::Source),
    Edge(Edge),
    InternalTick,
    Redraw(Box<cairo::Context>, Box<crate::application::Model>),
}

#[derive(Clone)]
pub struct Model {
    stream: relm::EventStream<Signal>,
    trigger: redpitaya_scpi::trigger::Trigger,
    channel: Option<Channel>,
    edge: Option<Edge>,
    mode: Mode,
}

#[relm_derive::widget(Clone)]
impl relm::Widget for Widget {
    fn model(relm: &relm::Relm<Self>, trigger: redpitaya_scpi::trigger::Trigger) -> Model {
        Self::Model {
            stream: relm.stream().clone(),
            trigger,
            mode: Mode::Normal,
            edge: None,
            channel: None,
        }
    }

    fn subscriptions(&mut self, relm: &relm::Relm<Self>) {
        relm::interval(relm.stream(), 1_000, || Signal::InternalTick);
    }

    fn update(&mut self, event: Signal) {
        match event {
            Signal::InternalTick => {
                match self.model.mode {
                    Mode::Auto => self.model.stream.emit(Signal::Auto),
                    Mode::Normal => self.model.stream.emit(Signal::Normal),
                    Mode::Single => (),
                };
            }
            Signal::Mode(mode) => {
                self.model.mode = mode;

                match mode {
                    Mode::Auto => self.single_button.set_visible(false),
                    Mode::Normal => self.single_button.set_visible(false),
                    Mode::Single => self.single_button.set_visible(true),
                };
            }
            Signal::Channel(channel) => {
                self.model.channel = Some(channel);
                if let Some(source) = self.get_source() {
                    self.model.stream.emit(Signal::Source(source));
                    self.model.trigger.enable(source);
                }
            }
            Signal::Edge(edge) => {
                self.model.edge = Some(edge);
                if let Some(source) = self.get_source() {
                    self.model.stream.emit(Signal::Source(source));
                    self.model.trigger.enable(source);
                }
            }
            Signal::Redraw(ref context, ref model) => self.draw(context, model),
            _ => (),
        }
    }

    view! {
        #[name="page"]
        gtk::Box {
            orientation: gtk::Orientation::Vertical,
            spacing: 10,

            ChannelWidget(crate::widget::radio::Model {
                title: "Source".to_string(),
                options: vec![Channel::CH1, Channel::CH2, Channel::EXT],
                current: Some(Channel::CH1),
            }) {
                Change(channel) => Signal::Channel(channel),
            },
            EdgeWidget(crate::widget::radio::Model {
                title: "Edge".to_string(),
                options: vec![Edge::Positive, Edge::Negative],
                current: Some(Edge::Positive),
            }) {
                Change(channel) => Signal::Edge(channel),
            },
            ModeWidget(crate::widget::radio::Model {
                title: "Mode".to_string(),
                options: vec![Mode::Auto, Mode::Normal, Mode::Single],
                current: Some(self.model.mode),
            }) {
                Change(channel) => Signal::Mode(channel),
            },
            #[name="single_button"]
            gtk::Button {
                child: {
                    pack_type: gtk::PackType::Start,
                    expand: false,
                    fill: false,
                    padding: 0,
                },
                label: "Single",

                clicked(_) => Signal::Single,
            }
        },
    }
}

impl Widget {
    fn get_source(&self) -> Option<redpitaya_scpi::trigger::Source> {
        if self.model.channel == Some(Channel::CH1) && self.model.edge == Some(Edge::Positive) {
            Some(redpitaya_scpi::trigger::Source::CH1_PE)
        } else if self.model.channel == Some(Channel::CH1)
            && self.model.edge == Some(Edge::Negative)
        {
            Some(redpitaya_scpi::trigger::Source::CH1_NE)
        } else if self.model.channel == Some(Channel::CH2)
            && self.model.edge == Some(Edge::Positive)
        {
            Some(redpitaya_scpi::trigger::Source::CH2_PE)
        } else if self.model.channel == Some(Channel::CH2)
            && self.model.edge == Some(Edge::Negative)
        {
            Some(redpitaya_scpi::trigger::Source::CH2_NE)
        } else if self.model.channel == Some(Channel::EXT)
            && self.model.edge == Some(Edge::Positive)
        {
            Some(redpitaya_scpi::trigger::Source::EXT_PE)
        } else if self.model.channel == Some(Channel::EXT)
            && self.model.edge == Some(Edge::Negative)
        {
            Some(redpitaya_scpi::trigger::Source::EXT_NE)
        } else {
            None
        }
    }

    fn draw(&self, context: &cairo::Context, model: &crate::application::Model) {
        if self.model.mode == Mode::Normal || self.model.mode == Mode::Single {
            let width = model.scales.get_width();
            let height = model.scales.get_height();
            let delay = model.offset("DELAY");
            let trigger = model.offset("TRIG");

            context.set_color(crate::color::TRIGGER);

            context.set_line_width(width / 1000.0);
            context.move_to(delay, model.scales.v.0);
            context.line_to(delay, model.scales.v.1);
            context.stroke();

            context.set_line_width(height / 1000.0);
            context.move_to(model.scales.h.0, trigger);
            context.line_to(model.scales.h.1, trigger);
            context.stroke();
        }
    }
}
