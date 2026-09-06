# Fallible Rust date selectors

Date/timestamp single and range selectors return `Result<Builder>` instead of
silently substituting index zero when conversion fails. Check both endpoints;
preserve intentional early sub-daily timestamp clamping.

Call `endpoint.get_date(date)?.fetch()` instead of
`endpoint.get_date(date).fetch()`. The same change applies to date ranges and
timestamp selectors. Numeric builders and fetch methods are unchanged.

The generator owns the checked-in client implementation. Tests verify generated
parity and accepted/rejected bounds without making HTTP requests.
