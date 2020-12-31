/// Menu wrappers and implementations
///

use nwg;
use std::collections::HashMap;
use super::app::App;

/// Menu templates help build complex menu structures
pub enum TMenu {
    Menu(String, bool), //text, disabled
    MenuItem(String, bool, bool), //text, disabled, check
    MenuSeparator(String), //text
    ContextMenu(String, bool), //text, disabled
    None
}

impl TMenu {
    pub fn get_text(&self) -> &str {
        match self {
            Self::Menu(text, _) => &text,
            Self::MenuItem(text, _, _) => &text,
            Self::MenuSeparator(text) => &text,
            Self::ContextMenu(text, _) => &text,
            _ => "",
        }
    }
}

// Function pointer to App::fn(&self)
type FnEventCb = fn(&App, &nwg::Event, &nwg::EventData);

/// Instances of the different menu types for a menu container
pub enum IMenu {
    Menu(nwg::Menu),
    MenuItem(nwg::MenuItem,Option<FnEventCb>),
    MenuSeparator(nwg::MenuSeparator),
}

impl From<nwg::Menu> for IMenu {
    fn from(menu: nwg::Menu) -> Self {
        IMenu::Menu(menu)
    }
}

impl From<&nwg::Menu> for IMenu {
    fn from(menu: &nwg::Menu) -> Self {
        IMenu::Menu(nwg::Menu { handle: menu.handle.clone() } )
    }
}

impl From<nwg::MenuItem> for IMenu {
    fn from(menu: nwg::MenuItem) -> Self {
        IMenu::MenuItem(menu, None)
    }
}

impl From<&nwg::MenuItem> for IMenu {
    fn from(menu: &nwg::MenuItem) -> Self {
        IMenu::MenuItem(nwg::MenuItem { handle: menu.handle.clone() }, None)
    }
}

impl From<nwg::MenuSeparator> for IMenu {
    fn from(menu: nwg::MenuSeparator) -> Self {
        IMenu::MenuSeparator(menu)
    }
}

impl From<&nwg::MenuSeparator> for IMenu {
    fn from(menu: &nwg::MenuSeparator) -> Self {
        IMenu::MenuSeparator(nwg::MenuSeparator { handle: menu.handle.clone() } )
    }
}

impl IMenu {
    pub fn handle(&self) -> &nwg::ControlHandle {
        match self {
            Self::Menu(m) => &m.handle,
            Self::MenuItem( m, _) => &m.handle,
            Self::MenuSeparator( m) => &m.handle
        }
    }

    pub fn menu(&self) -> Option<&nwg::Menu> {
        match self {
            Self::Menu( m) => Some(&m),
            Self::MenuItem(_m, _f) => None,
            Self::MenuSeparator(_m) => None,
        }
    }

    pub fn menu_mut(&mut self) -> Option<&mut nwg::Menu> {
        match self {
            Self::Menu( m) => Some(m),
            Self::MenuItem(_m, _f) => None,
            Self::MenuSeparator(_m) => None,
        }
    }

    pub fn menu_item(&self) -> Option<&nwg::MenuItem> {
        match self {
            Self::Menu( _m) => None,
            Self::MenuItem(m, _f) => Some(&m),
            Self::MenuSeparator(_m) => None,
        }
    }

    pub fn menu_item_mut(&mut self) -> Option<&mut nwg::MenuItem> {
        match self {
            Self::Menu( _m) => None,
            Self::MenuItem(m, _f) => Some(m),
            Self::MenuSeparator(_m) => None,
        }
    }

    pub fn menu_separator(&self) -> Option<&nwg::MenuSeparator> {
        match self {
            Self::MenuSeparator(m) => Some(&m),
            _ => None,
        }
    }

    pub fn menu_separator_mut(&mut self) -> Option<&mut nwg::MenuSeparator> {
        match self {
            Self::Menu(_m) => None,
            Self::MenuItem(_m, _f) => None,
            Self::MenuSeparator(m) => Some(m),
        }
    }

    pub fn is_enabled(&self) -> bool {
        match self {
            Self::Menu(m) => m.enabled(),
            Self::MenuItem(m, _f) => m.enabled(),
            Self::MenuSeparator(_m) => false
        }
    }

    pub fn enable(&self) {
        match self {
            Self::Menu(m) => m.set_enabled(true),
            Self::MenuItem(m, _f) => m.set_enabled(true),
            Self::MenuSeparator(_m) => {},
        }
    }

    pub fn disable(&self) {
        match self {
            Self::Menu(m) => m.set_enabled(false),
            Self::MenuItem(m, _f) => m.set_enabled(false),
            Self::MenuSeparator(_) => {},
        }
    }

    pub fn command(&self) -> Option<FnEventCb> {
        match self {
            Self::Menu(_) => None,
            Self::MenuItem(_, f) => *f,
            Self::MenuSeparator(_) => None,
        }
    }

    pub fn set_command(&mut self) {
        match self {
            Self::Menu(_) => {}
            Self::MenuItem(_, _) => {}
            Self::MenuSeparator(_) => {}
        }
    }
}

