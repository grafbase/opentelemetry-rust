use std::sync::Weak;

use crate::metrics::{
    aggregation::Aggregation,
    data::{ResourceMetrics, Temporality},
    instrument::InstrumentKind,
    pipeline::Pipeline,
    reader::{AggregationSelector, MetricReader, TemporalitySelector},
};
use futures_util::{future::BoxFuture, FutureExt};
use opentelemetry::metrics::Result;

#[derive(Debug)]
pub struct TestMetricReader {}

impl MetricReader for TestMetricReader {
    fn register_pipeline(&self, _pipeline: Weak<Pipeline>) {}

    fn collect(&self, _rm: &mut ResourceMetrics) -> Result<()> {
        Ok(())
    }

    fn force_flush(&self) -> Result<()> {
        Ok(())
    }

    fn shutdown(&self) -> BoxFuture<'_, Result<()>> {
        async move { self.force_flush() }.boxed()
    }
}

impl AggregationSelector for TestMetricReader {
    fn aggregation(&self, _kind: InstrumentKind) -> Aggregation {
        Aggregation::Drop
    }
}

impl TemporalitySelector for TestMetricReader {
    fn temporality(&self, _kind: InstrumentKind) -> Temporality {
        Temporality::Cumulative
    }
}
