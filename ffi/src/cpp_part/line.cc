#include <cmath>

#include "line.h"
#include "ffi/src/lib.rs.h"

#include "seastar_ffi.h"

const double EPSILON = 1e-5;

Line::Line(double a, double b) : a(a), b(b) {}

bool Line::contains_point(const Point &p) const {
    do_something_using_seastar(); // Yep, this function makes no sense.
    return fabs(p.y() - (a * p.x() + b)) < EPSILON;
}

std::unique_ptr<Line> new_line(double a, double b) {
    return std::make_unique<Line>(a, b);
}
