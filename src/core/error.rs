///
///
///

#[derive(Debug, Clone)]
pub enum DdsRetcode {
    Ok = 0,                     // Successs
    Error = -1,                 // Non specific error
    Unsupported = -2,           // Feature unsupported
    BadParameter = -3,          // Bad parameter value
    PreconditionNotMet = -4,    // Precondition for operation not met
    OutOfResources = -5,        // When an operation fails because of a lack of resources
    NotEnabled = -6,            // When a configurable feature is not enabled
    ImmutablePolicy = -7,       // When an attempt is made to modify an immutable policy
    InconsistentPolicy = -8,    // When a policy is used with inconsistent values
    AlreadyDeleted = -9,        // When an attempt is made to delete something more than once
    Timeout = -10,              // When a timeout has occurred
    NoData = -11,               // When expected data is not provided
    IllegalOperation = -12,     // When a function is called when it should not be
    NotAllowedBySecurity = -13, // When credentials are not enough to use the function
}

#[derive(Debug, Clone)]
pub struct DdsError {}

impl std::error::Error for DdsRetcode {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl std::fmt::Display for DdsRetcode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            DdsRetcode::Ok => "Successs",
            DdsRetcode::Error => "Non specific error",
            DdsRetcode::Unsupported => "Feature unsupported",
            DdsRetcode::BadParameter => "Bad parameter value",
            DdsRetcode::PreconditionNotMet => "Precondition for operation not met",
            DdsRetcode::OutOfResources => "Operation failed because of a lack of resources",
            DdsRetcode::NotEnabled => "A configurable feature is not enabled",
            DdsRetcode::ImmutablePolicy => "An attempt was made to modify an immutable policy",
            DdsRetcode::InconsistentPolicy => "A policy with inconsistent values was used",
            DdsRetcode::AlreadyDeleted => "An attempt was made to delete something more than once",
            DdsRetcode::Timeout => "A timeout has occurred",
            DdsRetcode::NoData => "Expected data is not provided",
            DdsRetcode::IllegalOperation => "A function was called when it should not be",
            DdsRetcode::NotAllowedBySecurity => {
                "Insufficient credentials supplied to use the function"
            }
        };

        write!(f, "{}", s)
    }
}
