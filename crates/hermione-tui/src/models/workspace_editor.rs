use crate::{
    controllers::editor::FormModel,
    entities::Workspace,
    models::editor::{Message, Model as EditorModel, Property, Signal as EditorSignal},
    Result,
};

pub struct Model {
    inner: EditorModel,
}

pub struct WorkspaceForm {
    pub name: String,
}

pub enum Signal {
    Exit,
    Submit(WorkspaceForm),
}

impl FormModel for Model {
    type Signal = Signal;

    fn update(&mut self, message: Message) -> Result<Option<Signal>> {
        match self.inner.update(message)? {
            Some(signal) => match signal {
                EditorSignal::Exit => Ok(Some(Signal::Exit)),
                EditorSignal::Submit(properties) => Ok(Some(self.workspace_form(properties))),
            },
            None => Ok(None),
        }
    }

    fn view(&self, frame: &mut ratatui::Frame) {
        self.inner.view(frame);
    }
}

impl Model {
    pub fn from_workspace(workspace: Option<&Workspace>) -> Result<Self> {
        let inner = EditorModel::new(properties(workspace))?;

        Ok(Self { inner })
    }

    fn workspace_form(&self, properties: Vec<Property>) -> Signal {
        let form = WorkspaceForm {
            name: properties[0].value.clone(),
        };

        Signal::Submit(form)
    }
}

fn properties(workspace: Option<&Workspace>) -> Vec<Property> {
    match workspace {
        None => vec![Property {
            name: "Name".into(),
            value: "".into(),
        }],
        Some(workspace) => vec![Property {
            name: "Name".into(),
            value: workspace.name.clone(),
        }],
    }
}
