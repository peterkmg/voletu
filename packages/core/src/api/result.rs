use super::{error::ApiError, response::ApiResponse};

/// Convenience alias for handler return types.
pub type ApiResult<T> = Result<ApiResponse<T>, ApiError>;
