
#include <stdio.h>
#include <stdarg.h>

/// when I link with I found some symbol not found
/// errmsg_internal errstart_cold errfinish So i impl those
/// functions myself, Is this a goodthing to do??
__attribute__((weak)) int errmsg_internal(const char *format, ...)
{
    va_list args;
    va_start(args, format);
    int result = vprintf(format, args);
    va_end(args);
    return result;
}
__attribute__((weak)) int errstart_cold(int elevel, const char *domain)
{
    return 1;
}
__attribute__((weak)) void errfinish(const char *filename, int lineno, const char *funcname)
{
}
void foo()
{
    errmsg_internal("hello from foo :-)\n");
}