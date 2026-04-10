# Leaf Kit Repo Tour

> leaf-kit의 마크다운 툴킷 CLI 도구를 Homebrew로 간편하게 설치할 수 있는 대화형 인스톨러입니다.

## 플랫폼 안내

- **macOS**: 현재 기본 지원
- **Linux / Windows**: 확장 예정

## 설치 가능한 CLI 도구 목록

| # | 도구명 | 설명 | 활용 상황 | 설치 명령 | 재설치 명령 |
|---|--------|------|-----------|-----------|-------------|
| 1 | **stylemd** | 올인원 마크다운 툴킷 (lint, format, fix, analyze) | 문서 스타일 검사·포맷팅·자동 수정, CI/CD 통합 | `brew install leaf-kit/stylemd/stylemd` | `brew reinstall leaf-kit/stylemd/stylemd` |
| 2 | **playgraph** | 애니메이션 마크다운 뷰어 (그래프·다이어그램 플레이) | 그래프/다이어그램 시각화, 프레젠테이션·교육 자료 | `brew install leaf-kit/playgraph/playgraph` | `brew reinstall leaf-kit/playgraph/playgraph` |
| 3 | **lsmd** | 마크다운 인식 디렉토리 목록 도구 | 마크다운 파일이 많은 프로젝트 탐색·관리 | `brew install leaf-kit/lsmd/lsmd` | `brew reinstall leaf-kit/lsmd/lsmd` |
| 4 | **gmd** | Grep Markdown — 마크다운 구조 인식 고속 검색 | 헤딩·코드 블록·링크 등 구조적 검색 | `brew install leaf-kit/gmd/gmd` | `brew reinstall leaf-kit/gmd/gmd` |
| 5 | **bark** | 터미널 마크다운 뷰어 (Browse And Render Markdown) | 터미널/SSH에서 마크다운 문서 렌더링 | `brew install leaf-kit/bark/bark` | `brew reinstall leaf-kit/bark/bark` |

### 관련 저장소

- [style.md](https://github.com/leaf-kit/style.md)
- [playgraph.md](https://github.com/leaf-kit/playgraph.md)
- [ls.md](https://github.com/leaf-kit/ls.md)
- [g.md](https://github.com/leaf-kit/g.md)
- [bark.md](https://github.com/leaf-kit/bark.md)

## 빌드 및 실행

Rust가 설치되어 있어야 합니다.

```bash
# 빌드
cargo build --release

# 실행
cargo run
# 또는
./target/release/repo-tour
```

## 사용법

실행하면 대화형 메뉴가 표시됩니다:

- **`a`** — 전체 설치 (5개 CLI 도구 모두)
- **`1,3,5`** — 번호로 선택 설치 (쉼표 구분)
- **`1-3`** — 범위로 선택 설치
- **`r`** — 재설치 모드 진입
- **`q`** — 종료

## 기술 스택

- **Rust** — 메인 언어
- **Homebrew** — macOS 패키지 관리

## 라이선스

MIT
