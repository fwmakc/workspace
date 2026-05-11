# Корпус тестов CORE OS

> **37 этапов = 37 отдельных тестовых документов.** Ни одна функция, ни один критерий приёмки, ни один сценарий из `plan/phase-NN-*.md` не остаются без теста. Минимум 1000 тест-кейсов.

---

## Философия

### Правило №1: один этап — один файл
Каждый `plan/phase-NN-*.md` имеет ровно один `tests/phase-NN-*.md`. Нет склеивания. Нет "хост-шимом всё в одну кучу".

### Правило №2: мок — позор
Если тест можно провести с реальным компонентом — используем реальный. Мок допустим только для:
- Внешних облачных API (OpenAI, Gmail IMAP) — но есть дублирующий интеграционный тест с sandbox-аккаунтом.
- Аппаратуры, которой нет в CI (TPM 2.0, Secure Enclave, AirPods) — но есть дублирующий тест на реальном железе в лабе.
- Не детерминированных вещей — но детерминизируем через seed.

Каждый тест с моком обязан иметь тест-близнец без мока:
```
TC-15-007-MOCK: Input Router с моком Host Shim
TC-15-007-REAL: Input Router с реальным Host Shim (ручной/лаб)
```

### Правило №3: реальные данные
- Текстовые документы — реальные заметки пользователей (анонимизированные).
- Изображения — реальные PNG/JPEG разных размеров, включая 4K HDR.
- Аудио — реальные записи голоса 50+ спикеров, мужские/женские, 10 языков.
- Сеть — реальные UDP/TCP пакеты, WireGuard handshake между физическими машинами.
- CRDT — реальные документы из `layers/`.

### Правило №4: каждая функция — тест
Если в плане есть таблица «Ключевые функции» — для каждой строки минимум 2 теста: positive + negative.

### Правило №5: каждый критерий приёмки — тест
Чеклист критериев приёмки = автоматизированный тест.

### Правило №6: последовательность = зависимость
Тест этапа N не запускается без PASS всех тестов этапов 1..N-1.

---

## Структура (37 файлов)

| Файл | Этап | Тест-кейсов |
|------|------|-------------|
| `phase-01-host-shim-windows.md` | 1 | 20 |
| `phase-02-host-shim-macos.md` | 2 | 18 |
| `phase-03-host-shim-linux.md` | 3 | 18 |
| `phase-04-host-shim-android.md` | 4 | 22 |
| `phase-05-host-shim-ios.md` | 5 | 20 |
| `phase-06-host-shim-audio.md` | 6 | 22 |
| `phase-07-host-shim-storage.md` | 7 | 20 |
| `phase-08-host-shim-network.md` | 8 | 22 |
| `phase-09-display-server-core.md` | 9 | 20 |
| `phase-10-display-server-2d-text.md` | 10 | 24 |
| `phase-11-display-server-compositor.md` | 11 | 24 |
| `phase-12-micro-kernel-core-ipc.md` | 12 | 24 |
| `phase-13-micro-kernel-security.md` | 13 | 24 |
| `phase-14-micro-kernel-vfs.md` | 14 | 24 |
| `phase-15-command-bar-engine.md` | 15 | 24 |
| `phase-16-command-bar-ui.md` | 16 | 20 |
| `phase-17-project-manager.md` | 17 | 24 |
| `phase-18-window-manager.md` | 18 | 22 |
| `phase-19-crdt-engine.md` | 19 | 26 |
| `phase-20-p2p-mesh.md` | 20 | 26 |
| `phase-21-backup-engine.md` | 21 | 22 |
| `phase-22-app-registry.md` | 22 | 20 |
| `phase-23-v8-isolate-runtime.md` | 23 | 26 |
| `phase-24-island-mode.md` | 24 | 20 |
| `phase-25-messenger.md` | 25 | 22 |
| `phase-26-email.md` | 26 | 20 |
| `phase-27-voip.md` | 27 | 20 |
| `phase-28-voice-pipeline.md` | 28 | 24 |
| `phase-29-intent-api-ai-core.md` | 29 | 24 |
| `phase-30-security-core.md` | 30 | 28 |
| `phase-31-admin-ui.md` | 31 | 20 |
| `phase-32-game-mode-energy.md` | 32 | 18 |
| `phase-33-accessibility-themes.md` | 33 | 20 |
| `phase-34-performance.md` | 34 | 18 |
| `phase-35-stress-tests.md` | 35 | 14 |
| `phase-36-ci-cd.md` | 36 | 16 |
| `phase-37-documentation.md` | 37 | 12 |
| **Итого** | **37** | **788** |

---

## Формат тест-кейса

