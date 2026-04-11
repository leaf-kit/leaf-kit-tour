use colored::Colorize;
use std::io::{self, Write};
use std::process::Command;

const VERSION: &str = env!("CARGO_PKG_VERSION");

struct Tool {
    name: &'static str,
    tap: &'static str,
    formula: &'static str,
    description: &'static str,
    description_ko: &'static str,
    example: &'static str,
    example_ko: &'static str,
    repo: &'static str,
    has_source: bool,
    has_binary: bool,
    supported_os: &'static [&'static str],
    binary_platforms: &'static [&'static str],
}

const TOOLS: &[Tool] = &[
    Tool {
        name: "stylemd",
        tap: "leaf-kit/stylemd",
        formula: "stylemd",
        description: "All-in-one Markdown toolkit — lint, format, fix, analyze",
        description_ko: "올인원 마크다운 툴킷 — lint, format, fix, analyze를 하나의 빠른 CLI로",
        example: "stylemd lint README.md\nstylemd format --fix docs/",
        example_ko: "stylemd lint README.md        # 마크다운 스타일 검사\nstylemd format --fix docs/    # 자동 포맷팅 및 수정",
        repo: "https://github.com/leaf-kit/style.md",
        has_source: true,
        has_binary: true,
        supported_os: &["macOS", "Linux", "Windows (WSL2)"],
        binary_platforms: &["macOS arm64", "macOS x86_64", "Linux x86_64", "Linux aarch64"],
    },
    Tool {
        name: "playgraph",
        tap: "leaf-kit/playgraph",
        formula: "playgraph",
        description: "Animated Markdown viewer — play text, graphs, and diagrams",
        description_ko: "세계 최초의 애니메이션 마크다운 뷰어 — 텍스트, 그래프, 다이어그램을 플레이",
        example: "",
        example_ko: "",
        repo: "https://github.com/leaf-kit/playgraph.md",
        has_source: true,
        has_binary: true,
        supported_os: &["macOS"],
        binary_platforms: &["macOS arm64", "macOS x86_64"],
    },
    Tool {
        name: "lsmd",
        tap: "leaf-kit/lsmd",
        formula: "lsmd",
        description: "Markdown-aware directory listing tool",
        description_ko: "마크다운 인식 디렉토리 목록 도구 — ls 명령의 마크다운 특화 버전",
        example: "lsmd\nlsmd --tree docs/",
        example_ko: "lsmd                          # 현재 디렉토리 마크다운 목록\nlsmd --tree docs/             # 트리 형태로 문서 탐색",
        repo: "https://github.com/leaf-kit/ls.md",
        has_source: true,
        has_binary: true,
        supported_os: &["macOS", "Linux", "Windows (WSL2)"],
        binary_platforms: &["macOS arm64", "macOS x86_64", "Linux x86_64", "Linux aarch64"],
    },
    Tool {
        name: "gmd",
        tap: "leaf-kit/gmd",
        formula: "gmd",
        description: "Grep Markdown — structure-aware fast search and analysis",
        description_ko: "Grep Markdown — 마크다운 구조 인식 고속 검색 및 분석 도구",
        example: "gmd search \"API\" docs/\ngmd headings README.md",
        example_ko: "gmd search \"API\" docs/        # 마크다운 내 구조적 검색\ngmd headings README.md        # 헤딩 구조 추출",
        repo: "https://github.com/leaf-kit/g.md",
        has_source: true,
        has_binary: true,
        supported_os: &["macOS", "Linux", "Windows (WSL2)"],
        binary_platforms: &["macOS arm64", "macOS x86_64", "Linux x86_64", "Linux aarch64"],
    },
    Tool {
        name: "bark",
        tap: "leaf-kit/bark",
        formula: "bark",
        description: "Terminal Markdown viewer — Browse And Render marKdown",
        description_ko: "터미널 마크다운 뷰어 — Browse And Render Markdown, Keenly",
        example: "bark README.md\nbark --theme dark docs/guide.md",
        example_ko: "bark README.md                # 터미널에서 마크다운 렌더링\nbark --theme dark guide.md    # 다크 테마로 문서 보기",
        repo: "https://github.com/leaf-kit/bark.md",
        has_source: true,
        has_binary: true,
        supported_os: &["macOS", "Linux", "Windows (WSL2)"],
        binary_platforms: &["macOS arm64", "macOS x86_64", "Linux x86_64", "Linux aarch64"],
    },
];

struct PlatformInfo {
    os: &'static str,
    arch: &'static str,
    status: &'static str,
    status_ko: &'static str,
}

const PLATFORMS: &[PlatformInfo] = &[
    PlatformInfo { os: "macOS 14+ (Sonoma)",  arch: "Apple Silicon (arm64)", status: "Supported",          status_ko: "지원" },
    PlatformInfo { os: "macOS 14+ (Sonoma)",  arch: "Intel (x86_64)",       status: "Supported",          status_ko: "지원" },
    PlatformInfo { os: "macOS 13 (Ventura)",  arch: "Apple Silicon (arm64)", status: "Supported",          status_ko: "지원" },
    PlatformInfo { os: "macOS 13 (Ventura)",  arch: "Intel (x86_64)",       status: "Supported",          status_ko: "지원" },
    PlatformInfo { os: "Linux (glibc 2.17+)", arch: "x86_64",              status: "Supported",          status_ko: "지원" },
    PlatformInfo { os: "Linux (glibc 2.17+)", arch: "aarch64",             status: "Supported",          status_ko: "지원" },
    PlatformInfo { os: "Windows (WSL2)",      arch: "x86_64",              status: "Via WSL2 + Homebrew", status_ko: "WSL2 + Homebrew" },
];

