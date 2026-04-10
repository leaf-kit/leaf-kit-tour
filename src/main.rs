use colored::Colorize;
use std::io::{self, Write};
use std::process::Command;

struct Tool {
    name: &'static str,
    tap: &'static str,
    formula: &'static str,
    description: &'static str,
    when_to_use: &'static str,
    repo: &'static str,
}

const TOOLS: &[Tool] = &[
    Tool {
        name: "stylemd",
        tap: "leaf-kit/stylemd",
        formula: "stylemd",
        description: "올인원 마크다운 툴킷 — lint, format, fix, analyze를 하나의 빠른 CLI로",
        when_to_use: "마크다운 문서의 스타일을 검사하고, 포맷팅하고, 자동 수정할 때 사용합니다. CI/CD 파이프라인에 통합하여 문서 품질을 유지하는 데 적합합니다.",
        repo: "https://github.com/leaf-kit/style.md",
    },
    Tool {
        name: "playgraph",
        tap: "leaf-kit/playgraph",
        formula: "playgraph",
        description: "세계 최초의 애니메이션 마크다운 뷰어 — 텍스트, 그래프, 다이어그램을 플레이",
        when_to_use: "마크다운 문서를 단순히 읽는 것을 넘어, 그래프와 다이어그램을 애니메이션으로 시각화하고 싶을 때 사용합니다. 프레젠테이션이나 교육 자료에 적합합니다.",
        repo: "https://github.com/leaf-kit/playgraph.md",
    },
    Tool {
        name: "lsmd",
        tap: "leaf-kit/lsmd",
        formula: "lsmd",
        description: "마크다운 인식 디렉토리 목록 도구 — ls 명령의 마크다운 특화 버전",
        when_to_use: "마크다운 파일이 많은 프로젝트에서 파일 목록을 구조적으로 확인하고 관리할 때 사용합니다. 문서 저장소를 탐색할 때 편리합니다.",
        repo: "https://github.com/leaf-kit/ls.md",
    },
    Tool {
        name: "gmd",
        tap: "leaf-kit/gmd",
        formula: "gmd",
        description: "Grep Markdown — 마크다운 구조 인식 고속 검색 및 분석 도구",
        when_to_use: "마크다운 문서 내에서 헤딩, 코드 블록, 링크 등 구조를 인식하며 검색할 때 사용합니다. 일반 grep으로는 찾기 어려운 마크다운 요소를 정확히 찾아줍니다.",
        repo: "https://github.com/leaf-kit/g.md",
    },
    Tool {
        name: "bark",
        tap: "leaf-kit/bark",
        formula: "bark",
        description: "터미널 마크다운 뷰어 — Browse And Render Markdown, Keenly",
        when_to_use: "터미널에서 마크다운 문서를 깔끔하게 렌더링하여 읽고 싶을 때 사용합니다. SSH 환경이나 GUI 없는 서버에서 문서를 확인할 때 유용합니다.",
        repo: "https://github.com/leaf-kit/bark.md",
    },
];

fn print_banner() {
    println!();
    println!(
        "{}",
        "══════════════════════════════════════════════════════════════"
            .green()
    );
    println!(
        "{}",
        "            Leaf Kit Repo Tour  —  CLI Installer             "
            .green()
            .bold()
    );
    println!(
        "{}",
        "══════════════════════════════════════════════════════════════"
            .green()
    );
    println!();
    println!(
        "{}",
        "leaf-kit의 마크다운 툴킷 CLI 도구를 Homebrew로 간편하게 설치합니다."
            .bright_white()
    );
    println!();
    println!("{}", "[플랫폼 안내]".yellow().bold());
    println!(
        "  현재 {}을 기본으로 지원하며, Linux 및 Windows는 확장 예정입니다.",
        "macOS".cyan().bold()
    );
    println!(
        "  자세한 내용: {}",
        "https://github.com/leaf-kit/leaf-kit-tour".underline()
    );
    println!();
}

fn print_tool_list() {
    println!(
        "{}",
        "──────────────────────────────────────────────────────────────"
            .bright_black()
    );
    println!("{}", "  #  도구명        설명".bold());
    println!(
        "{}",
        "──────────────────────────────────────────────────────────────"
            .bright_black()
    );

    for (i, tool) in TOOLS.iter().enumerate() {
        println!(
            "  {}  {:<12}  {}",
            format!("{}", i + 1).cyan().bold(),
            tool.name.green().bold(),
            tool.description
        );
        println!(
            "     {}            활용: {}",
            "",
            tool.when_to_use.bright_black()
        );
        println!(
            "     {}            설치: brew install {}/{}",
            "",
            tool.tap,
            tool.formula
        );
        println!(
            "     {}            재설치: brew reinstall {}/{}",
            "",
            tool.tap,
            tool.formula
        );
        println!(
            "     {}            저장소: {}",
            "",
            tool.repo.underline().bright_black()
        );
        if i < TOOLS.len() - 1 {
            println!(
                "{}",
                "  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -"
                    .bright_black()
            );
        }
    }

    println!(
        "{}",
        "──────────────────────────────────────────────────────────────"
            .bright_black()
    );
    println!();
}

