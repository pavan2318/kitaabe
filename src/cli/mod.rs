use crate::error::KitaabeError;

pub fn get_query() -> Result<String, KitaabeError> {
    let mut args = std::env::args().skip(1);

    let query = args
        .next()
        .ok_or(KitaabeError::MissingQuery)?;

    Ok(query)
}