fn print_banner(lang: &str) {
    println!();
    println!(
        "{}",
        "══════════════════════════════════════════════════════════════"
            .green()
    );
    println!(
        "{}",
        "          leaf-kit-tour  —  CLI Toolkit Installer            "
            .green()
            .bold()
    );
    println!(
        "{}",
        format!("                        v{}", VERSION)
            .green()
    );
    println!(
        "{}",
        "══════════════════════════════════════════════════════════════"
            .green()
    );
    println!();
    if lang == "ko" {
        println!(
            "{}",
            "leaf-kit의 마크다운 툴킷 CLI 도구를 Homebrew로 간편하게 설치합니다."
                .bright_white()
        );
    } else {
        println!(
            "{}",
            "Install leaf-kit Markdown CLI tools easily via Homebrew."
                .bright_white()
        );
    }
    println!();
}

fn get_self_latest_version() -> Option<String> {
    let result = Command::new("brew")
        .args(["info", "--json=v2", "leaf-kit/leaf-kit-tour/leaf-kit-tour"])
        .output();
    match result {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if let Some(pos) = stdout.find("\"stable\":\"") {
                let start = pos + "\"stable\":\"".len();
                if let Some(end) = stdout[start..].find('"') {
                    return Some(stdout[start..start + end].to_string());
                }
            }
            None
        }
        _ => None,
    }
}

fn check_self_update(lang: &str) {
    if lang == "ko" {
        println!(
            "{} leaf-kit-tour 최신 버전 확인 중...",
            "[*]".cyan().bold()
        );
    } else {
        println!(
            "{} Checking for leaf-kit-tour updates...",
            "[*]".cyan().bold()
        );
    }

    let latest = get_self_latest_version();

    match latest {
        Some(latest_ver) => {
            if latest_ver == VERSION {
                if lang == "ko" {
                    println!("{} 최신 버전입니다. (v{})\n", "[OK]".green().bold(), VERSION);
                } else {
                    println!("{} Already up to date. (v{})\n", "[OK]".green().bold(), VERSION);
                }
                return;
            }

            if lang == "ko" {
                println!(
                    "{} 새 버전이 있습니다: v{} (현재: v{})",
                    "[*]".yellow().bold(),
                    latest_ver.cyan(),
                    VERSION
                );
            } else {
                println!(
                    "{} New version available: v{} (current: v{})",
                    "[*]".yellow().bold(),
                    latest_ver.cyan(),
                    VERSION
                );
            }

            let answer = {
                let msg = if lang == "ko" {
                    format!(
                        "{} v{}(으)로 업데이트하시겠습니까? (Y/n) ",
                        ">>".cyan().bold(),
                        latest_ver
                    )
                } else {
                    format!(
                        "{} Update to v{}? (Y/n) ",
                        ">>".cyan().bold(),
                        latest_ver
                    )
                };
                prompt_input(&msg)
            };

            if answer == "n" || answer == "N" || answer == "no" {
                if lang == "ko" {
                    println!("{} 현재 버전(v{})으로 계속합니다.\n", "[*]".cyan().bold(), VERSION);
                } else {
                    println!("{} Continuing with current version (v{}).\n", "[*]".cyan().bold(), VERSION);
                }
                return;
            }

            if lang == "ko" {
                println!("{} 업데이트 중...", "[*]".yellow().bold());
            } else {
                println!("{} Updating...", "[*]".yellow().bold());
            }

            let update = Command::new("brew")
                .args(["upgrade", "leaf-kit/leaf-kit-tour/leaf-kit-tour"])
                .status();
            match update {
                Ok(status) if status.success() => {
                    if lang == "ko" {
                        println!(
                            "{} 업데이트 완료! (v{} → v{}) 최신 버전으로 다시 실행합니다.\n",
                            "[OK]".green().bold(),
                            VERSION,
                            latest_ver
                        );
                    } else {
                        println!(
                            "{} Updated! (v{} → v{}) Restarting with the latest version.\n",
                            "[OK]".green().bold(),
                            VERSION,
                            latest_ver
                        );
                    }
                    let args: Vec<String> = std::env::args().collect();
                    let _ = Command::new(&args[0])
                        .args(&args[1..])
                        .status();
                    std::process::exit(0);
                }
                _ => {
                    if lang == "ko" {
                        println!(
                            "{} 업데이트 실패. 현재 버전으로 계속합니다.\n",
                            "[!]".yellow().bold()
                        );
                    } else {
                        println!(
                            "{} Update failed. Continuing with current version.\n",
                            "[!]".yellow().bold()
                        );
                    }
                }
            }
        }
        None => {
            if lang == "ko" {
                println!(
                    "{} 업데이트 확인을 건너뜁니다. (brew를 통한 설치가 아닐 수 있음)\n",
                    "[!]".yellow().bold()
                );
            } else {
                println!(
                    "{} Skipping update check. (may not be installed via brew)\n",
                    "[!]".yellow().bold()
                );
            }
        }
    }
}

