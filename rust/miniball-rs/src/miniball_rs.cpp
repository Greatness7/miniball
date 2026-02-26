#include "miniball_rs.h"
#include "Seb.h"

namespace {
    using std::size_t;

    struct PointView {
        const Point* point;

        double operator[](size_t index) const {
            return (*point)[index];
        }
    };

    struct PointAccessor {
        const Point* data;
        size_t count;

        size_t size() const {
            return count;
        }

        PointView operator[](size_t index) const {
            return { &data[index] };
        }
    };
}

BoundingSphere compute_miniball_cpp(rust::Slice<const Point> points) {
    using namespace Seb;

    if (points.empty()) {
        return {};
    }

    auto accessor = PointAccessor { points.data(), points.size() };
    auto miniball = Smallest_enclosing_ball<double, PointView, PointAccessor>(3, accessor);

    auto center = miniball.center_begin();
    auto radius = miniball.radius();

    return { { center[0], center[1], center[2] }, radius };
}
