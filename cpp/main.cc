extern "C" void hello_from_rust(int x);

int main(void) {
    hello_from_rust(21);
}