struct ToolStatus {
    installed: bool,
    version: Option<String>,
    latest: Option<String>,
}

impl ToolStatus {
    fn needs_upgrade(&self) -> bool {
        match (&self.version, &self.latest) {
            (Some(cur), Some(lat)) => cur != lat,
            _ => false,
        }
    }
}

fn get_install_status() -> Vec<ToolStatus> {
    // Get installed versions
    let result = Command::new("brew")
        .args(["list", "--formula", "--versions"])
        .output();

    let lines: Vec<String> = match result {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            stdout.lines().map(|l| l.trim().to_string()).collect()
        }
        Err(_) => vec![],
    };

    TOOLS
        .iter()
        .map(|tool| {
            let found = lines.iter().find(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                parts.first().map_or(false, |name| *name == tool.formula)
            });
            match found {
                Some(line) => {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    let version = parts.last().map(|v| v.to_string());
                    let latest = get_latest_version(tool);
                    ToolStatus {
                        installed: true,
                        version,
                        latest,
                    }
                }
                None => ToolStatus {
                    installed: false,
                    version: None,
                    latest: None,
                },
            }
        })
        .collect()
}

fn install_type_badge(tool: &Tool, lang: &str) -> String {
    let mut parts = Vec::new();
    if tool.has_source {
        if lang == "ko" {
            parts.push(format!("{}", "소스".green()));
        } else {
            parts.push(format!("{}", "SRC".green()));
        }
    }
    if tool.has_binary {
        if lang == "ko" {
            parts.push(format!("{}", "바이너리".cyan()));
        } else {
            parts.push(format!("{}", "BIN".cyan()));
        }
    }
    parts.join(&format!("{}", "/".bright_black()))
}

