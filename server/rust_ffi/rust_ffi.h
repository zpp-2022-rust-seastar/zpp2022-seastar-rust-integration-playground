extern "C" void store_from_rust(const char *key, const char *value);

extern "C" char * load_from_rust(const char *key);

extern "C" void free_from_rust(char *ptr);
