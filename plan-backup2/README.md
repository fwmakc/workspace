# План реализации CORE OS — 29 этапов

> Этот документ описывает пошаговую разработку CORE OS от фундамента до готовой системы. Каждый этап — самодостоятельная единица: после его завершения систему можно собрать и протестировать. Этапы следуют строго последовательно.

---

## Принципы разделения

1. **Один язык на этап.** Каждый этап реализуется только на одном языке (Rust или TypeScript/Bun).
2. **Одна часть системы на этап.** Бэк, фронт, библиотеки, платформенные реализации — всё раздельно.
3. **Сборка и тестирование после каждого этапа.** Каждый этап даёт работающий инкремент.
4. **Строгая последовательность.** Этап N зависит только от этапов 1..N-1. Нет обратных зависимостей.
5. **Нет кода в плане.** Только требования, архитектура, контракты, критерии приёмки.

---

## Сводка по этапам

### Rust-этапы (Host Shim + Display Server)

| Этап | Название | Язык | Что работает после завершения | Примерное время |
|------|----------|------|-------------------------------|-----------------|
| 1 | Host Shim: Windows | Rust | Окно, ввод, event loop на Windows | 3 недели |
| 2 | Host Shim: macOS | Rust | Окно, ввод, event loop на macOS | 2 недели |
| 3 | Host Shim: Linux | Rust | Окно, ввод, event loop на Linux desktop | 2 недели |
| 4 | Host Shim: Android | Rust | Окно, ввод, event loop на Android (Native Activity) | 3 недели |
| 5 | Host Shim: iOS | Rust | Окно, ввод, event loop на iOS (UIKit) | 3 недели |
| 6 | Host Shim: Audio | Rust | Захват и воспроизведение аудио на всех ОС | 2 недели |
| 7 | Host Shim: Storage | Rust | Файловый доступ, watcher, USB на всех ОС | 2 недели |
| 8 | Host Shim: Network | Rust | Сокеты, WireGuard, P2P transport на всех ОС | 3 недели |
| 9 | Display Server: Core | Rust | WebGPU инициализация, swapchain, базовый рендер | 3 недели |
| 10 | Display Server: 2D | Rust | Примитивы, текст (все алфавиты), текстуры | 3 недели |
| 11 | Display Server: Compositor & Game Mode | Rust | Scene graph, эффекты, Game Mode direct GPU | 4 недели |

### TypeScript/Bun-этапы (Micro-Kernel + Фронт + Бэк)

