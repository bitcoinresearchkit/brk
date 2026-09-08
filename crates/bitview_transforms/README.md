# bitview_transforms

Stateless, element-wise Bitcoin value conversions and arithmetic. Transforms
implement vecdb unary or binary transform interfaces and use only the numeric
bounds their operations need. Storage ownership and range computations do not
belong here.

Families include currency conversion, ratios, OHLC, and arithmetic. Generic
adapters (`Ident`, `MapOption`, `ReverseOperands`) belong to vecdb. Bitcoin block
target constants belong to `brk_types`.
