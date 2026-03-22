pub trait ResultExt<T, E> {
    fn take_err<EE>(self, errors: &mut Vec<EE>) -> Option<T>
    where
        EE: From<E>;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn take_err<EE>(self, errors: &mut Vec<EE>) -> Option<T>
    where
        EE: From<E>,
    {
        match self {
            Ok(v) => Some(v),
            Err(e) => {
                errors.push(EE::from(e));
                None
            }
        }
    }
}
