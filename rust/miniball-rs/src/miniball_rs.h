#pragma once
#include "miniball-rs/src/lib.rs.h"

typedef std::array<double, 3> Point;

BoundingSphere compute_miniball_cpp(rust::Slice<const Point> points);
