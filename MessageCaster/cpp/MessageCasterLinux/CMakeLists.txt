cmake_minimum_required(VERSION 3.0)
project(MessageCasterLinux)

set(CMAKE_CXX_STANDARD 11)

# 소스 파일 자동 검색
file(GLOB_RECURSE SOURCES 
    src/main.cpp
    src/MessageCaster/*.cpp
    src/Messages/*.cpp
)

# 헤더 포함 디렉터리 추가
include_directories(
    src
    src/MessageCaster
    src/Messages
)

# 실행 파일 생성
add_executable(MessageCasterLinux ${SOURCES})

# 스레드 라이브러리 찾기
find_package(Threads REQUIRED)

# 라이브러리 링크
target_link_libraries(MessageCasterLinux Threads::Threads)
