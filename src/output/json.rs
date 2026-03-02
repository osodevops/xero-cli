use serde::Serialize;

pub fn render<T: Serialize>(items: &[T], compact: bool) -> String {
    if compact {
        serde_json::to_string(items).unwrap_or_else(|_| "[]".to_string())
    } else {
        serde_json::to_string_pretty(items).unwrap_or_else(|_| "[]".to_string())
    }
}

pub fn render_single<T: Serialize>(item: &T, compact: bool) -> String {
    if compact {
        serde_json::to_string(item).unwrap_or_else(|_| "{}".to_string())
    } else {
        serde_json::to_string_pretty(item).unwrap_or_else(|_| "{}".to_string())
    }
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
    fn render_pretty() {
        let items = vec![Item {
            name: "test".to_string(),
            value: 42,
        }];
        let output = render(&items, false);
        assert!(output.contains('\n'));
        assert!(output.contains("test"));
    }

    #[test]
    fn render_compact() {
        let items = vec![Item {
            name: "test".to_string(),
            value: 42,
        }];
        let output = render(&items, true);
        assert!(!output.contains('\n'));
    }

    #[test]
    fn render_single_item() {
        let item = Item {
            name: "single".to_string(),
            value: 1,
        };
        let output = render_single(&item, false);
        assert!(output.contains("single"));
    }
}
