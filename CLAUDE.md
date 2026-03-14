# CLAUDE.md

이 파일은 Claude Code (claude.ai/code)가 이 저장소에서 작업할 때 참고하는 가이드입니다.

## 프로젝트 개요

QuickShotMessage는 **FPS 게임서버와 같이 고빈도·저지연 환경**을 위한 경량형 바이너리 패킷 프레임워크입니다.
- **Qnerator**: `.qsmb` 메시지 정의 파일을 읽어 언어별 직렬화 코드를 생성하는 Rust CLI 도구
- **MessageCaster**: 생성된 메시지를 송수신하는 통신 프레임워크 (C++/Rust)
- C++, Rust, C# 참조 구현체

목표: 패킷 생성 오버헤드 최소화, 힙 할당 최소화, 고빈도 메시지 처리

---

## 빌드 명령어

### Qnerator (Rust 코드 생성기)
```bash
cd Qnerator/
cargo build
cargo test
# 직접 실행:
cargo run -- -d <대상_디렉토리> <언어> <출력_디렉토리>
```

### MessageCaster C++ (Linux)
```bash
cd MessageCaster/cpp/MessageCasterLinux/
mkdir -p build && cd build
cmake ..
make
```

### C++ 참조 구현체
```bash
cd cpp/cpp/
mkdir -p build && cd build
cmake ..
make
```

### C# 구현체
```bash
dotnet build CSharp/qsm/qsm.csproj
dotnet run --project CSharp/qsm/qsm.csproj
```

---

## Qnerator CLI 사용법

```bash
# 디렉토리 내 모든 .qsmb 파일로부터 코드 생성
qnerator -d <대상_디렉토리> <언어> <출력_디렉토리>

# 언어 옵션: rust, cpp, go, python, csharp
# 출력 디렉토리에 "-" 입력 시 ./gen 디렉토리로 출력

# 예시:
qnerator -d ./Qnerator/src/example rust -
qnerator -d ./Qnerator/src/example cpp ./output
```

## 메시지 정의 형식 (.qsmb)

```
msg MessageName
{
    Integer id
    Long timestamp
    Float val
    String name
    ArrayInteger nums
    ArrayFloat vals
}
```

지원 필드 타입: `Integer`, `Long`, `Float`, `String`, `ArrayInteger`, `ArrayFloat`

---

## 아키텍처

### Qnerator 코드 생성 파이프라인

1. `main.rs` — 진입점, `gen_prompt`에 위임
2. `gen_prompt.rs` — CLI 인자 파싱, 모드 결정 (디렉토리 `-d` 또는 파일)
3. `gen_trait.rs` — `CodeGenerator` 트레이트 정의; `.qsmb` 파일 파싱(렉싱/토크나이징) 및 언어별 생성기로 디스패치
4. `code_gen_option.rs` — `lazy_static`을 이용한 싱글톤 전역 설정 (대상 디렉토리, 언어, 출력 디렉토리)
5. 언어별 생성기 (`cpp_gen.rs`, `rust_gen.rs` 등) — `CodeGenerator` 트레이트 구현; struct 정의, 직렬화, 역직렬화 코드 생성

`gen_trait.rs`의 파서가 `.qsmb` 파일을 토크나이징하여 중간 표현(필드 타입과 이름 목록)을 만들고, 각 언어 생성기가 이를 소비합니다.

생성된 C++ 코드는 packed struct + 바이트 레벨 직렬화, Rust 코드는 `pub struct` + 리틀 엔디안 바이트 메서드를 사용합니다.

### MessageCaster

프로세스 간 메시지 전달을 위한 Publisher/Subscriber 패턴. C++ Linux/Windows 변형과 Rust 변형 모두 동일한 개념의 API를 구현합니다. 현재 TCP(`SOCK_STREAM`) 기반 블로킹 I/O를 사용합니다.

---

## 현재 구현의 문제점 및 개선 방향

FPS 게임서버 환경(틱당 수백~수천 패킷, 수십~수백ms 이내 처리)을 기준으로 분석한 개선점입니다.

### 1. 직렬화 성능 (C++ 생성 코드)

**현재 문제:**
- `buffer.insert(buffer.end(), b, b+4)` 를 필드마다 반복 호출 → 매번 memcpy + 내부 bounds check 발생
- `std::string` 필드가 많을수록 힙 재할당 빈도 증가

**개선 방향:**
- `buffer.reserve(approx)` 후 `std::memcpy()` 직접 사용으로 insert 루프 제거
- 고정 크기 필드만 있는 메시지는 `memcpy` 한 번으로 전체 직렬화 가능한 POD 구조체 생성 고려

### 2. 직렬화 성능 (Rust 생성 코드)

**현재 문제:**
- `Vec::new()` 후 `extend()` 반복 → 용량 초과 시 재할당 발생
- 역직렬화 시 `to_vec()`으로 문자열 중간 복사본 생성

**개선 방향:**
- 생성 코드에 `Vec::with_capacity(추정_크기)` 추가
- `bytes::Bytes` 또는 제로카피 역직렬화(`&[u8]` 슬라이스 참조)로 string 중간 할당 제거

### 3. 버퍼 관리

**현재 문제:**
- `MessageCaster.h`의 수신 버퍼가 **2KB 고정** → 이 크기를 초과하는 패킷은 묵시적으로 잘림
- 버퍼 풀 없이 연결마다 새 버퍼 할당

**개선 방향:**
- 동적 버퍼 크기 조정 또는 링 버퍼(ring buffer) 도입
- 오브젝트 풀로 버퍼 재사용, GC 압력/힙 단편화 감소

### 4. 네트워크 I/O 모델

**현재 문제:**
- C++: 메인 스레드에서 블로킹 `recv()`, 클라이언트당 스레드 필요
- Rust: 단일 스레드 순차 `accept()` → 동시 클라이언트 처리 불가
- TCP(`SOCK_STREAM`) 사용 — 위치 업데이트 등 손실 허용 패킷에 불리

**개선 방향:**
- **C++**: `epoll` (Linux) 기반 논블로킹 I/O 또는 `io_uring`으로 이벤트 루프 전환
- **Rust**: `tokio` 또는 `async-std` 기반 비동기 I/O로 전환
- 위치/회전 등 **손실 허용 패킷은 UDP**, 인증/결제 등 **신뢰성 필요 패킷은 TCP** 분리 고려

### 5. 메시지 프레이밍

**현재 문제:**
- 길이 헤더 없이 단순 recv → TCP 스트림에서 패킷 경계가 보장되지 않음 (partial read 미처리)
- 재연결 로직 없음

**개선 방향:**
- 고정 크기 헤더(예: 4바이트 message length + 2바이트 message type ID) 표준화
- 메시지 타입 ID를 `.qsmb` 정의에서 자동 할당하여 생성 코드에 포함

### 6. 메시지 정의 확장성

**현재 문제:**
- 버전 정보 없음 → 클라이언트/서버 버전 불일치 시 탐지 불가
- `bool`, `uint8`, `Vec<T>` 등 타입이 제한적

**개선 방향:**
- 필드 타입 확장: `Bool`, `Byte`, `Short`, 중첩 메시지(nested msg) 지원
- 메시지 버전 필드 자동 삽입 옵션

### 7. 생성 코드 품질 (Go, Python, C#)

**현재 상태:**
- Go, Python, C# 생성기는 스텁(stub) 수준으로 실제 코드 생성 미구현

**개선 방향:**
- 서버 로직을 Go/C#으로 작성하는 경우가 많으므로 최소한 C# 생성기 완성 우선순위 높음
