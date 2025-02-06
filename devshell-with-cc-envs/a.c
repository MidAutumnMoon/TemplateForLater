#include <sodium.h>
#include <zmq.h>

int main( void ) {
    (void) sodium_init();
    (void) zmq_ctx_new();
    printf( "Zeromq %d.%d\n", ZMQ_VERSION_MAJOR, ZMQ_VERSION_MINOR );
    printf( "Libsodium %s\n", SODIUM_VERSION_STRING );
    return 0;
}
