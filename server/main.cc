#include <seastar/core/app-template.hh>
#include <seastar/core/distributed.hh>
#include <seastar/net/api.hh>

#include "server.hh"
#include "rust/src/lib.rs.h"
#include "rust/cxx.h"

using namespace seastar;

void do_something_using_ffi() {
    Line line(2, -3);
    rust::Box<Point> point = new_point(1, -1);
    std::cout << "Is point (1, -1) on line y = 2x - 3? "
              << (line.contains_point(*point) ? "Yes.\n" : "No.\n");
    
    std::cout << "Is point (5, -2) a cross point of lines y = -3x + 13 and y = 2x - 12? "
              << (is_cross_point(-3, 13, 2, -12, 5, -2) ? "Yes.\n" : "No.\n");
}

int main(int ac, char** av) {
    do_something_using_ffi();

    /*app_template app;
    return app.run_deprecated(ac, av, [&] {
        uint16_t port = 5555;
        auto server = new distributed<tcp_server>;

        (void)server->start().then([server = std::move(server), port] () mutable {
            engine().at_exit([server] {
                return server->stop();
            });
            (void)server->invoke_on_all(&tcp_server::listen, ipv4_addr{port});
        }).then([port] {
            std::cout << "Seastar TCP server listening on port " << port << " ...\n";
        });
    });*/
}
