use provider::item::ProviderItem;

#[derive(Debug, Clone, Serialize)]
pub struct ProviderFolder {
    pub label: String,
    pub folders: Vec<String>,
    pub items: Vec<ProviderItem>
}

impl ProviderFolder {
    pub fn new(label: String, folders: Vec<String>, items: Vec<ProviderItem>) -> ProviderFolder {
        ProviderFolder {
            label,
            folders,
            items
        }
    }

    pub fn empty(label: String) -> ProviderFolder {
        ProviderFolder {
            label,
            folders: vec![],
            items: vec![]
        }
    }
}
