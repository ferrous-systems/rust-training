/**
 * @brief Parses a null-terminated string into an integer
 * 
 * @param s a null-terminated string
 * @return the integer represented by the string, or 0 
 */
unsigned int cool_library_function(const char* s) {
    unsigned int result = 0;
    for(const char* p = s; *p; p++) {
        result *= 10;
        if ((*p < '0') || (*p > '9')) {  return 0; } 
        result += (*p - '0');
    }
    return result;
}