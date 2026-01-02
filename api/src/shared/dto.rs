use std::fmt;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

//1베이스
#[serde_as]
#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    #[serde_as(as = "Option<DisplayFromStr>")]
    page: Option<usize>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    size: Option<usize>,
}

//0베이스
pub struct Pagination {
    pub page: ZeroBaseIndex,
    pub size: u64,
}

impl From<PaginationQuery> for Pagination {
    fn from(p: PaginationQuery) -> Self {
        let page = p.page.unwrap_or(1).max(1);
        let size = p.size.unwrap_or(20).clamp(1, 50);

        Self {
            page: ZeroBaseIndex((page - 1) as u64),
            size: size as u64,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    Desc,
    Asc,
}

impl Default for SortDirection {
    fn default() -> Self {
        SortDirection::Desc
    }
}

impl fmt::Display for SortDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            SortDirection::Desc => "DESC",
            SortDirection::Asc => "ASC",
        };

        write!(f, "{}", s)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OneBaseIndex(u64);

#[derive(Debug)]
pub struct ZeroBaseIndex(pub u64);

#[derive(Debug)]
pub struct Pageable {
    pub page: ZeroBaseIndex,
    pub size: u64,
    pub total_items: u64,
    pub total_pages: u64,
}

#[derive(Debug)]
pub struct Page<T> {
    pub content: Vec<T>,
    pub pageable: Pageable,
}

impl<T> Page<T> {
    pub fn new(content: Vec<T>, pageable: Pageable) -> Self{
        Self { content, pageable }
    }

    pub fn map<U, F>(self, mut f: F) -> Page<U> 
        where F: FnMut(T) -> U
    {
        Page { 
            content: self.content.into_iter().map(f).collect(),
            pageable: self.pageable 
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageableResponse{
    pub page: OneBaseIndex,
    pub size: u64,
    pub total_items: u64,
    pub total_pages: u64,
}

impl From<Pageable> for PageableResponse {
    fn from(value: Pageable) -> Self {
        Self { page: OneBaseIndex(value.page.0 + 1), size: value.size, total_items: value.total_items, total_pages: value.total_pages }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageResponse<T> {
    pub content: Vec<T>,
    pub pageable: PageableResponse,
}

impl<T> From<Page<T>> for PageResponse<T> {
    fn from(page: Page<T>) -> Self {
        Self {
            content: page.content,
            pageable: PageableResponse::from(page.pageable),
        }
    }
}