```markdown
### TC-NN-XXX: Название
**Тип:** E2E | Integration | Unit | Stress | Performance
**Платформа:** Windows | macOS | Linux | Android | iOS | All
**Данные:** Реальные | Мок (дублируется TC-NN-XXX-REAL)
**Требование:** plan/phase-NN-*.md, таблица X, строка Y
**Приоритет:** P0 | P1 | P2
**Предусловия:**
**Шаги:**
1. ...
2. ...
**Ожидаемый результат:**
- ...
**Критерий приёмки:** PASS / FAIL
**Автоматизация:** автоматический | полуавтоматический | ручной
**Заметки:**
```

---

## Инфраструктура

### Test Lab

| Устройство | ОС | Роль |
|------------|-----|------|
| Desktop PC i9/RTX4090 | Windows 11 | Primary + GPU |
| Mac Studio M2 Ultra | macOS | Apple Silicon |
| Intel NUC | Ubuntu 24.04 | Linux primary |
| Pixel 8 Pro | Android 14 | Mobile |
| iPhone 15 Pro | iOS 17 | Mobile |
| Surface Pro X | Windows ARM64 | ARM |
| Raspberry Pi 5 | Debian ARM64 | Embedded |

### CI
- **Per-PR:** Unit + Integration (VM, ~500 тестов, < 15 мин)
- **Nightly:** E2E на физических устройствах (~200 тестов)
- **Weekly:** Stress tests (~60 тестов)
- **Per-release:** Полный regression (все 820+ тестов)

### Coverage
| Уровень | Целевое | Минимум |
|---------|---------|---------|
| Unit (Rust) | > 90% | > 80% |
| Unit (TypeScript) | > 85% | > 75% |
| Integration | > 85% | > 75% |
| E2E | > 80% | > 70% |
| Stress | 100% | 100% |

---

## Статус

| Этап | Название | Тестов | Авто | Ручное | Статус |
|------|----------|--------|------|--------|--------|
| 1 | Host Shim Windows | 20 | 17 | 3 | 🔲 |
| 2 | Host Shim macOS | 18 | 15 | 3 | 🔲 |
| 3 | Host Shim Linux | 18 | 15 | 3 | 🔲 |
| 4 | Host Shim Android | 22 | 17 | 5 | 🔲 |
| 5 | Host Shim iOS | 20 | 15 | 5 | 🔲 |
| 6 | Host Shim Audio | 22 | 20 | 2 | 🔲 |
| 7 | Host Shim Storage | 20 | 17 | 3 | 🔲 |
| 8 | Host Shim Network | 22 | 19 | 3 | 🔲 |
| 9 | Display Server Core | 20 | 18 | 2 | 🔲 |
| 10 | Display Server 2D | 24 | 21 | 3 | 🔲 |
| 11 | Display Server Compositor | 24 | 21 | 3 | 🔲 |
| 12 | Micro-Kernel Core | 24 | 22 | 2 | 🔲 |
| 13 | Micro-Kernel Security | 24 | 22 | 2 | 🔲 |
| 14 | Micro-Kernel VFS | 24 | 21 | 3 | 🔲 |
| 15 | Command Bar Engine | 24 | 22 | 2 | 🔲 |
| 16 | Command Bar UI | 20 | 18 | 2 | 🔲 |
| 17 | Project Manager | 24 | 21 | 3 | 🔲 |
| 18 | Window Manager | 22 | 20 | 2 | 🔲 |
| 19 | CRDT Engine | 26 | 24 | 2 | 🔲 |
| 20 | P2P Mesh | 26 | 22 | 4 | 🔲 |
| 21 | Backup Engine | 22 | 19 | 3 | 🔲 |
| 22 | App Registry | 20 | 18 | 2 | 🔲 |
| 23 | V8 Isolate Runtime | 26 | 23 | 3 | 🔲 |
| 24 | Island Mode | 20 | 18 | 2 | 🔲 |
| 25 | Messenger | 22 | 19 | 3 | 🔲 |
| 26 | Email | 20 | 17 | 3 | 🔲 |
| 27 | VoIP | 20 | 18 | 2 | 🔲 |
| 28 | Voice Pipeline | 24 | 21 | 3 | 🔲 |
| 29 | Intent API & AI Core | 24 | 20 | 4 | 🔲 |
| 30 | Security Core | 28 | 25 | 3 | 🔲 |
| 31 | Admin UI | 20 | 17 | 3 | 🔲 |
| 32 | Game Mode & Energy | 18 | 16 | 2 | 🔲 |
| 33 | Accessibility & Themes | 20 | 18 | 2 | 🔲 |
| 34 | Performance | 18 | 18 | 0 | 🔲 |
| 35 | Stress Tests | 14 | 14 | 0 | 🔲 |
| 36 | CI/CD | 16 | 16 | 0 | 🔲 |
| 37 | Documentation | 12 | 8 | 4 | 🔲 |
| **Итого** | | **788** | **686** | **102** | |

🔲 Не запускались | ⏳ В процессе | ✅ PASS | ❌ FAIL
