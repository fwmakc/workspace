# Этап 29 — Admin UI & System Polish

## Цель
Завершить систему: создать интерфейсы администрирования (Core.Backoffice GUI, Core.Hardcore TUI/CLI), системные режимы (Game Mode, Energy Manager, Accessibility, Themes), и финальную полировку. После этого этапа CORE OS — полнофункциональная операционная система, готовая к использованию.

## Язык и стек
- **Язык:** TypeScript (Backoffice GUI, Hardcore CLI), Rust (Hardcore TUI — ratatui)
- **Runtime:** Bun (GUI/CLI), native Rust (TUI)
- **Ключевые зависимости:** `ratatui` (TUI), `clap` (CLI parser), `@core/ui` (Backoffice components)
- **Целевые ОС:** Windows, macOS, Linux, Android

## Зависимости
- **Этап 11** — Display Server: Compositor (Game Mode direct surface, chrome).
- **Этап 17** — Window Manager (window states, fullscreen).
- **Этап 28** — Security Core (RBAC, Audit, Key Manager, remote wipe).
- **Все предыдущие этапы** (1–26).

## Часть системы
**Level 1 — Бэк/Фронт: Admin & System Modes** [См. layer-3 §3, layer-8 §16, layer-1 §4.5–4.9, layer-9]

## Требования

### 27.1 Core.Backoffice (GUI)
- **Назначение:** графический интерфейс администрирования для Owner и администраторов.
- **Доступ:** через Command Bar (`> backoffice`) или ярлык.
- **Разделы:**
  - **Users:** список пользователей, создание, удаление, назначение ролей.
  - **Spaces:** управление Spaces, перенос проектов между ними.
  - **Apps:** установленные приложения, обновления, permissions.
  - **Security:** RBAC редактор, audit log viewer (фильтры, export), Key Manager status.
  - **Backup:** запуск backup, restore, просмотр истории backup'ов.
  - **AI:** настройки SLM, Cloud Bridge, voice models.
  - **Support:** remote support session (подключение техподдержки через relay).
- **Рендеринг:** через `@core/ui` (этап 20) → Display Server.
- **Корпоративный режим:** если `allow_gui_admin: false` — Core.Backoffice не устанавливается, единственный способ администрирования — Core.Hardcore [См. layer-3 §3.2].

### 27.2 Core.Hardcore (TUI + CLI)
- **Назначение:** администрирование без GUI (для серверов, embedded, корп. режима).
- **SSH:** russh сервер, прослушивающий порт (по умолчанию 2222, не конфликтует с системным 22).
- **TUI:** `ratatui` или custom WebGPU TUI. Экраны:
  - Users (список, редактирование).
  - Security (audit viewer с пагинацией).
  - Backup (запуск, статус).
  - AI (модели, статус).
  - System (CPU, RAM, network stats).
- **CLI команды:**
  - `core-cli user add --name "Иван" --role developer`
  - `core-cli user list`
  - `core-cli role create --name "developer" --capabilities "fs.read,fs.write"`
  - `core-cli backup --full --target usb`
  - `core-cli restore --target usb --date 2025-01-15`
  - `core-cli audit query --category "auth" --from "2025-01-01"`
  - `core-cli settings set --key "security.level" --value "enhanced"`
  - `core-cli ai model set --asr "whisper-medium"`
  - `core-cli remote-wipe --device-id "..."` (требует recovery phrase).

### 27.3 Game Mode (System API)
- **API для приложений:** `core.game.requestMode()` — запрос перехода в Game Mode.
- **Политики:**
  - Game Mode доступен только для приложений level 5 (полный натив) и level 4 (с `@core/graphics`).
  - User может запретить Game Mode для конкретного приложения (чёрный список).
- **Energy в Game Mode:** отключение background sync, pause backup, minimum notifications.
- **Performance overlay:** опциональный FPS counter, GPU load, temperature (для power users).

