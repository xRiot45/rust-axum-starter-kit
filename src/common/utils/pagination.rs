use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

fn default_page() -> u32 {
    1
}
fn default_limit() -> u32 {
    10
}

/// Standard pagination query parameters.
#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_limit")]
    pub limit: u32,
}

impl PaginationQuery {
    pub fn offset(&self) -> i64 {
        ((self.page.saturating_sub(1)) * self.limit) as i64
    }
    pub fn limit(&self) -> i64 {
        self.limit as i64
    }
}

/// Paginated response wrapper.
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub success: bool,
    pub status_code: u16,
    pub timestamp: DateTime<Utc>,
    pub message: String,
    pub data: Vec<T>,
    pub meta: PaginationMeta,
}

#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub page: u32,
    pub limit: u32,
    pub total_items: i64,
    pub total_pages: u32,
    pub has_next_page: bool,
    pub has_previous_page: bool,
}

impl<T: Serialize> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, total_items: i64, query: &PaginationQuery, message: &str) -> Self {
        let total_pages = ((total_items as f64) / (query.limit as f64)).ceil() as u32;

        Self {
            success: true,
            status_code: 200,
            timestamp: Utc::now(),
            message: message.to_string(),
            data,
            meta: PaginationMeta {
                page: query.page,
                limit: query.limit,
                total_items,
                total_pages,
                has_next_page: query.page < total_pages,
                has_previous_page: query.page > 1,
            },
        }
    }
}
