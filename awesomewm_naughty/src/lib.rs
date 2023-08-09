use std::collections::HashMap;

pub struct Notification {
    notification: HashMap<String, String>    
}

impl Notification {
    pub fn new() -> Notification {
        Notification { notification: HashMap::new() }    
    }

    fn add_to_notification(mut self, key: &str, value: String) -> Notification {
        self.notification.insert(key.to_string(), value);
        self
    }

    pub fn ignore_suspend(self, ignore_suspend: bool) -> Notification {
        self.add_to_notification("ignore_suspend", ignore_suspend.to_string())
    }

    pub fn border_color(self, border_color: String) -> Notification {
        self.add_to_notification("border_color", process_string(border_color))
    }

    pub fn border_width(self, border_width: usize) -> Notification {
        self.add_to_notification("border_width", border_width.to_string())
    }

    pub fn bg(self, bg: String) -> Notification {
        self.add_to_notification("bg", process_string(bg))
    }

    pub fn fg(self, fg: String) -> Notification {
        self.add_to_notification("fg", process_string(fg))
    }

    pub fn font(self, font: String) -> Notification {
        self.add_to_notification("font", process_string(font))
    }

    pub fn width(self, width: usize) -> Notification {
        self.add_to_notification("width", width.to_string())
    }

    pub fn height(self, height: usize) -> Notification {
        self.add_to_notification("height", height.to_string())
    }

    pub fn ontop(self, ontop: bool) -> Notification {
        self.add_to_notification("ontop", ontop.to_string())
    }

    pub fn text(self, text: String) -> Notification {
        self.add_to_notification("text", process_string(text))
    }

    pub fn title(self, title: String) -> Notification {
        self.add_to_notification("title", process_string(title))
    }

    pub fn timeout(self, timeout: usize) -> Notification {
        self.add_to_notification("timeout", timeout.to_string())
    }

    pub fn hover_timeout(self, hover_timeout: usize) -> Notification {
        self.add_to_notification("hover_timeout", hover_timeout.to_string())
    }

    pub fn screen(self, screen: usize) -> Notification {
        self.add_to_notification("screen", screen.to_string())
    }

    pub fn position(self, position: String) -> Notification {
        self.add_to_notification("position", process_string(position))
    }

    pub fn build(self) -> String {
        let props: String = self.notification.iter().map(|(key, val)| format!("{} = {},", key, val)).collect();
        let props = format!("{{ {} }}", props);

        format!("naughty.notify({})", props)
    }
}

fn process_string(val: String) -> String {
    format!("\"{}\"", val)
}

pub fn send(notification_string: String) -> Result<(), std::io::Error> {
    use std::process::Command;

    Command::new("/usr/bin/awesome-client")
        .arg(notification_string)
        .output()?;

    Ok(())
}

