use crate::error::{Result, XeroCliError};
use serde::Serialize;

pub fn render<T: Serialize>(items: &[T]) -> Result<String> {
    serde_yaml::to_string(items).map_err(|e| XeroCliError::Io(std::io::Error::other(e)))
}

pub fn render_single<T: Serialize>(item: &T) -> Result<String> {
    serde_yaml::to_string(item).map_err(|e| XeroCliError::Io(std::io::Error::other(e)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[derive(Serialize)]
    struct Item {
        name: String,
        value: i32,
    }

    #[test]
    fn render_yaml_list() {
        let items = vec![Item {
            name: "test".to_string(),
            value: 42,
        }];
        let output = render(&items).unwrap();
        assert!(output.contains("name: test"));
        assert!(output.contains("value: 42"));
    }

    #[test]
    fn render_yaml_single() {
        let item = Item {
            name: "single".to_string(),
            value: 1,
        };
        let output = render_single(&item).unwrap();
        assert!(output.contains("name: single"));
    }
}
