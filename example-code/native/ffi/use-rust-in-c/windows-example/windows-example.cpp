// windows-example.cpp : This file contains the 'main' function. Program execution begins and ends there.
//

#include <iostream>

extern "C" {
	/// Designed to have the exact same shape as the Rust version
	typedef struct magic_adder_t {
		uint32_t amount;
	} magic_adder_t;

	/// Wraps MagicAdder::new
	extern magic_adder_t magicadder_new(uint32_t amount);

	/// Wraps MagicAdder::process_value
	extern uint32_t magicadder_process_value(magic_adder_t* self, uint32_t value);

	/// Heap allocate a new magic_adder_t
	magic_adder_t* magicadder_allocate(uint32_t amount);

	/// Destroy a magic_adder_t that was created with `magicadder_allocate`
	void magicadder_free(magic_adder_t* p_adder);

}

int main()
{
	magic_adder_t ma = magicadder_new(5);
	printf("5 + 6 = %u\n", magicadder_process_value(&ma, 6));

	magic_adder_t* p_ma = magicadder_allocate(10);
	printf("10 + 6 = %u\n", magicadder_process_value(p_ma, 6));
	magicadder_free(p_ma);
	magicadder_free(NULL); // won't explode
	return 0;
}

