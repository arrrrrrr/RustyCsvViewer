use crate::BoxedResult;
use crate::ui::{AppState, Component, ComponentParams, MainWindow};

pub struct MenuContainer
{
    name: String,
    data: nwg::Menu,
    children: Vec<Box<dyn Component + 'static>>,
}

impl MenuContainer {
    pub fn new(name: &str, data: nwg::Menu) -> MenuContainer {
        MenuContainer {
            name: name.to_string(), data, children: Vec::new()
        }
    }
}

impl Component for MenuContainer {
    fn name(&self) -> &str {
        &self.name
    }

    fn handle(&self) -> &nwg::ControlHandle {
        &self.data.handle
    }

    fn children(&self) -> Option<&Vec<Box<dyn Component + 'static>>> {
        Some(&self.children)
    }

    fn add_child(&mut self, child: Box<dyn Component + 'static>) {
        self.children.push(child)
    }

    fn clear_children(&mut self) {
        self.children.clear()
    }
}

pub struct MenuItemContainer
{
    name: String,
    data: nwg::MenuItem,
    cmd: Box<dyn Fn(&MainWindow, &mut AppState, &nwg::Event, &nwg::EventData) -> BoxedResult<()> + 'static>,
}

impl MenuItemContainer
{
    pub fn new<F>(name: &str, data: nwg::MenuItem, cmd: F) -> MenuItemContainer
        where F: Fn(&MainWindow, &mut AppState, &nwg::Event, &nwg::EventData) -> BoxedResult<()> + 'static
    {
        MenuItemContainer {
            name: name.to_string(), data, cmd: Box::new(cmd),
        }
    }
}

impl Component for MenuItemContainer
{
    fn name(&self) -> &str {
        &self.name
    }

    fn handle(&self) -> &nwg::ControlHandle {
        &self.data.handle
    }

    fn run(&self, params: ComponentParams) -> BoxedResult<()> {
        let callee = &*params.callee.borrow();
        let mut state = params.state.lock().unwrap();
        (&self.cmd)(callee, &mut state, &params.event, &params.event_data)
    }

    fn add_child(&mut self, _child: Box<dyn Component + 'static>) {
        panic!("add_child not implemented for MenuItemContainer")
    }

    fn clear_children(&mut self) {
        panic!("clear_children not implemented for MenuItemContainer")
    }
}


pub struct MenuSepContainer {
    data: nwg::MenuSeparator,
}

impl MenuSepContainer {
    pub fn new(data: nwg::MenuSeparator) -> MenuSepContainer {
        MenuSepContainer {
            data
        }
    }
}

impl Component for MenuSepContainer {
    fn handle(&self) -> &nwg::ControlHandle {
        &self.data.handle
    }

    fn add_child(&mut self, _child: Box<dyn Component + 'static>) {
        panic!("add_child not implemented for MenuSepContainer")
    }

    fn clear_children(&mut self) {
        panic!("clear_children not implemented for MenuSepContainer")
    }
}

subclass_control!(MenuContainer, Menu, data);
subclass_control!(MenuItemContainer, MenuItem, data);
subclass_control!(MenuSepContainer, MenuSeparator, data);