use diesel_derive_enum::DbEnum;

#[derive(Debug, PartialEq, DbEnum, Clone)]
#[DieselType = "AccountStatusEnum"]
#[DbValueStyle = "SCREAMING_SNAKE_CASE"]
pub enum AccountStatus {
    CREATED,
    CONFIRMED,
    DISABLED,
}
