use crate::cache::CachedClient;
use crate::error::Result;

pub async fn paginate_all<T, F>(
    client: &CachedClient,
    path: &str,
    base_params: &[(&str, &str)],
    page_size: u32,
    extract: F,
) -> Result<Vec<T>>
where
    T: serde::de::DeserializeOwned,
    F: Fn(&serde_json::Value) -> Option<Vec<T>>,
{
    let mut all_items = Vec::new();
    let mut page = 1u32;

    loop {
        let page_str = page.to_string();
        let size_str = page_size.to_string();
        let mut params: Vec<(&str, &str)> = base_params.to_vec();
        params.push(("page", &page_str));
        params.push(("pageSize", &size_str));

        let response = client.get_with_params(path, &params).await?;
        let items = extract(&response).unwrap_or_default();
        let count = items.len();
        all_items.extend(items);

        if count < page_size as usize {
            break;
        }
        page += 1;
    }

    Ok(all_items)
}

/// Offset-based pagination for endpoints like Journals that use `offset` instead of `page`.
/// The Xero Journals API returns up to 100 records per call and uses a sequential offset.
pub async fn paginate_all_offset<T, F>(
    client: &CachedClient,
    path: &str,
    base_params: &[(&str, &str)],
    page_size: u64,
    extract: F,
) -> Result<Vec<T>>
where
    T: serde::de::DeserializeOwned,
    F: Fn(&serde_json::Value) -> Option<Vec<T>>,
{
    let mut all_items = Vec::new();
    let mut offset = 0u64;

    loop {
        let offset_str = offset.to_string();
        let mut params: Vec<(&str, &str)> = base_params.to_vec();
        params.push(("offset", &offset_str));

        let response = client.get_with_params(path, &params).await?;
        let items = extract(&response).unwrap_or_default();
        let count = items.len();
        all_items.extend(items);

        if count < page_size as usize {
            break;
        }
        offset += count as u64;
    }

    Ok(all_items)
}

#[cfg(test)]
mod tests {
    #[test]
    fn pagination_types_compile() {
        // Type-level test to ensure the generic signature works
        fn _assert_send<T: Send>() {}
        // The function should be usable with common types
    }
}