| Этап | Название | Язык | Что работает после завершения | Примерное время |
|------|----------|------|-------------------------------|-----------------|
| 12 | Micro-Kernel: Core & IPC | TS/Bun | Bun runtime, IPC bridge, SQLite schema | 3 недели |
| 13 | Micro-Kernel: Security Engine | TS/Bun | Capability Security, RBAC base, sandbox | 2 недели |
| 14 | Micro-Kernel: VFS | TS/Bun | Виртуальная ФС, CID, теги, Smart Folders | 3 недели |
| 15 | Command Bar Engine | TS/Bun | 8 режимов ввода, suggestions, hotkeys | 3 недели |
| 16 | Project Manager | TS/Bun | Проекты, Spaces, теги, layout, checkpoint | 3 недели |
| 17 | Window Manager | TS/Bun | Окна, Z-стек, snap, Alt+Tab, Static UI Overlay | 3 недели |
| 18 | CRDT Engine | TS/Bun | Causal Trees, LWW, oplog, локальная sync | 3 недели |
| 19 | P2P Mesh | TS/Bun | mDNS, libp2p, WireGuard, global sync, handoff | 4 недели |
| 20 | Backup Engine | TS/Bun | 3-2-1 backup, USB/S3/P2P, restore, recovery phrase | 3 недели |
| 21 | App Registry | TS/Bun | Установка, обновление, удаление, core.json, подписи | 2 недели |
| 22 | V8 Isolate Runtime | TS/Bun | Sandbox, @core/* API, permissions UI, checkpoint | 4 недели |
| 23 | Island Mode | TS/C++ | WebView embedding, CEF/WebKit, web sandbox | 3 недели |
| 24 | Messenger Engine | TS/Bun | P2P чат, группы, CRDT messages, offline delivery | 3 недели |
| 25 | Email & VoIP | TS/Bun | IMAP/SMTP, WebRTC calls через WireGuard | 3 недели |
| 26 | Voice Pipeline | TS/Rust | Whisper ASR, TTS, Zero UI, Intent Queue | 3 недели |
| 27 | Intent API & AI Core | TS/Bun | Intent parser, Generative UI, Smart Scheduler, Cloud Bridge | 4 недели |
| 28 | Security Core | TS/Bun | RBAC full, Audit (13 кат.), Key Manager, Session, remote wipe | 3 недели |
| 29 | Admin UI & System Polish | TS/Rust | Backoffice, Hardcore, Game Mode API, Energy, Accessibility, Themes, Docs | 4 недели |

**Итого:** ~80 недель (20 месяцев) на полноценную систему командой 5–7 человек.

---

## Технологический стек (константа для всех этапов)

| Компонент | Язык / Runtime | Назначение |
|-----------|----------------|------------|
| Host Shim | Rust (winit, wgpu, CPAL, Oboe, AVAudioEngine) | Окно, ввод, GPU, аудио, файлы, сеть |
| Display Server | Rust (wgpu, WGSL) | WebGPU рендеринг, композитинг, эффекты |
| Micro-Kernel | Bun (TypeScript) + V8 | IPC, SQLite, Capability Security, Runtime |
| App Runtime | V8 Isolates (via Bun) | Приложения, sandbox, @core/* API |
| Island Mode | CEF / WebKit / WebView2 / WKWebView | Веб-контент, legacy apps |
| AI Engine | Bun + ONNX / Ollama | Whisper, SLM, TTS, embeddings |
| P2P / Sync | Bun + Rust (WireGuard) | Mesh, CRDT, анонсирование |
| Storage | SQLite (Bun built-in) | Данные, индексы, настройки, audit |

---

## Формат файла этапа

Каждый файл `phase-NN-*.md` содержит:

1. **Цель** — что должно работать после этапа
2. **Язык и стек** — на чём пишем, ключевые зависимости, целевые ОС
3. **Зависимости** — от каких предыдущих этапов зависит
4. **Часть системы** — какой уровень архитектуры, ссылки на слои
5. **Требования** — детальное описание функциональности
6. **Ключевые функции** — таблица: функция, описание, тест
7. **Интеграция** — входы/выходы с другими этапами
8. **Критерии приёмки** — как проверить, что этап готов
9. **Ссылки** — cross-reference на `layers/`

---

## Зависимости между этапами (кратко)

```
1 → 2 → 3 → 4 → 5 → 6 → 7 → 8 → 9 → 10 → 11
                                                  ↓
12 → 13 → 14 → 15 → 16 → 17
   ↓               ↓       ↓       ↓
  18 → 19 → 20   21 → 22 → 23
   ↓       ↓               ↓
  24 → 25               26 → 27
                            ↓
                            28 → 29
```

Все этапы Rust (1–11) не зависят от TS-этапов (12–29) и могут разрабатываться параллельно в начале. Однако в плане они упорядочены последовательно: каждый следующий этап использует результаты предыдущего как reference или shared abstraction.

---

## Cross-reference со слоями

Каждый этап явно ссылается на файлы из `layers/` через относительные ссылки (`../layers/layer-N-*.md`). Для быстрого поиска:

- **layer-1** (UX) → этапы 11, 15, 16, 17, 24, 25, 26, 29
- **layer-2** (AI) → этапы 26, 27
- **layer-3** (System Split) → этапы 12, 16, 17, 19, 24, 28, 29
- **layer-4** (Installation) → этапы 1, 2, 3, 4, 5, 20, 29
- **layer-5** (Devices) → этапы 7, 8, 14, 18, 19
- **layer-6** (Apps) → этапы 13, 21, 22, 23
- **layer-7** (Security) → этапы 13, 20, 25, 28, 29
- **layer-8** (Technical Decomposition) → все этапы
- **layer-9** (Hardware) → этапы 1–11, 29
- **layer-10** (Business) → этап 29 (docs)
- **layer-11** (Developer Reference) → этапы 15, 21, 22, 27, 29
