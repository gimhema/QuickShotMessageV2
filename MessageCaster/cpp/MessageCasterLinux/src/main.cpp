#include <iostream>
#include "MessageCaster/MessageCaster.h"
#include "MessageCaster/TestNode.h"
#include "Messages/ExampleMessage.hpp"

int main()
{
    MessageCaster* _caster = new MessageCaster();
    std::cout << "Message Caster Linux" << std::endl;
    return 0;
}