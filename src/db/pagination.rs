use crate::services::error::ServiceError;

pub struct Pagination {
    pub limit: i64,
    pub offset: i64,
}

impl Pagination {
    pub fn new(page: i64, per_page: i64, max_per_page: i64) -> Result<Self, ServiceError> {
        if page < 1 {
            return Err(ServiceError::Validation {
                reason: "page must be >= 1".into(),
            });
        }
        if per_page < 1 || per_page > max_per_page {
            return Err(ServiceError::Validation {
                reason: format!("per_page must be between 1 and {}", max_per_page),
            });
        };
        Ok(Pagination {
            limit: per_page,
            offset: (page - 1) * per_page,
        })
    }
}
