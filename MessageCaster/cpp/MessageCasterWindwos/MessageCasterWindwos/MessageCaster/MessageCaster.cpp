#include "MessageCaster.h"
#include "../Messages/ExampleMessage.hpp"


MessageCaster::MessageCaster()
{

}


MessageCaster::~MessageCaster()
{

}


void MessageCaster::RecvPostProcess()
{

}

void MessageCaster::SendPreProcess()
{

}

void MessageCaster::HandleMessage(const std::vector<uint8_t>& buffer)
{
    BaseMessage base_message = BaseMessage::deserialize(buffer);

    switch (base_message.id) {
    case 0: {

        std::cout << "Invalid message id: 0\n";
        break;
    }
    case 1: {
        std::cout << "Case 1 " << std::endl;
        ExampleMessage example_message = ExampleMessage::deserialize(buffer);

        std::cout << "id : " << example_message.id << std::endl;
        std::cout << "val : " << example_message.val << std::endl;
        std::cout << "name : " << example_message.name << std::endl;
        
        for (size_t i = 0; i < example_message.nums.size(); ++i)
        {
            std::cout << "num[" << i << "] : " << example_message.nums[i] << std::endl;
        }

        break;
    }
    case 2: {
        std::cout << "Case 2 " << std::endl;
        break;
    }
    default: {
        std::cout << "Unknown message id: " << base_message.id << "\n";
        break;
    }
    }
}