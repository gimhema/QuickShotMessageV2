#include "MessageCaster.h"


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
        // id가 0이면 Invalid 출력
        std::cout << "Invalid message id: 0\n";
        break;
    }
    case 1: {
        std::cout << "Case 1 " << std::endl;
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