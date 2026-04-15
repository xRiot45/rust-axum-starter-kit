use serde::{Deserialize, Serialize};

/// Standard pagination query parameters.
#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_per_page")]
    pub per_page: u32,
}

fn default_page() -> u32 { 1 }
fn default_per_page() -> u32 { 20 }

impl PaginationQuery {
    pub fn offset(&self) -> i64 {
        ((self.page.saturating_sub(1)) * self.per_page) as i64
    }
    pub fn limit(&self) -> i64 {
        self.per_page as i64
    }
}

/// Paginated response wrapper.
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub data: Vec<T>,
    pub meta: PaginationMeta,
}

#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub page: u32,
    pub per_page: u32,
    pub total: i64,
    pub total_pages: u32,
}

impl<T: Serialize> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, total: i64, query: &PaginationQuery) -> Self {
        let total_pages = ((total as f64) / (query.per_page as f64)).ceil() as u32;
        Self {
            data,
            meta: PaginationMeta {
                page: query.page,
                per_page: query.per_page,
                total,
                total_pages,
            },
        }
    }
}
