# CLI 명령 매핑

모든 명령은 구조화된 출력을 위해 `--json`을 지원합니다.

## `quote`
실시간 스왑 금액을 가져옵니다.
- `token_address`: 대상 토큰.
- `amount`: 입력 금액.
- `--action`: `buy` 또는 `sell`.
- `--chain`: `bsc` (기본값) 또는 `solana`.

## `buy`
- `token_address`: 받을 토큰.
- `amount`: 지불할 네이티브 SOL/BNB 금액.
- `--slippage`: 0-100 (예: 15%의 경우 15.0).
- `--jito-tip`: (Solana 전용) SOL 금액.
- `--no-dry-run`: 실제 거래를 실행합니다.

## `sell`
- `token_address`: 지불할 토큰.
- `amount`: 판매할 토큰 단위.
- `--no-dry-run`: 실제 거래를 실행합니다.

## `balance`
- `--token-address`: SPL/BEP20 주소 (선택 사항).
- `--chain`: `bsc` 또는 `solana`.

## `wallet`
활성 워렛 인덱스에 대한 파생 주소를 표시합니다.
