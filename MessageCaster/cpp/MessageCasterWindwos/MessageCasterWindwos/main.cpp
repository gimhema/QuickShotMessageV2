#include "Messages/ExampleMessage.hpp"
#include "MessageCaster/MessageCaster.h"
#include "MessageCaster/TestNode.h"

#include <iostream>

int main()
{
	std::cout << "QuickShot Message Caster " << std::endl;

	TestNode* testNode = new TestNode();
	testNode->Init("127.0.0.1", 8080);
	testNode->Run();

}