fn print_tool_details(lang: &str) {
    let statuses = get_install_status();

    println!();
    if lang == "ko" {
        println!("{}", "[도구 상세 정보]".yellow().bold());
    } else {
        println!("{}", "[Tool Details]".yellow().bold());
    }

    for (i, tool) in TOOLS.iter().enumerate() {
        let status = &statuses[i];
        println!(
            "{}",
            "──────────────────────────────────────────────────────────────"
                .bright_black()
        );
        println!(
            "  {} {}",
            format!("{}.", i + 1).cyan().bold(),
            tool.name.green().bold()
        );

        // Version
        if status.installed {
            let ver = status.version.as_deref().unwrap_or("?");
            if status.needs_upgrade() {
                let lat = status.latest.as_deref().unwrap_or("?");
                if lang == "ko" {
                    println!(
                        "     {} v{} (최신: v{})",
                        "버전:".bold(),
                        ver.yellow(),
                        lat.cyan()
                    );
                } else {
                    println!(
                        "     {} v{} (latest: v{})",
                        "Version:".bold(),
                        ver.yellow(),
                        lat.cyan()
                    );
                }
            } else {
                if lang == "ko" {
                    println!("     {} v{}", "버전:".bold(), ver.green());
                } else {
                    println!("     {} v{}", "Version:".bold(), ver.green());
                }
            }
        } else {
            if lang == "ko" {
                println!("     {} {}", "버전:".bold(), "미설치".bright_black());
            } else {
                println!("     {} {}", "Version:".bold(), "not installed".bright_black());
            }
        }

        // Supported OS
        if lang == "ko" {
            println!(
                "     {} {}",
                "지원 OS:".bold(),
                tool.supported_os.join(", ")
            );
        } else {
            println!(
                "     {} {}",
                "Supported OS:".bold(),
                tool.supported_os.join(", ")
            );
        }

        // Source code
        if lang == "ko" {
            if tool.has_source {
                println!(
                    "     {} {} ({})",
                    "소스코드:".bold(),
                    "있음".green(),
                    tool.repo.underline().bright_black()
                );
            } else {
                println!(
                    "     {} {}",
                    "소스코드:".bold(),
                    "없음 (바이너리 전용)".bright_black()
                );
            }
        } else {
            if tool.has_source {
                println!(
                    "     {} {} ({})",
                    "Source:".bold(),
                    "available".green(),
                    tool.repo.underline().bright_black()
                );
            } else {
                println!(
                    "     {} {}",
                    "Source:".bold(),
                    "none (binary only)".bright_black()
                );
            }
        }

        // Binary platforms
        if lang == "ko" {
            if tool.has_binary {
                println!(
                    "     {} {}",
                    "바이너리:".bold(),
                    tool.binary_platforms
                        .iter()
                        .map(|p| format!("{}", p.cyan()))
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            } else {
                println!(
                    "     {} {}",
                    "바이너리:".bold(),
                    "없음 (소스 빌드 전용)".bright_black()
                );
            }
        } else {
            if tool.has_binary {
                println!(
                    "     {} {}",
                    "Binary:".bold(),
                    tool.binary_platforms
                        .iter()
                        .map(|p| format!("{}", p.cyan()))
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            } else {
                println!(
                    "     {} {}",
                    "Binary:".bold(),
                    "none (source build only)".bright_black()
                );
            }
        }

        // Install type badge
        if lang == "ko" {
            println!(
                "     {} {}",
                "설치 방식:".bold(),
                install_type_badge(tool, lang)
            );
        } else {
            println!(
                "     {} {}",
                "Install:".bold(),
                install_type_badge(tool, lang)
            );
        }
    }

    println!(
        "{}",
        "──────────────────────────────────────────────────────────────"
            .bright_black()
    );
    if lang == "ko" {
        println!(
            "  {} 소스 빌드는 모든 플랫폼에서 가능 (Rust 1.70+)",
            "i".cyan().bold()
        );
    } else {
        println!(
            "  {} Source build available on all platforms (Rust 1.70+)",
            "i".cyan().bold()
        );
    }
    println!();
}

fn print_platform_support(lang: &str) {
    println!();
    if lang == "ko" {
        println!("{}", "[바이너리(Bottle) 지원 플랫폼]".yellow().bold());
    } else {
        println!("{}", "[Pre-built Binary (Bottle) Platforms]".yellow().bold());
    }
    println!(
        "{}",
        "──────────────────────────────────────────────────────────────"
            .bright_black()
    );
    if lang == "ko" {
        println!(
            "  {:<24} {:<24} {}",
            "플랫폼".bold(),
            "아키텍처".bold(),
            "지원".bold()
        );
    } else {
        println!(
            "  {:<24} {:<24} {}",
            "Platform".bold(),
            "Architecture".bold(),
            "Support".bold()
        );
    }
    println!(
        "{}",
        "──────────────────────────────────────────────────────────────"
            .bright_black()
    );

    for p in PLATFORMS {
        let status_str = if lang == "ko" { p.status_ko } else { p.status };
        let icon = if status_str.contains("WSL") || status_str.contains("WSL2") {
            format!("{}", "⚠".yellow())
        } else {
            format!("{}", "✓".green())
        };
        println!(
            "  {:<24} {:<24} {} {}",
            p.os, p.arch, icon, status_str
        );
    }

    println!(
        "{}",
        "──────────────────────────────────────────────────────────────"
            .bright_black()
    );
    if lang == "ko" {
        println!(
            "  {} Bottle이 없는 환경에서도 소스 빌드 가능 (Rust 1.70+)",
            "i".cyan().bold()
        );
    } else {
        println!(
            "  {} Source build available on all platforms (Rust 1.70+)",
            "i".cyan().bold()
        );
    }
    println!();
}

fn print_tool_list(lang: &str) {
    let statuses = get_install_status();

    println!(
        "{}",
        "══════════════════════════════════════════════════════════════════════════"
            .bright_black()
    );
    if lang == "ko" {
        println!(
            "  {}  {}   {:<12}  {:<36} {}",
            "#".bold(),
            "상태".bold(),
            "도구명".bold(),
            "설명".bold(),
            "설치 방식".bold()
        );
    } else {
        println!(
            "  {}  {}   {:<12}  {:<36} {}",
            "#".bold(),
            "Status".bold(),
            "Tool".bold(),
            "Description".bold(),
            "Install".bold()
        );
    }
    println!(
        "{}",
        "══════════════════════════════════════════════════════════════════════════"
            .bright_black()
    );

    for (i, tool) in TOOLS.iter().enumerate() {
        let status = &statuses[i];
        let (status_icon, version_info) = if status.installed {
            if status.needs_upgrade() {
                let cur = status.version.as_deref().unwrap_or("?");
                let lat = status.latest.as_deref().unwrap_or("?");
                let icon = if lang == "ko" {
                    "[업그레이드 필요]".yellow().bold().to_string()
                } else {
                    "[upgrade]".yellow().bold().to_string()
                };
                let info = format!("v{} → v{}", cur, lat).yellow().to_string();
                (icon, info)
            } else {
                let ver = status.version.as_deref().unwrap_or("?");
                let icon = if lang == "ko" {
                    "[최신]".green().bold().to_string()
                } else {
                    "[latest]".green().bold().to_string()
                };
                let info = format!("v{}", ver).green().to_string();
                (icon, info)
            }
        } else {
            let icon = if lang == "ko" {
                "[미설치]".bright_black().to_string()
            } else {
                "[not installed]".bright_black().to_string()
            };
            (icon, String::new())
        };

        let desc = if lang == "ko" {
            tool.description_ko
        } else {
            tool.description
        };

        let badge = install_type_badge(tool, lang);

        if version_info.is_empty() {
            println!(
                "  {}  {}  {:<12}  {:<36} {}",
                format!("{}", i + 1).cyan().bold(),
                status_icon,
                tool.name.green().bold(),
                desc,
                badge
            );
        } else {
            println!(
                "  {}  {} {}  {:<12}  {:<36} {}",
                format!("{}", i + 1).cyan().bold(),
                status_icon,
                version_info,
                tool.name.green().bold(),
                desc,
                badge
            );
        }

        let example = if lang == "ko" {
            tool.example_ko
        } else {
            tool.example
        };

        for line in example.lines() {
            println!(
                "     {}              {}  {}",
                "",
                "$".bright_black(),
                line.bright_black()
            );
        }

        println!(
            "     {}              {}",
            "",
            tool.repo.underline().bright_black()
        );

        if i < TOOLS.len() - 1 {
            println!(
                "{}",
                "  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -"
                    .bright_black()
            );
        }
    }

    println!(
        "{}",
        "══════════════════════════════════════════════════════════════════════════"
            .bright_black()
    );

    // Install type legend
    if lang == "ko" {
        println!(
            "  {} {} = 소스(Formula)빌드  {} = 사전빌드 바이너리(Bottle)",
            "i".cyan().bold(),
            "소스".green(),
            "바이너리".cyan()
        );
    } else {
        println!(
            "  {} {} = Source (Formula) build  {} = Pre-built binary (Bottle)",
            "i".cyan().bold(),
            "SRC".green(),
            "BIN".cyan()
        );
    }
    println!();
}

fn print_menu(lang: &str) {
    if lang == "ko" {
        println!("{}", "[설치 옵션]".yellow().bold());
        println!("  {}  전체 설치 (모든 CLI 도구)", "a".cyan().bold());
        println!(
            "  {}  번호로 선택 설치 (예: 1,3,5 또는 1-3)",
            "번호".cyan().bold()
        );
        println!("  {}  재설치 모드", "r".cyan().bold());
        println!("  {}  전체 업그레이드 (설치된 도구 최신화)", "u".cyan().bold());
        println!("  {}  삭제 모드 (번호 지정 또는 전체 삭제)", "d".cyan().bold());
        println!("  {}  도구 상세 정보 (OS, 소스, 바이너리)", "i".cyan().bold());
        println!("  {}  지원 플랫폼 보기", "p".cyan().bold());
        println!("  {}  설치 현황 새로고침", "s".cyan().bold());
        println!("  {}  종료", "q".cyan().bold());
    } else {
        println!("{}", "[Options]".yellow().bold());
        println!("  {}  Install all CLI tools", "a".cyan().bold());
        println!(
            "  {}    Select by number (e.g. 1,3,5 or 1-3)",
            "N".cyan().bold()
        );
        println!("  {}  Reinstall mode", "r".cyan().bold());
        println!("  {}  Upgrade all installed tools", "u".cyan().bold());
        println!("  {}  Uninstall mode (by number or all)", "d".cyan().bold());
        println!("  {}  Tool details (OS, source, binary)", "i".cyan().bold());
        println!("  {}  Show supported platforms", "p".cyan().bold());
        println!("  {}  Refresh install status", "s".cyan().bold());
        println!("  {}  Quit", "q".cyan().bold());
    }
    println!();
}

fn parse_selection(input: &str) -> Vec<usize> {
    let mut indices = Vec::new();
    for part in input.split(',') {
        let part = part.trim();
        if part.contains('-') {
            let range: Vec<&str> = part.split('-').collect();
            if range.len() == 2 {
                if let (Ok(start), Ok(end)) = (
                    range[0].trim().parse::<usize>(),
                    range[1].trim().parse::<usize>(),
                ) {
                    for i in start..=end {
                        if i >= 1 && i <= TOOLS.len() {
                            indices.push(i - 1);
                        }
                    }
                }
            }
        } else if let Ok(n) = part.parse::<usize>() {
            if n >= 1 && n <= TOOLS.len() {
                indices.push(n - 1);
            }
        }
    }
    indices.sort();
    indices.dedup();
    indices
}

fn check_brew_installed() -> bool {
    Command::new("brew").arg("--version").output().is_ok()
}

fn install_tool(tool: &Tool, lang: &str) {
    if lang == "ko" {
        println!(
            "\n{} {} 설치 중...",
            ">>".green().bold(),
            tool.name.green().bold()
        );
    } else {
        println!(
            "\n{} Installing {}...",
            ">>".green().bold(),
            tool.name.green().bold()
        );
    }

    println!("  -> brew tap {}", tool.tap);
    let tap_result = Command::new("brew").args(["tap", tool.tap]).status();

    match tap_result {
        Ok(status) if status.success() => {}
        _ => {
            if lang == "ko" {
                println!("  {} tap 추가 실패: brew tap {}", "[X]".red().bold(), tool.tap);
            } else {
                println!("  {} tap failed: brew tap {}", "[X]".red().bold(), tool.tap);
            }
            return;
        }
    }

    let formula = format!("{}/{}", tool.tap, tool.formula);
    println!("  -> brew install {}", formula);
    let install_result = Command::new("brew").args(["install", &formula]).status();

    match install_result {
        Ok(status) if status.success() => {
            if lang == "ko" {
                println!(
                    "  {} {} 설치 완료!",
                    "[OK]".green().bold(),
                    tool.name.green().bold()
                );
            } else {
                println!(
                    "  {} {} installed!",
                    "[OK]".green().bold(),
                    tool.name.green().bold()
                );
            }
        }
        _ => {
            if lang == "ko" {
                println!(
                    "  {} {} 설치 실패. 수동으로 시도해보세요: brew install {}",
                    "[FAIL]".red().bold(),
                    tool.name,
                    formula
                );
            } else {
                println!(
                    "  {} {} install failed. Try manually: brew install {}",
                    "[FAIL]".red().bold(),
                    tool.name,
                    formula
                );
            }
        }
    }
}

fn get_tool_version(tool: &Tool) -> Option<String> {
    let result = Command::new("brew")
        .args(["list", "--formula", "--versions"])
        .output();
    match result {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            stdout.lines().find_map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.first().map_or(false, |name| *name == tool.formula) {
                    parts.last().map(|v| v.to_string())
                } else {
                    None
                }
            })
        }
        Err(_) => None,
    }
}

