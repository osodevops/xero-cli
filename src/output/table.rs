use super::Tabular;
use comfy_table::{presets::UTF8_FULL, Table};

pub fn render<T: Tabular>(items: &[T]) -> String {
    if items.is_empty() {
        return "No results found.".to_string();
    }

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(T::headers());

    for item in items {
        table.add_row(item.row());
    }

    table.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestItem {
        name: String,
        value: String,
    }

    impl Tabular for TestItem {
        fn headers() -> Vec<String> {
            vec!["Name".to_string(), "Value".to_string()]
        }

        fn row(&self) -> Vec<String> {
            vec![self.name.clone(), self.value.clone()]
        }
    }

    #[test]
    fn render_empty() {
        let items: Vec<TestItem> = vec![];
        let output = render(&items);
        assert_eq!(output, "No results found.");
    }

    #[test]
    fn render_items() {
        let items = vec![
            TestItem {
                name: "Alice".to_string(),
                value: "100".to_string(),
            },
            TestItem {
                name: "Bob".to_string(),
                value: "200".to_string(),
            },
        ];
        let output = render(&items);
        assert!(output.contains("Alice"));
        assert!(output.contains("Bob"));
        assert!(output.contains("Name"));
    }
}
