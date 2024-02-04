/**
 * A horrendous C++ footgun I discovered in the SO question.
 * "When does the conversion happen when passing arguments to thread function?"
 * https://stackoverflow.com/q/73725046/11556864
 * 
 * Below innocent looking code can cause an UNDEFINED BEHAVIOR.
 */

#include <string>
#include <thread>

void foo(int i, std::string const &s) {
	/* stuffs */
}

void bar(int arg) {
	char buffer[1024];
	sprintf(buffer, "%i", arg);

	std::thread t(foo, 3, buffer);
	t.detach();
}

// 1. `foo()` takes `string` for the `s` parameter
// 2. In `bar()`, a new thread `t` is created with `foo()` and `buffer`
// 3. `buffer` is `char*`, which means it needs to be converted to `string`
// 4. ***But when does that conversion takes place?***
//
// Turns out the conversion happens AFTER creating a new thread, in the new thread.
// Rather than converting `buffer` to a `string` and passing that to a new thread,
// C++ creates an expression that converts arguments to proper types, and runs `foo()`.
// And that expression is executed in the new thread.
//
// This means that the conversion can potentially happen after `bar()` returns,
// when the 1024 bytes is no longer owned by the `buffer` and trying to access
// the dangling `char*` can cause an undefined behavior.
//
// Let's have a moment to appreciate the ownership and lifetime system of Rust.