fn get_latest_version(tool: &Tool) -> Option<String> {
    let formula = format!("{}/{}", tool.tap, tool.formula);
    let result = Command::new("brew")
        .args(["info", "--json=v2", &formula])
        .output();
    match result {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // Parse JSON manually to find "stable" version
            // Format: "version":{"stable":"x.y.z",...}
            if let Some(pos) = stdout.find("\"stable\":\"") {
                let start = pos + "\"stable\":\"".len();
                if let Some(end) = stdout[start..].find('"') {
                    return Some(stdout[start..start + end].to_string());
                }
            }
            // Fallback: look for "versions":{"stable":"x.y.z"}
            if let Some(pos) = stdout.find("\"version\":\"") {
                let start = pos + "\"version\":\"".len();
                if let Some(end) = stdout[start..].find('"') {
                    return Some(stdout[start..start + end].to_string());
                }
            }
            None
        }
        _ => None,
    }
}

fn upgrade_tool(tool: &Tool, lang: &str) {
    let formula = format!("{}/{}", tool.tap, tool.formula);

    // Get current version before upgrade
    let current_version = get_tool_version(tool);

    if current_version.is_none() {
        if lang == "ko" {
            println!(
                "\n{} {} 미설치 상태 — 건너뜁니다.",
                "[—]".bright_black(),
                tool.name
            );
        } else {
            println!(
                "\n{} {} not installed — skipping.",
                "[—]".bright_black(),
                tool.name
            );
        }
        return;
    }

    let cur_ver = current_version.unwrap();

    if lang == "ko" {
        println!(
            "\n{} {} 업그레이드 확인 중... (현재: v{})",
            ">>".cyan().bold(),
            tool.name.cyan().bold(),
            cur_ver
        );
    } else {
        println!(
            "\n{} Checking upgrade for {}... (current: v{})",
            ">>".cyan().bold(),
            tool.name.cyan().bold(),
            cur_ver
        );
    }

    // Check latest available version
    let latest_version = get_latest_version(tool);
    if let Some(ref latest) = latest_version {
        if lang == "ko" {
            println!("  최신 버전: v{}", latest.cyan());
        } else {
            println!("  Latest version: v{}", latest.cyan());
        }
    }

    // If versions match, skip upgrade
    if let Some(ref latest) = latest_version {
        if latest == &cur_ver {
            if lang == "ko" {
                println!(
                    "  {} {} 이미 최신 버전입니다. (v{})",
                    "[OK]".green().bold(),
                    tool.name.green().bold(),
                    cur_ver
                );
            } else {
                println!(
                    "  {} {} already up to date. (v{})",
                    "[OK]".green().bold(),
                    tool.name.green().bold(),
                    cur_ver
                );
            }
            return;
        }
    }

    println!("  -> brew upgrade {}", formula);
    let result = Command::new("brew").args(["upgrade", &formula]).output();

    match result {
        Ok(output) if output.status.success() => {
            let new_version = get_tool_version(tool);
            let new_ver = new_version.as_deref().unwrap_or("?");

            if new_ver == cur_ver {
                if lang == "ko" {
                    println!(
                        "  {} {} 이미 최신 버전입니다. (v{})",
                        "[OK]".green().bold(),
                        tool.name.green().bold(),
                        cur_ver
                    );
                } else {
                    println!(
                        "  {} {} already up to date. (v{})",
                        "[OK]".green().bold(),
                        tool.name.green().bold(),
                        cur_ver
                    );
                }
            } else {
                if lang == "ko" {
                    println!(
                        "  {} {} 업그레이드 완료! (v{} → v{})",
                        "[OK]".green().bold(),
                        tool.name.green().bold(),
                        cur_ver,
                        new_ver
                    );
                } else {
                    println!(
                        "  {} {} upgraded! (v{} → v{})",
                        "[OK]".green().bold(),
                        tool.name.green().bold(),
                        cur_ver,
                        new_ver
                    );
                }
            }
        }
        _ => {
            if lang == "ko" {
                println!("  {} {} 업그레이드 실패.", "[FAIL]".red().bold(), tool.name);
            } else {
                println!("  {} {} upgrade failed.", "[FAIL]".red().bold(), tool.name);
            }
        }
    }
}

