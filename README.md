# leaf-kit-tour

> [English](README_en.md)

> leaf-kit 마크다운 툴킷 CLI를 Homebrew로 간편하게 설치할 수 있는 대화형 인스톨러입니다.

## 설치

```bash
brew tap leaf-kit/leaf-kit-tour
brew install leaf-kit-tour
```

## 실행

```bash
# 영어 (기본)
leaf-kit-tour

# 한국어
leaf-kit-tour --lang ko
```

## 기능

- **자동 업데이트** — 실행 시 최신 버전이 있으면 자동으로 업데이트 후 실행
- **설치 현황 조회** — `brew list` 기반으로 각 도구의 실제 설치 여부를 표시
- **전체 설치 / 선택 설치 / 재설치 / 업그레이드 / 삭제** — 대화형 메뉴에서 선택
- **다국어 지원** — 시작 시 언어 선택 또는 `--lang ko` / `--lang en`으로 바로 실행

## 터미널 실행 화면

### 언어 선택 (인자 없이 실행 시)

```
══════════════════════════════════════════════════════════════
          leaf-kit-tour  —  CLI Toolkit Installer
══════════════════════════════════════════════════════════════

[언어 선택 / Select Language]
  1  한국어 (기본값)
  2  English

선택/select (1)> 1
```

> Enter 또는 `1` → 한국어, `2` → English
> `--lang ko` 또는 `--lang en` 인자를 주면 선택 화면 없이 바로 시작합니다.

### 시작 화면 (한국어)

```
══════════════════════════════════════════════════════════════
          leaf-kit-tour  —  CLI Toolkit Installer
                        v0.1.0
══════════════════════════════════════════════════════════════

leaf-kit의 마크다운 툴킷 CLI 도구를 Homebrew로 간편하게 설치합니다.

[OK] Homebrew 감지됨

[*] leaf-kit-tour 최신 버전 확인 중...
[OK] 최신 버전입니다. (v0.1.0)
```

### 도구 목록 및 설치 현황

```
══════════════════════════════════════════════════════════════════════════
  #  상태   도구명        설명                                  설치 방식
══════════════════════════════════════════════════════════════════════════
  1  [최신] v1.2.0  stylemd       올인원 마크다운 툴킷 — lint, format   소스/바이너리
                                  $ stylemd lint README.md
                                  $ stylemd format --fix docs/
                                  https://github.com/leaf-kit/style.md
  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  2  [미설치]       playgraph     애니메이션 마크다운 뷰어 (UI 앱)      소스/바이너리
                                  https://github.com/leaf-kit/playgraph.md
  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  3  [업그레이드 필요] v0.8.0→v0.9.1  lsmd     마크다운 인식 디렉토리   소스/바이너리
                                  $ lsmd --tree docs/
                                  https://github.com/leaf-kit/ls.md
  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  4  [미설치]       gmd           Grep Markdown — 구조 인식 고속 검색   소스/바이너리
                                  $ gmd search "API" docs/
                                  https://github.com/leaf-kit/g.md
  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  5  [최신] v1.0.0  bark          터미널 마크다운 뷰어                   소스/바이너리
                                  $ bark README.md
                                  https://github.com/leaf-kit/bark.md
══════════════════════════════════════════════════════════════════════════
  i 소스 = 소스(Formula)빌드  바이너리 = 사전빌드 바이너리(Bottle)
```

> **상태 표시 기준**
> - `[최신] vX.Y.Z` — 설치됨, 최신 버전
> - `[업그레이드 필요] vX.Y.Z → vA.B.C` — 설치됨, 새 버전 있음
> - `[미설치]` — 아직 설치되지 않음
>
> **설치 방식**
> - `소스` — Formula를 통해 소스에서 빌드 (초록색)
> - `바이너리` — 사전 빌드된 Bottle로 즉시 설치 (청색)

### 메뉴 옵션

```
[설치 옵션]
  a  전체 설치 (모든 CLI 도구)
  번호  번호로 선택 설치 (예: 1,3,5 또는 1-3)
  r  재설치 모드
  u  전체 업그레이드 (설치된 도구 최신화)
  d  삭제 모드 (번호 지정 또는 전체 삭제)
  p  지원 플랫폼 보기
  s  설치 현황 새로고침
  q  종료

선택>
```

### 전체 설치 예시

```
선택> a

>> 전체 5개 도구를 설치합니다.

>> stylemd 설치 중...
  -> brew tap leaf-kit/stylemd
  -> brew install leaf-kit/stylemd/stylemd
  [OK] stylemd 설치 완료!

>> playgraph 설치 중...
  -> brew tap leaf-kit/playgraph
  -> brew install leaf-kit/playgraph/playgraph
  [OK] playgraph 설치 완료!

...

전체 설치 완료!
```

### 선택 설치 예시

```
선택> 1,3

>> 선택한 2개 도구를 설치합니다.

>> stylemd 설치 중...
  -> brew tap leaf-kit/stylemd
  -> brew install leaf-kit/stylemd/stylemd
  [OK] stylemd 설치 완료!

>> lsmd 설치 중...
  -> brew tap leaf-kit/lsmd
  -> brew install leaf-kit/lsmd/lsmd
  [OK] lsmd 설치 완료!

선택 설치 완료!
```

### 재설치 모드

```
선택> r

재설치할 도구 번호를 입력하세요 (예: 1,3,5 또는 a=전체):
재설치> 2

>> playgraph 재설치 중...
  -> brew reinstall leaf-kit/playgraph/playgraph
  [OK] playgraph 재설치 완료!
```

### 전체 업그레이드 (버전 비교 표시)

