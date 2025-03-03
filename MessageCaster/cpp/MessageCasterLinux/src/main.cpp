#include <iostream>
#include "MessageCaster/MessageCaster.h"
#include "MessageCaster/TestNode.h"
#include "Messages/ExampleMessage.hpp"

int main()
{
    TestNode* tNode = new TestNode();
    std::cout << "Message Caster Linux" << std::endl;

    tNode->Init("127.0.0.1", 8080);
    
    tNode->Run();


    return 0;
}