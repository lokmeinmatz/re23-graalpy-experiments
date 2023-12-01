mkdir -p _c_headers
cp -r /Users/mkind/Dev/Uni/RE23/graal/sdk/mxbuild/darwin-aarch64/libpythonvm.dylib.image/*.h ./_c_headers
# rewrite header import
sed -i '' 's|#include <graal_isolate.h>|#include "graal_isolate.h"|' _c_headers/libpythonvm.h
echo copied  and updated files, calling bindgen...
bindgen -o ./exec/src/bindings.rs ./_c_headers/libpythonvm.h