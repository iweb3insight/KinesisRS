# 환경 및 설정

## 환경 변수
- `BSC_RPC_URL`: 쉼표로 구분된 RPC URL.
- `SOL_RPC_URL`: Solana RPC 엔드포인트.
- `BSC_PRIVATE_KEY_1`, `BSC_PRIVATE_KEY_2` 등.
- `SOL_PRIVATE_KEY_1`, `SOL_PRIVATE_KEY_2` 등.
- `JITO_RPC_URL`: (선택 사항) Jito Block Engine URL.

## 바이너리 사용
이 스킬은 `solana_claw_coin_cli` 바이너리가 빌드되어 프로젝트 루트에 있거나 PATH에 추가되었다고 가정합니다.
빌드 방법: `cargo build --release`
바이너리 위치: `target/release/solana_claw_coin_cli`
