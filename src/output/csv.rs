use super::Tabular;
use crate::error::Result;

pub fn render<T: Tabular>(items: &[T]) -> Result<String> {
    let mut wtr = csv::Writer::from_writer(Vec::new());

    wtr.write_record(T::headers())
        .map_err(|e| crate::error::XeroCliError::Io(std::io::Error::other(e)))?;

    for item in items {
        wtr.write_record(item.row())
            .map_err(|e| crate::error::XeroCliError::Io(std::io::Error::other(e)))?;
    }

    let bytes = wtr
        .into_inner()
        .map_err(|e| crate::error::XeroCliError::Io(std::io::Error::other(e)))?;

    String::from_utf8(bytes).map_err(|e| crate::error::XeroCliError::Io(std::io::Error::other(e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestItem {
        a: String,
        b: String,
    }

    impl Tabular for TestItem {
        fn headers() -> Vec<String> {
            vec!["A".to_string(), "B".to_string()]
        }
        fn row(&self) -> Vec<String> {
            vec![self.a.clone(), self.b.clone()]
        }
    }

    #[test]
    fn render_csv() {
        let items = vec![TestItem {
            a: "hello".to_string(),
            b: "world".to_string(),
        }];
        let output = render(&items).unwrap();
        assert!(output.contains("A,B"));
        assert!(output.contains("hello,world"));
    }
}
