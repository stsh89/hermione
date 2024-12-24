use super::State;

pub struct DrawOperation<'a, T> {
    pub renderer: &'a mut T,
}

pub trait Render {
    fn render(&mut self, state: &State) -> anyhow::Result<()>;
}

impl<T> DrawOperation<'_, T>
where
    T: Render,
{
    pub fn execute(&mut self, state: &State) -> anyhow::Result<()> {
        self.renderer.render(state)
    }
}