fn uninstall_tool(tool: &Tool, lang: &str) {
    let formula = format!("{}/{}", tool.tap, tool.formula);
    if lang == "ko" {
        println!(
            "\n{} {} 삭제 중...",
            ">>".red().bold(),
            tool.name.red().bold()
        );
    } else {
        println!(
            "\n{} Uninstalling {}...",
            ">>".red().bold(),
            tool.name.red().bold()
        );
    }
    println!("  -> brew uninstall {}", formula);

    let result = Command::new("brew").args(["uninstall", &formula]).status();

    match result {
        Ok(status) if status.success() => {
            if lang == "ko" {
                println!(
                    "  {} {} 삭제 완료!",
                    "[OK]".green().bold(),
                    tool.name.green().bold()
                );
            } else {
                println!(
                    "  {} {} uninstalled!",
                    "[OK]".green().bold(),
                    tool.name.green().bold()
                );
            }
        }
        _ => {
            if lang == "ko" {
                println!("  {} {} 삭제 실패.", "[FAIL]".red().bold(), tool.name);
            } else {
                println!("  {} {} uninstall failed.", "[FAIL]".red().bold(), tool.name);
            }
        }
    }
}

fn reinstall_tool(tool: &Tool, lang: &str) {
    let formula = format!("{}/{}", tool.tap, tool.formula);
    if lang == "ko" {
        println!(
            "\n{} {} 재설치 중...",
            ">>".yellow().bold(),
            tool.name.yellow().bold()
        );
    } else {
        println!(
            "\n{} Reinstalling {}...",
            ">>".yellow().bold(),
            tool.name.yellow().bold()
        );
    }
    println!("  -> brew reinstall {}", formula);

    let result = Command::new("brew").args(["reinstall", &formula]).status();

    match result {
        Ok(status) if status.success() => {
            if lang == "ko" {
                println!(
                    "  {} {} 재설치 완료!",
                    "[OK]".green().bold(),
                    tool.name.green().bold()
                );
            } else {
                println!(
                    "  {} {} reinstalled!",
                    "[OK]".green().bold(),
                    tool.name.green().bold()
                );
            }
        }
        _ => {
            if lang == "ko" {
                println!("  {} {} 재설치 실패.", "[FAIL]".red().bold(), tool.name);
            } else {
                println!("  {} {} reinstall failed.", "[FAIL]".red().bold(), tool.name);
            }
        }
    }
}

