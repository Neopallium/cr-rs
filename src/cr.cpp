#define CR_HOST
#include "../vendor/cr.h"

extern "C" void wrap_cr_set_temporary(cr_plugin &ctx, const char *path) {
	std::string str_path = path;
	cr_set_temporary_path(ctx, str_path);
}

/*! TODO: Add host-side callbacks.
 *  \todo Add host-side callbacks.
 */
extern "C" int simple_host_main(const char *fullpath, void *userdata) {
	// load plugin
	cr_plugin ctx;
	ctx.userdata = userdata;
	cr_plugin_load(ctx, fullpath);

	// update loop
	while(!cr_plugin_update(ctx)) {
	}

	// cleanup
	cr_plugin_close(ctx);

	return 0;
}
