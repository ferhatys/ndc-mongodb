use enum_iterator::{all, Sequence};

// TODO: How can we unify this with the Accumulator type in the mongodb module?
#[derive(Copy, Clone, Debug, PartialEq, Eq, Sequence)]
pub enum AggregationFunction {
    Avg,
    Count,
    Min,
    Max,
    Sum,
}

use AggregationFunction as A;

use crate::interface_types::MongoAgentError;

impl AggregationFunction {
    pub fn graphql_name(self) -> &'static str {
        match self {
            A::Avg => "avg",
            A::Count => "count",
            A::Min => "min",
            A::Max => "max",
            A::Sum => "sum",
        }
    }

    pub fn from_graphql_name(s: &str) -> Result<Self, MongoAgentError> {
        all::<AggregationFunction>()
            .find(|variant| variant.graphql_name() == s)
            .ok_or(MongoAgentError::UnknownAggregationFunction(s.to_owned()))
    }
}