fn prompt_input(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn print_usage() {
    println!("leaf-kit-tour — leaf-kit CLI toolkit installer\n");
    println!("USAGE:");
    println!("  leaf-kit-tour [OPTIONS]\n");
    println!("OPTIONS:");
    println!("  (no flag)       대화형 언어 선택 (기본값: 한국어)");
    println!("  --lang ko       한국어 인터페이스로 바로 실행");
    println!("  --lang en       Run in English directly");
    println!("  --version       Show version / 버전 표시");
    println!("  --help          Show this help / 도움말 표시");
}

fn select_language() -> &'static str {
    println!();
    println!(
        "{}",
        "══════════════════════════════════════════════════════════════"
            .green()
    );
    println!(
        "{}",
        "          leaf-kit-tour  —  CLI Toolkit Installer            "
            .green()
            .bold()
    );
    println!(
        "{}",
        "══════════════════════════════════════════════════════════════"
            .green()
    );
    println!();
    println!("{}", "[언어 선택 / Select Language]".yellow().bold());
    println!("  {}  한국어 (기본값)", "1".cyan().bold());
    println!("  {}  English", "2".cyan().bold());
    println!();

    let input = prompt_input(&format!("{} ", "선택/select (1)>".cyan().bold()));

    match input.as_str() {
        "2" | "en" | "EN" | "english" => "en",
        _ => "ko", // default: 한국어
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Parse arguments
    let mut lang: Option<&str> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--lang" | "-lang" => {
                if i + 1 < args.len() {
                    lang = Some(match args[i + 1].as_str() {
                        "ko" | "kr" => "ko",
                        _ => "en",
                    });
                    i += 1;
                }
            }
            "--version" | "-v" => {
                println!("leaf-kit-tour v{}", VERSION);
                return;
            }
            "--help" | "-h" => {
                print_usage();
                return;
            }
            _ => {}
        }
        i += 1;
    }

    // If --lang not specified, ask interactively
    let lang = lang.unwrap_or_else(|| select_language());

    print_banner(lang);

    if !check_brew_installed() {
        if lang == "ko" {
            println!(
                "{} Homebrew가 설치되어 있지 않습니다.",
                "[!]".red().bold()
            );
            println!(
                "  먼저 Homebrew를 설치해주세요: {}",
                "https://brew.sh".underline()
            );
        } else {
            println!(
                "{} Homebrew is not installed.",
                "[!]".red().bold()
            );
            println!(
                "  Install Homebrew first: {}",
                "https://brew.sh".underline()
            );
        }
        std::process::exit(1);
    }

    if lang == "ko" {
        println!("{} Homebrew 감지됨\n", "[OK]".green().bold());
    } else {
        println!("{} Homebrew detected\n", "[OK]".green().bold());
    }

    // Self-update check
    check_self_update(lang);

    loop {
        print_tool_list(lang);
        print_menu(lang);

        let prompt_str = if lang == "ko" {
            format!("{} ", "선택>".cyan().bold())
        } else {
            format!("{} ", "select>".cyan().bold())
        };

        let input = prompt_input(&prompt_str);

        match input.as_str() {
            "q" | "Q" | "quit" | "exit" => {
                if lang == "ko" {
                    println!(
                        "\n{}",
                        "leaf-kit-tour를 이용해주셔서 감사합니다!".green()
                    );
                } else {
                    println!(
                        "\n{}",
                        "Thanks for using leaf-kit-tour!".green()
                    );
                }
                break;
            }
            "a" | "A" | "all" => {
                if lang == "ko" {
                    println!(
                        "\n{} 전체 {}개 도구를 설치합니다.",
                        ">>".bold(),
                        TOOLS.len()
                    );
                } else {
                    println!(
                        "\n{} Installing all {} tools.",
                        ">>".bold(),
                        TOOLS.len()
                    );
                }
                for tool in TOOLS {
                    install_tool(tool, lang);
                }
                if lang == "ko" {
                    println!("\n{}\n", "전체 설치 완료!".green().bold());
                } else {
                    println!("\n{}\n", "All tools installed!".green().bold());
                }
            }
            "r" | "R" | "reinstall" => {
                if lang == "ko" {
                    println!(
                        "\n재설치할 도구 번호를 입력하세요 (예: 1,3,5 또는 a=전체):"
                    );
                } else {
                    println!(
                        "\nEnter tool numbers to reinstall (e.g. 1,3,5 or a=all):"
                    );
                }
                let sel_prompt = if lang == "ko" {
                    format!("{} ", "재설치>".yellow().bold())
                } else {
                    format!("{} ", "reinstall>".yellow().bold())
                };
                let sel = prompt_input(&sel_prompt);
                if sel == "a" || sel == "A" {
                    for tool in TOOLS {
                        reinstall_tool(tool, lang);
                    }
                } else {
                    let indices = parse_selection(&sel);
                    if indices.is_empty() {
                        if lang == "ko" {
                            println!("{}", "[!] 올바른 번호를 입력하세요.".red());
                        } else {
                            println!("{}", "[!] Enter a valid number.".red());
                        }
                    } else {
                        for &idx in &indices {
                            reinstall_tool(&TOOLS[idx], lang);
                        }
                    }
                }
                println!();
            }
            "d" | "D" | "delete" | "uninstall" => {
                if lang == "ko" {
                    println!(
                        "\n삭제할 도구 번호를 입력하세요 (예: 1,3,5 또는 a=전체):"
                    );
                } else {
                    println!(
                        "\nEnter tool numbers to uninstall (e.g. 1,3,5 or a=all):"
                    );
                }
                let sel_prompt = if lang == "ko" {
                    format!("{} ", "삭제>".red().bold())
                } else {
                    format!("{} ", "uninstall>".red().bold())
                };
                let sel = prompt_input(&sel_prompt);
                if sel == "a" || sel == "A" {
                    for tool in TOOLS {
                        uninstall_tool(tool, lang);
                    }
                } else {
                    let indices = parse_selection(&sel);
                    if indices.is_empty() {
                        if lang == "ko" {
                            println!("{}", "[!] 올바른 번호를 입력하세요.".red());
                        } else {
                            println!("{}", "[!] Enter a valid number.".red());
                        }
                    } else {
                        for &idx in &indices {
                            uninstall_tool(&TOOLS[idx], lang);
                        }
                    }
                }
                println!();
            }
            "u" | "U" | "upgrade" => {
                if lang == "ko" {
                    println!(
                        "\n{} 설치된 도구를 전체 업그레이드합니다.",
                        ">>".bold()
                    );
                } else {
                    println!(
                        "\n{} Upgrading all installed tools.",
                        ">>".bold()
                    );
                }
                for tool in TOOLS {
                    upgrade_tool(tool, lang);
                }
                if lang == "ko" {
                    println!("\n{}\n", "전체 업그레이드 완료!".green().bold());
                } else {
                    println!("\n{}\n", "All upgrades complete!".green().bold());
                }
            }
            "i" | "I" | "info" | "detail" | "details" => {
                print_tool_details(lang);
            }
            "p" | "P" | "platform" => {
                print_platform_support(lang);
            }
            "s" | "S" | "status" => {
                if lang == "ko" {
                    println!("\n{} 설치 현황을 새로고침합니다...\n", "[*]".cyan().bold());
                } else {
                    println!("\n{} Refreshing install status...\n", "[*]".cyan().bold());
                }
                // Loop will re-print the list with updated status
            }
            _ => {
                let indices = parse_selection(&input);
                if indices.is_empty() {
                    if lang == "ko" {
                        println!(
                            "\n{} 올바른 옵션을 선택하세요. (번호, a=전체설치, r=재설치, u=업그레이드, d=삭제, i=상세정보, p=플랫폼, s=현황, q=종료)\n",
                            "[!]".yellow()
                        );
                    } else {
                        println!(
                            "\n{} Invalid option. (number, a=all, r=reinstall, u=upgrade, d=uninstall, i=details, p=platforms, s=status, q=quit)\n",
                            "[!]".yellow()
                        );
                    }
                } else {
                    if lang == "ko" {
                        println!(
                            "\n{} 선택한 {}개 도구를 설치합니다.",
                            ">>".bold(),
                            indices.len()
                        );
                    } else {
                        println!(
                            "\n{} Installing {} selected tool(s).",
                            ">>".bold(),
                            indices.len()
                        );
                    }
                    for &idx in &indices {
                        install_tool(&TOOLS[idx], lang);
                    }
                    if lang == "ko" {
                        println!("\n{}\n", "선택 설치 완료!".green().bold());
                    } else {
                        println!("\n{}\n", "Selected tools installed!".green().bold());
                    }
                }
            }
        }
    }
}
