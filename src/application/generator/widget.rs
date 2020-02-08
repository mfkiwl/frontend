use gtk::{
    self,
    BoxExt,
    OrientableExt,
};
use relm_attributes::widget;
use super::Signal;
use super::output::Widget as OutputWidget;
use super::output::Model as OutputModel;
use super::output::Signal::{
    Amplitude,
    DutyCycle,
    Frequency,
    Offset,
    Form,
    Start,
    Stop,
};

#[widget]
impl ::relm::Widget for Widget {
    fn model(generator: ::redpitaya_scpi::generator::Generator) -> ::redpitaya_scpi::generator::Generator {
        generator
    }

    fn update(&mut self, event: Signal) {
        match event {
            Signal::Redraw(ref context, ref model) => self.draw(context, model),
            _ => (),
        }
    }

    view! {
        gtk::Box {
            orientation: gtk::Orientation::Vertical,
            spacing: 10,
            #[name="out1"]
            OutputWidget(OutputModel {
                source: ::redpitaya_scpi::generator::Source::OUT1,
                generator: self.model.clone(),
            }) {
                Amplitude(amplitude) => Signal::Amplitude(::redpitaya_scpi::generator::Source::OUT1, amplitude),
                DutyCycle(duty_cycle) => Signal::DutyCycle(::redpitaya_scpi::generator::Source::OUT1, duty_cycle),
                Frequency(frequency) => Signal::Frequency(::redpitaya_scpi::generator::Source::OUT1, frequency),
                Offset(offset) => Signal::Offset(::redpitaya_scpi::generator::Source::OUT1, offset),
                Form(form) => Signal::Form(::redpitaya_scpi::generator::Source::OUT1, form),
                Start => Signal::Start(::redpitaya_scpi::generator::Source::OUT1),
                Stop => Signal::Stop(::redpitaya_scpi::generator::Source::OUT1),
            },
            #[name="out2"]
            OutputWidget(OutputModel {
                source: ::redpitaya_scpi::generator::Source::OUT2,
                generator: self.model.clone(),
            }) {
                Amplitude(amplitude) => Signal::Amplitude(::redpitaya_scpi::generator::Source::OUT2, amplitude),
                DutyCycle(duty_cycle) => Signal::DutyCycle(::redpitaya_scpi::generator::Source::OUT2, duty_cycle),
                Frequency(frequency) => Signal::Frequency(::redpitaya_scpi::generator::Source::OUT2, frequency),
                Offset(offset) => Signal::Offset(::redpitaya_scpi::generator::Source::OUT2, offset),
                Form(form) => Signal::Form(::redpitaya_scpi::generator::Source::OUT2, form),
                Start => Signal::Start(::redpitaya_scpi::generator::Source::OUT2),
                Stop => Signal::Stop(::redpitaya_scpi::generator::Source::OUT2),
            },
        },
    }
}

impl Widget {
    fn draw(&self, context: &::cairo::Context, model: &::application::Model) {
        context.save();
        self.out1.emit(super::output::Signal::Redraw(context.clone(), model.clone()));
        context.restore();
        context.save();
        self.out2.emit(super::output::Signal::Redraw(context.clone(), model.clone()));
        context.restore();
    }
}

impl Clone for Widget {
    fn clone(&self) -> Self {
        Self {
            gtkbox1: self.gtkbox1.clone(),
            model: self.model.clone(),
            out1: self.out1.clone(),
            out2: self.out2.clone(),
        }
    }
}
