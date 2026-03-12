# FreedomAgent Rust v0.6.0

FreedomAgent는 LLM 에이전트를 위해 설계된 상태 비저장(stateless), JSON 우선, 멀티체인 암호화폐 거래 실행 레이어입니다.

## 주요 기능
- **멀티체인 지원**: BNB 스마트 체인(BSC) 및 Solana 기본 실행 지원.
- **에이전트 우선 설계**: LLM과의 원활한 통합을 위한 JSON 우선 통신 프로토콜.
- **고성능**: 병렬 RPC 레이싱 및 트랜잭션 사전 구축.
- **Solana 경로 탐색**: Pump.fun 및 Raydium V3 자동 감지 및 실행.
- **보안**: 강제 dry-run 시뮬레이션 및 안전한 개인 키 관리.

## 시작하기
1. 저장소를 클론합니다.
2. `.env.example`을 `.env`로 복사하고 키를 추가합니다.
3. 빌드: `cargo build --release`
4. 실행: `./target/release/kinesis_rs balance --chain solana`