fn print_menu() {
    println!("{}", "[설치 옵션]".yellow().bold());
    println!("  {}  전체 설치 (모든 CLI 도구)", "a".cyan().bold());
    println!(
        "  {}  번호로 선택 설치 (예: 1,3,5 또는 1-3)",
        "번호".cyan().bold()
    );
    println!("  {}  재설치 모드", "r".cyan().bold());
    println!("  {}  종료", "q".cyan().bold());
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

fn install_tool(tool: &Tool) {
    println!(
        "\n{} {} 설치 중...",
        ">>".green().bold(),
        tool.name.green().bold()
    );

    // tap 추가
    println!("  -> brew tap {}", tool.tap);
    let tap_result = Command::new("brew").args(["tap", tool.tap]).status();

    match tap_result {
        Ok(status) if status.success() => {}
        _ => {
            println!(
                "  [X] tap 추가 실패: brew tap {}",
                tool.tap
            );
            return;
        }
    }

    // install
    let formula = format!("{}/{}", tool.tap, tool.formula);
    println!("  -> brew install {}", formula);
    let install_result = Command::new("brew").args(["install", &formula]).status();

    match install_result {
        Ok(status) if status.success() => {
            println!(
                "  {} {} 설치 완료!",
                "[OK]".green().bold(),
                tool.name.green().bold()
            );
        }
        _ => {
            println!(
                "  {} {} 설치 실패. 수동으로 시도해보세요: brew install {}",
                "[FAIL]".red().bold(),
                tool.name,
                formula
            );
        }
    }
}

fn reinstall_tool(tool: &Tool) {
    let formula = format!("{}/{}", tool.tap, tool.formula);
    println!(
        "\n{} {} 재설치 중...",
        ">>".yellow().bold(),
        tool.name.yellow().bold()
    );
    println!("  -> brew reinstall {}", formula);

    let result = Command::new("brew").args(["reinstall", &formula]).status();

    match result {
        Ok(status) if status.success() => {
            println!(
                "  {} {} 재설치 완료!",
                "[OK]".green().bold(),
                tool.name.green().bold()
            );
        }
        _ => {
            println!(
                "  {} {} 재설치 실패.",
                "[FAIL]".red().bold(),
                tool.name
            );
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

fn main() {
    print_banner();

    if !check_brew_installed() {
        println!(
            "{} Homebrew가 설치되어 있지 않습니다.",
            "[!]".red().bold()
        );
        println!(
            "  먼저 Homebrew를 설치해주세요: {}",
            "https://brew.sh".underline()
        );
        std::process::exit(1);
    }

    println!("{} Homebrew 감지됨\n", "[OK]".green().bold());

    loop {
        print_tool_list();
        print_menu();

        let input = prompt_input(&format!("{} ", "선택>".cyan().bold()));

        match input.as_str() {
            "q" | "Q" | "quit" | "exit" => {
                println!(
                    "\n{}",
                    "Leaf Kit Repo Tour를 이용해주셔서 감사합니다!".green()
                );
                break;
            }
            "a" | "A" | "all" => {
                println!(
                    "\n{} 전체 {}개 도구를 설치합니다.",
                    ">>".bold(),
                    TOOLS.len()
                );
                for tool in TOOLS {
                    install_tool(tool);
                }
                println!("\n{}\n", "전체 설치 완료!".green().bold());
            }
            "r" | "R" | "reinstall" => {
                println!(
                    "\n재설치할 도구 번호를 입력하세요 (예: 1,3,5 또는 a=전체):"
                );
                let sel = prompt_input(&format!("{} ", "재설치>".yellow().bold()));
                if sel == "a" || sel == "A" {
                    for tool in TOOLS {
                        reinstall_tool(tool);
                    }
                } else {
                    let indices = parse_selection(&sel);
                    if indices.is_empty() {
                        println!("{}", "[!] 올바른 번호를 입력하세요.".red());
                    } else {
                        for &i in &indices {
                            reinstall_tool(&TOOLS[i]);
                        }
                    }
                }
                println!();
            }
            _ => {
                let indices = parse_selection(&input);
                if indices.is_empty() {
                    println!(
                        "\n{} 올바른 옵션을 선택하세요. (번호, a=전체설치, r=재설치, q=종료)\n",
                        "[!]".yellow()
                    );
                } else {
                    println!(
                        "\n{} 선택한 {}개 도구를 설치합니다.",
                        ">>".bold(),
                        indices.len()
                    );
                    for &i in &indices {
                        install_tool(&TOOLS[i]);
                    }
                    println!("\n{}\n", "선택 설치 완료!".green().bold());
                }
            }
        }
    }
}
