#ifndef __LIBPYTHONVM_H
#define __LIBPYTHONVM_H

#include "graal_isolate.h"


#if defined(__cplusplus)
extern "C" {
#endif

int Py_IsInitialized(graal_isolatethread_t* thread);

#if defined(__cplusplus)
}
#endif
#endif
