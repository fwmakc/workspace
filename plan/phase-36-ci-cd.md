# Этап 36 — CI/CD Pipeline

## Цель
Создать пайплайн непрерывной интеграции и доставки: автоматическая сборка, тестирование, релиз. После этого этапа каждый коммит проверяется, каждый релиз собирается автоматически.

## Язык и стек
- **Язык:** YAML (CI конфигурация), Shell scripts, PowerShell, Python
- **Инфраструктура:** GitHub Actions / GitLab CI (или self-hosted runners)
- **Ключевые зависимости:** `cargo` (Rust builds), `bun` (TypeScript builds), `docker` (контейнеры для тестов)
- **Целевые ОС:** Windows, macOS, Linux, Android, iOS

## Зависимости
- **Все предыдущие этапы** (1–35).

## Часть системы
**Level 0–4 — Cross-cutting: DevOps** [См. layer-8 §18]

## Требования

### 36.1 Build Matrix
- **Платформы:** Windows (x64, ARM64), macOS (Intel, Apple Silicon), Linux (x64, ARM64), Android (ARM64, ARMv7, x86_64), iOS (ARM64).
- **Конфигурации:** Debug, Release.
- **Rust builds:** `cargo build --target <triple> --release`.
- **Bun builds:** `bun build` для TypeScript, `bun test` для unit tests.
- **Cross-compilation:** Android (NDK), iOS (Xcode toolchain).
- **Artifact:** бинарники, `.msi` (Windows), `.dmg` (macOS), `.AppImage` (Linux), `.apk` (Android), `.ipa` (iOS).

### 36.2 Test Pipeline
- **Unit Tests:** Rust (`cargo test`), TypeScript (`bun test`). Запускаются на каждом коммите.
- **Integration Tests:** сквозные тесты (Host Shim → Display Server → Micro-Kernel). Запускаются на PR.
- **Stress Tests:** этап 35. Запускаются nightly (1 раз в день).
- **Performance Regression:** сравнение с baseline. Fail если деградация > 10%.

### 36.3 Quality Gates
- **Lint:** `clippy` (Rust), `eslint` + `prettier` (TypeScript), `markdownlint` (docs).
- **Format check:** `rustfmt`, `prettier`.
- **Security scan:** `cargo audit` (Rust dependencies), `npm audit` (TypeScript dependencies), `bandit` (Python scripts).
- **License check:** FOSSA или `cargo-deny`. Все зависимости — MIT/Apache/BSD/compatible.

### 36.4 Release Pipeline
- **Versioning:** Semantic Versioning (`MAJOR.MINOR.PATCH`). Git tags (`v1.0.0`).
- **Changelog:** автогенерация из commit messages (Conventional Commits).
- **Signing:** бинарники подписываются Ed25519 (Key Manager). Проверка подписи при установке.
- **Distribution:**
  - GitHub Releases (assets).
  - P2P CDN (этап 20, verified seeding).
  - Website download.
- **Update mechanism:** in-app update check (сравнение версии с latest release). Скачивание delta-update (только изменённые файлы).

### 36.5 Monitoring
- **Build status dashboard:** зелёный/красный статус для каждой платформы.
- **Test coverage:** target > 80% для Rust, > 70% для TypeScript.
- **Flaky test detection:** если тест падает > 3 раза из 10 — помечается flaky и требует внимания.

## Ключевые функции

| Функция | Описание | Тест |
|---------|----------|------|
| Build all platforms | Сборка | Коммит → все 5 платформ собираются |
| Unit tests | Тесты | PR → unit tests проходят |
| Nightly stress | Стресс | Ночь → stress tests запускаются |
| Security audit | Безопасность | `cargo audit` → 0 уязвимостей |
| Signed release | Подпись | Бинарник подписан, подпись валидна |
| Auto update | Обновление | Старый билд → обнаруживает новый → обновляется |

## Интеграция с будущими этапами
- **Вход:** все предыдущие этапы.
- **Выход:** CI статус → этап 37 (Documentation).

## Критерии приёмки
- [ ] Build matrix: все 5 платформ собираются за < 30 минут (parallel).
- [ ] Unit tests: проходят на всех платформах.
- [ ] Nightly stress: запускаются автоматически, отчёт генерируется.
- [ ] Security audit: 0 critical vulnerabilities.
- [ ] Signed release: подпись валидна, проверка при установке.
- [ ] Auto update: delta-update < 10% размера полного билда.

## Ссылки
- [layer-8-technical-decomposition.md](../layers/layer-8-technical-decomposition.md) — CI/CD §18
