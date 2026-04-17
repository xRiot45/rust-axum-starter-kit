use chrono::Utc;
use serde::{Deserialize, Serialize};

/// Default pagination values
fn default_page() -> u32 {
    1
}

fn default_limit() -> u32 {
    10
}

/// Pagination Query
#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    pub page: u32,

    #[serde(default = "default_limit")]
    pub limit: u32,

    pub sort_by: Option<String>,
    pub sort_order: Option<String>, // "asc" | "desc"
}

impl PaginationQuery {
    pub fn offset(&self) -> i64 {
        ((self.page.saturating_sub(1)) * self.limit) as i64
    }

    pub fn limit(&self) -> i64 {
        self.limit as i64
    }

    pub fn sort_order(&self) -> &str {
        match self.sort_order.as_deref() {
            Some("desc") | Some("DESC") => "DESC",
            _ => "ASC",
        }
    }
}

/// Pagination Meta
#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub page: u32,
    pub limit: u32,
    pub total_items: i64,
    pub total_pages: u32,
    pub has_next_page: bool,
    pub has_previous_page: bool,
}

impl PaginationMeta {
    pub fn new(total_items: i64, query: &PaginationQuery) -> Self {
        let total_pages = ((total_items as f64) / (query.limit as f64)).ceil() as u32;
        Self {
            page: query.page,
            limit: query.limit,
            total_items,
            total_pages,
            has_next_page: query.page < total_pages,
            has_previous_page: query.page > 1,
        }
    }
}

/// Global Meta
#[derive(Debug, Serialize)]
pub struct GlobalMeta {
    pub timestamp: String,
    pub request_id: String,
    pub path: String,
    pub method: String,
    pub pagination: Option<PaginationMeta>,
}

impl GlobalMeta {
    pub fn new() -> Self {
        Self {
            timestamp: Utc::now().to_rfc3339(),
            request_id: "req_xxx".to_string(), // TODO: inject in middleware
            path: "".to_string(),              // TODO: inject in middleware
            method: "".to_string(),            // TODO: inject in middleware
            pagination: None,
        }
    }
}

/// Standard API Response
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub status_code: u16,
    pub message: String,
    pub data: T,
    pub meta: GlobalMeta,
}

/// Non-Paginated Responses
impl<T> ApiResponse<T> {
    /// Default OK (200)
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            status_code: 200,
            message: "Success".to_string(),
            data,
            meta: GlobalMeta::new(),
        }
    }

    /// Created Data (201)
    pub fn created(data: T) -> Self {
        Self {
            success: true,
            status_code: 201,
            message: "Created".to_string(),
            data,
            meta: GlobalMeta::new(),
        }
    }

    /// Custom message
    pub fn with_message(message: impl Into<String>, data: T) -> Self {
        Self {
            success: true,
            status_code: 200,
            message: message.into(),
            data,
            meta: GlobalMeta::new(),
        }
    }

    /// Custom status code & message
    pub fn custom(status_code: u16, message: impl Into<String>, data: T) -> Self {
        Self {
            success: true,
            status_code: status_code,
            message: message.into(),
            data,
            meta: GlobalMeta::new(),
        }
    }
}

/// No Content Response
impl ApiResponse<()> {
    pub fn no_content(message: impl Into<String>) -> Self {
        Self {
            success: true,
            status_code: 200,
            message: message.into(),
            data: (),
            meta: GlobalMeta::new(),
        }
    }
}

/// Paginated Responses
impl<T> ApiResponse<Vec<T>> {
    pub fn with_pagination(
        data: Vec<T>,
        total_items: i64,
        query: &PaginationQuery,
        message: impl Into<String>,
    ) -> Self {
        let pagination = PaginationMeta::new(total_items, query);

        Self {
            success: true,
            status_code: 200,
            message: message.into(),
            data,
            meta: GlobalMeta {
                pagination: Some(pagination),
                ..GlobalMeta::new()
            },
        }
    }
}
