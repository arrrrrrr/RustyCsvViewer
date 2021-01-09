use crate::ui::{Component, App, AppState};
use crate::ui::menu::{MenuContainer, MenuItemContainer, MenuSepContainer};
use crate::utils::menu_resource_to_lc;
use crate::BoxedResult;

type NwgResult<T> = Result<T, nwg::NwgError>;

pub enum MenuBuildType
{
    /// Menu(Name, Popup, Children)
    /// This can be a recursive definition
    Menu(String, bool, Vec<MenuBuildType>),
    /// MenuItem(Name, Disabled, Lambda)
    MenuItem(String, bool,
             Box<dyn Fn(&App, &mut AppState, &nwg::Event, &nwg::EventData) -> BoxedResult<()> + 'static>),
    /// MenuSeparator
    MenuSeparator
}

pub struct MenuBuilder
{
    root: Option<MenuBuildType>,
}

type NwgCompResult = NwgResult<Box<dyn Component>>;

impl MenuBuilder
{
    pub fn builder(root: MenuBuildType) -> MenuBuilder {
        MenuBuilder {
            root: Some(root)
        }
    }

    pub fn build(&mut self, parent: &nwg::ControlHandle) -> NwgCompResult  {
        // magic to move the value from self
        let mut mbt = None;
        std::mem::swap(&mut mbt, &mut self.root);
        // Call the interna; build function
        self.internal_build(mbt.unwrap(), parent.clone())
    }

    fn internal_build(& self, root: MenuBuildType, parent_handle: nwg::ControlHandle) -> NwgCompResult {
        use crate::ui::menu::MenuBuildType as BT;
        let mut phandle = parent_handle;

        // Recur through the tree of menus until we find the base case
        // then construct the containers backwards
        match root {
            BT::Menu(name, popup, children) => {
                let mut mc = self.internal_build_menu(phandle, &name, popup)?;
                // Update the parent handle to our handle
                phandle = mc.handle().clone();

                for child in children.into_iter() {
                    let comp = self.internal_build(child, phandle.clone())?;
                    mc.add_child(comp);
                }

                Ok(Box::new(mc))
            },
            BT::MenuItem(name, disabled, lambda) => {
                let mi: MenuItemContainer = self.internal_build_menu_item(
                    phandle, &name, disabled, lambda)?;
                Ok(Box::new(mi))
            },
            BT::MenuSeparator => {
                let ms = self.internal_build_menu_sep(phandle)?;
                Ok(Box::new(ms))
            }
        }
    }

    fn internal_build_menu_item<F>(&self,
                    parent: nwg::ControlHandle,
                    name: &str,
                    disabled: bool,
                    lambda: F)
        -> NwgResult<MenuItemContainer>
    where F: Fn(&App, &mut AppState, &nwg::Event, &nwg::EventData) -> BoxedResult<()> + 'static
    {
        let mut mi = nwg::MenuItem::default();

        nwg::MenuItem::builder()
            .text(name)
            .disabled(disabled)
            .parent(&parent)
            .build(&mut mi)?;

        Ok(MenuItemContainer::new(&menu_resource_to_lc(&name),mi,lambda))
    }

    fn internal_build_menu_sep(&self, parent: nwg::ControlHandle) -> NwgResult<MenuSepContainer> {
        let mut ms = nwg::MenuSeparator::default();

        nwg::MenuSeparator::builder()
            .parent(&parent)
            .build(&mut ms)?;

        Ok(MenuSepContainer::new(ms))
    }

    fn internal_build_menu(&self, parent: nwg::ControlHandle, name: &str, popup: bool)
        -> NwgResult<MenuContainer>
    {
        let mut m = nwg::Menu::default();

        nwg::Menu::builder()
            .text(name)
            .popup(popup)
            .parent(&parent)
            .build(&mut m)?;

        Ok(MenuContainer::new(&menu_resource_to_lc(&name), m))
    }
}