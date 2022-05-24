pub enum BaseCompResult<Content, SuccessCode, FailureCode> {
    // success
    Content(Content),
    // success
    Id(u128),
    // success but only with a code
    Success(SuccessCode),
    Failed(FailureCode),
}
