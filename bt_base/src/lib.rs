pub mod types;

#[cfg(test)]
mod tests {
    use chrono::{Local, NaiveDate};

    use super::types::{GameInfo, Frame, Game};

    #[test]
    fn info_create_part() {
        let def_part = GameInfo::build_part();
        let cust_part = GameInfo::build_date(NaiveDate::from_ymd_opt(2023, 2, 5).unwrap());

        assert_eq!(def_part, GameInfo::Partial(Local::now().date_naive()));
    }
}