/// Container for a top level and submenu
pub struct CMenu {
    pub parent: IMenu,
    pub command: Option<FnEventCb>,
    pub children: HashMap<String, IMenu>,
}

impl CMenu {
    pub fn new(inst: IMenu,
               sub_name: &mut Vec<&str>,
               sub_inst: Vec<IMenu>) -> Self
    {
        let mut sub_menu = HashMap::<String, IMenu>::new();

        // Take names and values and build a hash map
        for (&key, value) in sub_name.iter().zip(sub_inst.into_iter()) {
            sub_menu.insert(key.to_owned(), value);
        }

        CMenu {
            parent: inst,
            children: sub_menu,
            command: None,
        }
    }

    pub fn get_menu(&self) -> &IMenu {
        &self.parent
    }

    pub fn get_menu_mut(&mut self) -> &mut IMenu {
        &mut self.parent
    }

    pub fn get_submenu(&self, name: &str) -> Option<&IMenu> {
        self.children.get(name)
    }

    pub fn get_submenu_mut(&mut self, name: &str) -> Option<&mut IMenu> {
        self.children.get_mut(name)
    }
}

/// Helper to bulk build a complete menu
pub struct BulkMenuBuilder {
    top: TMenu,
    items: Vec<TMenu>,
}

impl BulkMenuBuilder {
    pub fn new() -> Self {
        BulkMenuBuilder {
            top: TMenu::None, items: vec![]
        }
    }

    pub fn add_menu(mut self, menu: TMenu) -> BulkMenuBuilder {
        self.top = menu;
        self
    }

    pub fn add_submenu_item(mut self, menu: TMenu) -> BulkMenuBuilder {
        self.items.push(menu);
        self
    }

    fn build_menu<C: Into<nwg::ControlHandle>>(
        &self, template: &TMenu,
        inst: &mut IMenu, parent: C) -> Result<(),nwg::NwgError>
    {
        match template {
            TMenu::Menu(text, disabled) => {
                nwg::Menu::builder()
                    .text(&text)
                    .disabled(*disabled)
                    .popup(false)
                    .parent(parent)
                    .build(inst.menu_mut().unwrap())?;
            },
            _ => {}
        }

        Ok(())
    }

    fn build_menu_item<C: Into<nwg::ControlHandle>>(
        &self, template: &TMenu,
        inst: &mut IMenu, parent: C) -> Result<(),nwg::NwgError>
    {
         match template {
            TMenu::MenuItem(text, disabled, check) => {
                nwg::MenuItem::builder()
                    .text(&text)
                    .disabled(*disabled)
                    .check(*check)
                    .parent(parent)
                    .build(inst.menu_item_mut().unwrap())?;
            },
            _ => {}
        }

        Ok(())
    }

    fn build_menu_separator<C: Into<nwg::ControlHandle>>(
        &self, template: &TMenu,
        inst: &mut IMenu, parent: C) -> Result<(),nwg::NwgError>
    {
        nwg::MenuSeparator::builder()
            .parent(parent)
            .build(inst.menu_separator_mut().unwrap())?;

        Ok(())
    }

    fn build_context_menu<C: Into<nwg::ControlHandle>>(
        &self, template: &TMenu,
        inst: &mut IMenu, parent: C) -> Result<(),nwg::NwgError>
    {
        match template {
            TMenu::ContextMenu(text, disabled) => {
                nwg::Menu::builder()
                    .text(&text)
                    .disabled(*disabled)
                    .popup(true)
                    .parent(parent)
                    .build(inst.menu_mut().unwrap())?;
            },
            _ => {}
        }

        Ok(())
    }

    fn build_one<C: Into<nwg::ControlHandle>>(
        &self, template: &TMenu,
        instance: &mut IMenu, root: C) -> Result<(),nwg::NwgError>
    {
        match template {
            TMenu::Menu(..) => {
                self.build_menu(template, instance, root)?;
            },
            TMenu::MenuItem(..) => {
                self.build_menu_item(template, instance, root)?;
            },
            TMenu::MenuSeparator(..) => {
                self.build_menu_separator(template, instance, root)?;
            },
            TMenu::ContextMenu(..) => {
                self.build_menu(template, instance, root)?;
            },
            _ => {}
        }

        Ok(())
    }

    pub fn build<C: Into<nwg::ControlHandle>>(
        &self, container: &mut CMenu, root: C) -> Result<(),nwg::NwgError>
    {
        self.build_one(&self.top, &mut container.parent, root)?;

        for v in &self.items {
            let i= container.children.get_mut(v.get_text())
                .expect("failed to get mut key");

            let i_ = &container.parent;
            match i_ {
                IMenu::Menu(_) =>
                    self.build_one(&v, i, i_.menu().unwrap())?,
                IMenu::MenuItem(_,_) =>
                    self.build_one(&v, i, i_.menu_item().unwrap())?,
                IMenu::MenuSeparator(_) =>
                    self.build_one(&v, i, i_.menu_separator().unwrap())?,
            }
        }

        Ok(())
    }

}