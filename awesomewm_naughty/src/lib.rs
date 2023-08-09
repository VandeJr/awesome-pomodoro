use std::collections::HashMap;

pub struct Notification {
    notification: HashMap<String, String>    
}

impl Notification {
    pub fn new() -> Notification {
        Notification { notification: HashMap::new() }    
    }

    pub fn ignore_suspend(mut self, ignore_suspend: String) -> Notification {
        self.notification.insert("ignore_suspend".to_string(), ignore_suspend);
        self
    }

    pub fn border_color(mut self, border_color: String) -> Notification {
        self.notification.insert("border_color".to_string(), process_string(border_color));
        self
    }

    pub fn border_width(mut self, border_width: String) -> Notification {
        self.notification.insert("border_width".to_string(), process_string(border_width));
        self
    }

    pub fn bg(mut self, bg: String) -> Notification {
        self.notification.insert("bg".to_string(), process_string(bg));
        self
    }

    pub fn fg(mut self, fg: String) -> Notification {
        self.notification.insert("fg".to_string(), process_string(fg));
        self
    }

    pub fn font(mut self, font: String) -> Notification {
        self.notification.insert("font".to_string(), process_string(font));
        self
    }

    pub fn width(mut self, width: String) -> Notification {
        self.notification.insert("width".to_string(), width);
        self
    }

    pub fn height(mut self, height: String) -> Notification {
        self.notification.insert("height".to_string(), height);
        self
    }

    pub fn ontop(mut self, ontop: String) -> Notification {
        self.notification.insert("ontop".to_string(), ontop);
        self
    }

    pub fn text(mut self, text: String) -> Notification {
        self.notification.insert("text".to_string(), process_string(text));
        self
    }

    pub fn title(mut self, title: String) -> Notification {
        self.notification.insert("title".to_string(), process_string(title));
        self
    }

    pub fn timeout(mut self, timeout: String) -> Notification {
        self.notification.insert("timeout".to_string(), timeout);
        self
    }

    pub fn hover_timeout(mut self, hover_timeout: String) -> Notification {
        self.notification.insert("hover_timeout".to_string(), hover_timeout);
        self
    }

    pub fn screen(mut self, screen: String) -> Notification {
        self.notification.insert("screen".to_string(), screen);
        self
    }

    pub fn position(mut self, position: String) -> Notification {
        self.notification.insert("position".to_string(), process_string(position));
        self
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_build() {
        
    }
}
