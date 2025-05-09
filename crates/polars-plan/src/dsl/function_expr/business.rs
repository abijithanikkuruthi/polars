use std::fmt::{Display, Formatter};

use polars_core::prelude::*;
use polars_ops::prelude::Roll;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::dsl::SpecialEq;
use crate::map_as_slice;
use crate::prelude::ColumnsUdf;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub enum BusinessFunction {
    #[cfg(feature = "business")]
    BusinessDayCount {
        week_mask: [bool; 7],
        holidays: Vec<i32>,
    },
    #[cfg(feature = "business")]
    AddBusinessDay {
        week_mask: [bool; 7],
        holidays: Vec<i32>,
        roll: Roll,
    },
    #[cfg(feature = "business")]
    IsBusinessDay {
        week_mask: [bool; 7],
        holidays: Vec<i32>,
    },
}

impl Display for BusinessFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use BusinessFunction::*;
        let s = match self {
            #[cfg(feature = "business")]
            &BusinessDayCount { .. } => "business_day_count",
            #[cfg(feature = "business")]
            &AddBusinessDay { .. } => "add_business_days",
            #[cfg(feature = "business")]
            &IsBusinessDay { .. } => "is_business_day",
        };
        write!(f, "{s}")
    }
}
impl From<BusinessFunction> for SpecialEq<Arc<dyn ColumnsUdf>> {
    fn from(func: BusinessFunction) -> Self {
        use BusinessFunction::*;
        match func {
            #[cfg(feature = "business")]
            BusinessDayCount {
                week_mask,
                holidays,
            } => {
                map_as_slice!(business_day_count, week_mask, &holidays)
            },
            #[cfg(feature = "business")]
            AddBusinessDay {
                week_mask,
                holidays,
                roll,
            } => {
                map_as_slice!(add_business_days, week_mask, &holidays, roll)
            },
            #[cfg(feature = "business")]
            IsBusinessDay {
                week_mask,
                holidays,
            } => {
                map_as_slice!(is_business_day, week_mask, &holidays)
            },
        }
    }
}

#[cfg(feature = "business")]
pub(super) fn business_day_count(
    s: &[Column],
    week_mask: [bool; 7],
    holidays: &[i32],
) -> PolarsResult<Column> {
    let start = &s[0];
    let end = &s[1];
    polars_ops::prelude::business_day_count(
        start.as_materialized_series(),
        end.as_materialized_series(),
        week_mask,
        holidays,
    )
    .map(Column::from)
}

#[cfg(feature = "business")]
pub(super) fn add_business_days(
    s: &[Column],
    week_mask: [bool; 7],
    holidays: &[i32],
    roll: Roll,
) -> PolarsResult<Column> {
    let start = &s[0];
    let n = &s[1];
    polars_ops::prelude::add_business_days(
        start.as_materialized_series(),
        n.as_materialized_series(),
        week_mask,
        holidays,
        roll,
    )
    .map(Column::from)
}

#[cfg(feature = "business")]
pub(super) fn is_business_day(
    s: &[Column],
    week_mask: [bool; 7],
    holidays: &[i32],
) -> PolarsResult<Column> {
    let dates = &s[0];
    polars_ops::prelude::is_business_day(dates.as_materialized_series(), week_mask, holidays)
        .map(Column::from)
}