```
선택> u

>> 설치된 도구를 전체 업그레이드합니다.

>> stylemd 업그레이드 확인 중... (현재: v1.1.0)
  최신 버전: v1.2.0
  -> brew upgrade leaf-kit/stylemd/stylemd
  [OK] stylemd 업그레이드 완료! (v1.1.0 → v1.2.0)

[—] playgraph 미설치 상태 — 건너뜁니다.

>> lsmd 업그레이드 확인 중... (현재: v0.9.1)
  최신 버전: v0.9.1
  [OK] lsmd 이미 최신 버전입니다. (v0.9.1)

...

전체 업그레이드 완료!
```

### 삭제 모드

```
선택> d

삭제할 도구 번호를 입력하세요 (예: 1,3,5 또는 a=전체):
삭제> 1,3

>> stylemd 삭제 중...
  -> brew uninstall leaf-kit/stylemd/stylemd
  [OK] stylemd 삭제 완료!

>> lsmd 삭제 중...
  -> brew uninstall leaf-kit/lsmd/lsmd
  [OK] lsmd 삭제 완료!
```

### 전체 삭제

```
선택> d

삭제할 도구 번호를 입력하세요 (예: 1,3,5 또는 a=전체):
삭제> a

>> stylemd 삭제 중...
  -> brew uninstall leaf-kit/stylemd/stylemd
  [OK] stylemd 삭제 완료!

>> playgraph 삭제 중...
  -> brew uninstall leaf-kit/playgraph/playgraph
  [OK] playgraph 삭제 완료!

...
```

## 설치 가능한 CLI 도구 요약

| # | 도구명 | 설명 | 설치 방식 | 설치 명령 |
|---|--------|------|-----------|-----------|
| 1 | **stylemd** | 올인원 마크다운 툴킷 (lint, format, fix, analyze) | 🟢 소스 &nbsp; 🔵 바이너리 | `brew install leaf-kit/stylemd/stylemd` |
| 2 | **playgraph** | 애니메이션 마크다운 뷰어 (그래프·다이어그램 플레이) | 🟢 소스 &nbsp; 🔵 바이너리 | `brew install leaf-kit/playgraph/playgraph` |
| 3 | **lsmd** | 마크다운 인식 디렉토리 목록 도구 | 🟢 소스 &nbsp; 🔵 바이너리 | `brew install leaf-kit/lsmd/lsmd` |
| 4 | **gmd** | Grep Markdown — 마크다운 구조 인식 고속 검색 | 🟢 소스 &nbsp; 🔵 바이너리 | `brew install leaf-kit/gmd/gmd` |
| 5 | **bark** | 터미널 마크다운 뷰어 (Browse And Render Markdown) | 🟢 소스 &nbsp; 🔵 바이너리 | `brew install leaf-kit/bark/bark` |

> **설치 방식 범례**
> - 🟢 **소스(Formula)** — `brew install`로 소스에서 빌드 (Rust 툴체인 필요)
> - 🔵 **바이너리(Cask/Bottle)** — 사전 빌드된 바이너리로 즉시 설치

### 바이너리(Bottle) 지원 플랫폼

| 플랫폼 | 아키텍처 | 지원 |
|--------|---------|------|
| macOS Sonoma 14+ | Apple Silicon (arm64) | ✅ |
| macOS Sonoma 14+ | Intel (x86_64) | ✅ |
| macOS Ventura 13 | Apple Silicon (arm64) | ✅ |
| macOS Ventura 13 | Intel (x86_64) | ✅ |
| Linux (glibc 2.17+) | x86_64 | ✅ |
| Linux (glibc 2.17+) | aarch64 | ✅ |
| Windows (WSL2) | x86_64 | ⚠️ WSL2 + Homebrew로 사용 가능 |

> **참고**: Bottle이 없는 환경에서도 Formula를 통해 소스 빌드가 가능합니다. Rust 1.70+ 툴체인이 필요합니다.

### 각 도구 명령어 예시

```bash
# stylemd — 마크다운 스타일 검사·수정
stylemd lint README.md
stylemd format --fix docs/

# playgraph — 애니메이션 마크다운 뷰어 (UI 애플리케이션)
# 터미널 CLI가 아닌 UI 앱으로 실행됩니다

# lsmd — 마크다운 디렉토리 목록
lsmd
lsmd --tree docs/

# gmd — 마크다운 구조 검색
gmd search "API" docs/
gmd headings README.md

# bark — 터미널 마크다운 렌더링
bark README.md
bark --theme dark guide.md
```

### 관련 저장소

- [style.md](https://github.com/leaf-kit/style.md)
- [playgraph.md](https://github.com/leaf-kit/playgraph.md)
- [ls.md](https://github.com/leaf-kit/ls.md)
- [g.md](https://github.com/leaf-kit/g.md)
- [bark.md](https://github.com/leaf-kit/bark.md)

## 옵션

| 플래그 | 설명 |
|--------|------|
| (없음) | 대화형 언어 선택 후 시작 (기본값: 한국어) |
| `--lang ko` | 한국어 인터페이스로 바로 실행 |
| `--lang en` | 영어 인터페이스로 바로 실행 |
| `--version` | 버전 표시 |
| `--help` | 도움말 표시 |

## 플랫폼

| OS | 아키텍처 | Homebrew 설치 | 상태 |
|----|---------|--------------|------|
| macOS 13+ | Apple Silicon (arm64) | `brew install` | 🟢 기본 지원 |
| macOS 13+ | Intel (x86_64) | `brew install` | 🟢 기본 지원 |
| Linux | x86_64 / aarch64 | `brew install` (Linuxbrew) | 🟡 지원 |
| Windows | x86_64 | WSL2 + `brew install` | 🟡 지원 |

## 라이선스

MIT
