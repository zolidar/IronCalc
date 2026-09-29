mod beta;
mod binom;
mod chisq;
mod correl;
mod count_and_average;
mod covariance;
mod devsq;
mod exponential;
mod fisher;
mod forecast;
mod frequency;
mod gamma;
mod gauss;
mod geomean;
mod hypegeom;
mod if_ifs;
mod linest;
mod log_normal;
mod mode_functions;
mod normal;
mod pearson;
mod percentile;
mod permut;
mod phi;
mod poisson;
mod prob;
mod quartile;
mod rank_eq_avg;
mod standard_dev;
mod standardize;
mod t_dist;
mod trimmean;
mod variance;
mod weibull;
mod z_test;

/// Sum of squared deviations from the mean, computed in two passes.
///
/// The one-pass `sumsq - sum * sum / n` form cancels catastrophically when the
/// values are close together and can go negative. The mean is taken relative to
/// the first value so that identical inputs give a mean equal to that value and
/// a result of exactly 0.
pub(crate) fn sum_of_squared_deviations(values: &[f64]) -> f64 {
    let Some(&shift) = values.first() else {
        return 0.0;
    };
    let n = values.len() as f64;
    let mean = shift + values.iter().map(|v| v - shift).sum::<f64>() / n;
    values.iter().map(|v| (v - mean) * (v - mean)).sum()
}
