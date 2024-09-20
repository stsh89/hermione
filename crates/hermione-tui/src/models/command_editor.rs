use crate::{
    controllers::editor::FormModel,
    entities::Command,
    models::editor::{Message, Model as EditorModel, Property, Signal as EditorSignal},
    Result,
};

pub struct Model {
    inner: EditorModel,
}

pub struct CommandForm {
    pub name: String,
    pub program: String,
}

pub enum Signal {
    Exit,
    Submit(CommandForm),
}

impl FormModel for Model {
    type Signal = Signal;

    fn update(&mut self, message: Message) -> Result<Option<Signal>> {
        match self.inner.update(message)? {
            Some(signal) => match signal {
                EditorSignal::Exit => Ok(Some(Signal::Exit)),
                EditorSignal::Submit(properties) => Ok(Some(self.command_form(properties))),
            },
            None => Ok(None),
        }
    }

    fn view(&self, frame: &mut ratatui::Frame) {
        self.inner.view(frame);
    }
}

impl Model {
    pub fn from_command(command: Option<&Command>) -> Result<Self> {
        let inner = EditorModel::new(properties(command))?;

        Ok(Self { inner })
    }

    fn command_form(&self, properties: Vec<Property>) -> Signal {
        let form = CommandForm {
            name: properties[0].value.clone(),
            program: properties[1].value.clone(),
        };

        Signal::Submit(form)
    }
}

fn properties(command: Option<&Command>) -> Vec<Property> {
    match command {
        None => vec![
            Property {
                name: "Name".into(),
                value: "".into(),
            },
            Property {
                name: "Program".into(),
                value: "".into(),
            },
        ],
        Some(command) => vec![
            Property {
                name: "Name".into(),
                value: command.name.clone(),
            },
            Property {
                name: "Program".into(),
                value: command.program.clone(),
            },
        ],
    }
}
