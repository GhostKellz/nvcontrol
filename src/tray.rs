#[cfg(feature = "tray")]
use std::sync::mpsc;
#[cfg(feature = "tray")]
use tray_icon::{
    TrayIcon, TrayIconBuilder,
    menu::{Menu, MenuItem},
};

pub enum TrayEvent {
    ShowGui,
    QuickVibrance(i16),
    Exit,
}

#[cfg(feature = "tray")]
pub struct SystemTray {
    _tray_icon: TrayIcon,
    event_receiver: mpsc::Receiver<TrayEvent>,
}

#[cfg(feature = "tray")]
impl SystemTray {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (_sender, receiver) = mpsc::channel();

        let menu = Menu::new();
        let _show_item = MenuItem::new("Show nvcontrol", true, None);
        let _vibrance_menu = Menu::new();
        let _vibrance_low = MenuItem::new("Low Vibrance", true, None);
        let _vibrance_high = MenuItem::new("High Vibrance", true, None);
        let _exit_item = MenuItem::new("Exit", true, None);

        let tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip("nvcontrol")
            .build()?;

        Ok(Self {
            _tray_icon: tray_icon,
            event_receiver: receiver,
        })
    }

    pub fn try_recv(&self) -> Option<TrayEvent> {
        self.event_receiver.try_recv().ok()
    }
}

#[cfg(not(feature = "tray"))]
pub struct SystemTray;

#[cfg(not(feature = "tray"))]
impl SystemTray {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(SystemTray)
    }

    pub fn try_recv(&self) -> Option<TrayEvent> {
        None
    }
}
