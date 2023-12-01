#ifndef __LIBPYTHONVM_H
#define __LIBPYTHONVM_H

#include <graal_isolate_dynamic.h>


#if defined(__cplusplus)
extern "C" {
#endif

typedef int (*Py_IsInitialized_fn_t)(graal_isolatethread_t* thread);

#if defined(__cplusplus)
}
#endif
#endif
