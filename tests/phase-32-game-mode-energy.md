# Тесты: Этап 32 — Game Mode & Energy Manager

> Game Mode API, direct surface, switch latency, performance policy, Alt+Tab overlay, Energy Manager thresholds, TTS warning, battery modes. Все тесты на реальном железе.

---

### TC-32-001: Game Mode — enter
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. `core.gameMode.enter()`.
**Ожидаемый результат:**
- Game Mode active. CPU priority: realtime.
**Автоматизация:** автоматический.

### TC-32-002: Game Mode — exit
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. `core.gameMode.exit()`.
**Ожидаемый результат:**
- Normal mode. Priorities restored.
**Автоматизация:** автоматический.

### TC-32-003: Game Mode — switch < 33ms
**Тип:** Performance | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Замер: enter() → first frame.
**Ожидаемый результат:**
- < 33 мс.
**Автоматизация:** автоматический.

### TC-32-004: Game Mode — direct surface
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Enter Game Mode. 2. Draw.
**Ожидаемый результат:**
- Direct scanout. Compositor skips.
**Автоматизация:** автоматический.

### TC-32-005: Game Mode — Alt+Tab overlay
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Game Mode. 2. Alt+Tab.
**Ожидаемый результат:**
- Overlay поверх. Game не закрыт.
**Автоматизация:** автоматический.

### TC-32-006: Game Mode — notification muted
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Game Mode. 2. Incoming message.
**Ожидаемый результат:**
- No popup. Silent notification.
**Автоматизация:** автоматический.

### TC-32-007: Game Mode — background apps throttled
**Тип:** Performance | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P1
**Шаги:**
1. Game Mode. 2. 10 background apps.
**Ожидаемый результат:**
- Background: below normal. Game: realtime.
**Автоматизация:** автоматический.

### TC-32-008: Game Mode — FPS uncapped
**Тип:** Performance | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P1
**Шаги:**
1. Game Mode. 2. Check vsync.
**Ожидаемый результат:**
- VSync off (if requested). FPS > 60.
**Автоматизация:** автоматический.

### TC-32-009: Game Mode — fullscreen detection
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P1
**Шаги:**
1. App goes fullscreen. 2. Auto-enter Game Mode.
**Ожидаемый результат:**
- Game Mode auto-enabled.
**Автоматизация:** автоматический.

### TC-32-010: Energy Manager — battery > 20%
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Battery 50%. 2. Check policy.
**Ожидаемый результат:**
- Normal mode. No restrictions.
**Автоматизация:** автоматический.

### TC-32-011: Energy Manager — battery < 20%
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Battery 15%.
**Ожидаемый результат:**
- FPS capped to 30. Background sync paused.
**Автоматизация:** автоматический (mock battery).

### TC-32-012: Energy Manager — battery < 10%
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Battery 8%.
**Ожидаемый результат:**
- Display brightness reduced. TTS warning.
**Автоматизация:** автоматический.

### TC-32-013: Energy Manager — TTS warning
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Battery < 10%.
**Ожидаемый результат:**
- TTS: "Low battery. Switching to power save mode."
**Автоматизация:** автоматический.

### TC-32-014: Energy Manager — plugged in
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Plug in charger.
**Ожидаемый результат:**
- Normal mode restored.
**Автоматизация:** автоматический.

### TC-32-015: Energy Manager — thermal throttling
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P1
**Шаги:**
1. CPU temp > 90°C.
**Ожидаемый результат:**
- Throttle. FPS capped.
**Автоматизация:** автоматический (mock temp).

### TC-32-016: Energy Manager — hibernate on critical
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P1
**Шаги:**
1. Battery 3%.
**Ожидаемый результат:**
- Hibernate. Data saved.
**Автоматизация:** автоматический (mock battery).

### TC-32-017: Game + Energy — conflict resolution
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P1
**Шаги:**
1. Game Mode active. 2. Battery < 10%.
**Ожидаемый результат:**
- Game Mode priority. Warning shown.
**Автоматизация:** автоматический.

### TC-32-018: Profile — custom energy profile
**Тип:** Integration | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P1
**Шаги:**
1. Create custom profile.
**Ожидаемый результат:**
- Thresholds customizable.
**Автоматизация:** автоматический.
