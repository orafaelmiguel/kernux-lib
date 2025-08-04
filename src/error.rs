use core::ffi::c_int;

pub type KernelResult<T> = Result<T, KernelError>;

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum KernelError {
    EPERM = 1,
    ENOENT = 2,
    EIO = 5,
    EAGAIN = 11,
    ENOMEM = 12,
    EACCES = 13,
    EFAULT = 14,
    EBUSY = 16,
    EEXIST = 17,
    ENODEV = 19,
    EINVAL = 22,
    Unknown(c_int),
}

impl From<c_int> for KernelError {
    fn from(val: c_int) -> Self {
        let val = val.abs();
        match val {
            1 => KernelError::EPERM,
            2 => KernelError::ENOENT,
            5 => KernelError::EIO,
            11 => KernelError::EAGAIN,
            12 => KernelError::ENOMEM,
            13 => KernelError::EACCES,
            14 => KernelError::EFAULT,
            16 => KernelError::EBUSY,
            17 => KernelError::EEXIST,
            19 => KernelError::ENODEV,
            22 => KernelError::EINVAL,
            _ => KernelError::Unknown(val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{KernelError, KernelResult};

    #[test]
    fn test_error_from_c_int() {
        assert_eq!(KernelError::from(-1), KernelError::EPERM);
        assert_eq!(KernelError::from(-22), KernelError::EINVAL);
        assert_eq!(KernelError::from(-12), KernelError::ENOMEM);
    }

    #[test]
    fn test_unknown_error_code() {
        assert_eq!(KernelError::from(-999), KernelError::Unknown(999));
    }

    #[test]
    fn test_kernel_result_type() {
        let success: KernelResult<()> = Ok(());
        assert_eq!(success, Ok(()));

        let failure: KernelResult<()> = Err(KernelError::ENODEV);
        assert_eq!(failure, Err(KernelError::from(-19)));
    }
}