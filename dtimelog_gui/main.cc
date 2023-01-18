#include <iostream>

struct Greeter {
	std::string greeting;
};

extern "C" void Greeter::greet(std::string);

int main (void) {
	auto greeter = Greeter("Hello");
	greeter.greet("David");
}