### 27.4 Energy Manager
- **Battery monitoring:** Host Shim предоставляет API для уровня заряда (через OS APIs).
- **Политики по уровню заряда:**
  - 100–50%: нормальный режим.
  - 50–20%: power save (30 FPS, отключение blur/shadows, pause background sync).
  - 20–10%: critical (сохранение checkpoint'ов, закрытие background apps, остановка P2P, минимальная яркость).
  - 10–0%: emergency (graceful shutdown с сохранением всех данных).
- **Plug detection:** при подключении зарядки — возврат к нормальному режиму.
- **TTS feedback:** "Включён режим энергосбережения" (этап 24).

### 27.5 Accessibility
- **High Contrast:** пост-обработка в Display Server (инверсия или повышение контраста через shader).
- **Large Text:** глобальный масштаб шрифта (1.25x, 1.5x, 2x).
- **Screen Reader:** Display Server предоставляет API для получения текстовых элементов в текущем layout (bounding box + text + role). Screen reader (сторонний или встроенный) использует этот API.
- **Reduced Motion:** отключение анимаций (tween duration = 0).
- **Color Blindness:** цветовые фильтры (Deuteranopia, Protanopia, Tritanopia) через post-processing shader.
- **Keyboard Navigation:** Tab/Shift+Tab для фокуса, Enter/Space для активации, Esc для закрытия.

### 27.6 Themes
- **Theme engine:** JSON-файл с цветовой палитрой, шрифтами, размерами, радиусами.
- **Встроенные темы:** Light, Dark, High Contrast (связано с Accessibility).
- **Кастомные темы:** пользователь может создать свою тему через Settings.
- **System theme sync:** следование системной теме хост-ОС (Windows/macOS theme).
- **Hot reload:** изменение темы применяется мгновенно (без перезагрузки).

### 27.7 Migration Tools
- **Export:** `core-cli backup --export` или GUI wizard. Выбор: все данные, только проекты, только настройки.
- **Import:** `core-cli restore --import`. Валидация формата, версионность.
- **Cross-platform:** backup с Windows можно восстановить на macOS (blob'ы + SQLite).

### 27.8 Performance & Stress Tests
- **Stress tests:**
  - 1000 окон: открытие, переключение, закрытие. Цель: > 30 FPS.
  - 1000 документов CRDT: синхронизация. Цель: < 1 сек.
  - RAM 95%: поведение системы (graceful degradation, checkpoint, kill background apps).
  - Network 500 мс latency: P2P sync stability.
  - Battery critical: graceful shutdown.
- **Performance budget:**
  - Frame time < 16.67 мс (60 FPS) при 10 окнах.
  - Input latency < 8 мс.
  - Cold boot < 5 сек (этапы 1–10 загружены).
  - Warm boot < 2 сек (restore from checkpoint).

### 27.9 Documentation
- **User docs:** getting started, projects, voice control, security, troubleshooting.
- **Developer docs:** architecture, app model, Intent API, P2P protocol, `@core/*` API reference.
- **Admin docs:** installation, Backoffice, Hardcore, security hardening.
- **API reference:** Intent API, app manifest, P2P RPC.

## Ключевые функции

| Функция | Описание | Тест |
|---------|----------|------|
| Backoffice GUI | Админка | Открыть → список пользователей, audit log |
| Hardcore SSH | Удалённый доступ | SSH localhost:2222 → core-cli работает |
| Hardcore TUI | Интерфейс | Запустить → меню, навигация стрелками |
| Game Mode API | Запрос | Приложение запрашивает → вход < 33 мс |
| Energy save | Энергосбережение | 20% батареи → power save активен |
| High contrast | Доступность | Включить → цвета инвертированы |
| Theme switch | Темы | Переключить Light/Dark → мгновенно |
| Migration | Перенос | Экспорт → импорт → данные на месте |
| Stress 1000 wins | Нагрузка | 1000 окон → система отзывчива |

## Интеграция с будущими этапами
- **Вход:** этап 9 (Compositor) — Game Mode direct surface, post-processing (Accessibility).
- **Вход:** этап 15 (Window Manager) — window states, fullscreen.
- **Вход:** этап 26 (Security) — RBAC, Audit, Key Manager, remote wipe.
- **Вход:** этап 24 (Voice) — TTS feedback для Energy Manager.
- **Выход:** Backoffice/Hardcore → управление всеми предыдущими этапами.
- **Выход:** Theme → этап 9 (Display Server shaders, colors).
- **Выход:** Accessibility → этап 9 (post-processing, font scale).

## Критерии приёмки
- [ ] Backoffice GUI: все 7 разделов работают, рендерятся через `@core/ui`.
- [ ] Hardcore SSH: подключение, аутентификация, core-cli команды.
- [ ] Hardcore TUI: навигация, выполнение команд.
- [ ] Game Mode API: вход < 33 мс, выход (Panic Gesture) мгновенно.
- [ ] Energy Manager: 3 режима (normal, power save, critical), автопереключение.
- [ ] Accessibility: high contrast, large text, reduced motion, keyboard nav.
- [ ] Themes: Light/Dark/Custom, hot reload, system sync.
- [ ] Migration: export → import → данные идентичны.
- [ ] Stress tests: 1000 окон (> 30 FPS), 1000 docs CRDT (< 1 сек sync).
- [ ] Docs: user, developer, admin, API reference — полнота и корректность.

## Ссылки
- [layer-1-user-experience.md](../layers/layer-1-user-experience.md) — Game Mode, Energy, Accessibility, Themes
- [layer-3-system-split.md](../layers/layer-3-system-split.md) — Core.Backoffice, Core.Hardcore, корп. режим
- [layer-7-security.md](../layers/layer-7-security.md) — Remote wipe, Secure Transaction
- [layer-8-technical-decomposition.md](../layers/layer-8-technical-decomposition.md) — Admin §16, Game Mode §3.6, Energy §4.10
- [layer-9-hardware-requirements.md](../layers/layer-9-hardware-requirements.md) — Performance targets
