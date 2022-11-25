#pragma once
#include <memory>

#include "rust/cxx.h"

struct Point;

class Line {
private:
    double a, b;
public:
    Line(double a, double b);
    bool contains_point(const Point &p) const;
};

std::unique_ptr<Line> new_line(double a, double b);
