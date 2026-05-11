# Тесты: Этап 29 — Intent API & AI Core

> NLU parser, Smart Scheduler, Generative UI, Cloud Bridge, SLM, privacy, 100 intents/sec. Все тесты на реальных данных и моделях.

---

### TC-29-001: NLU — "open calculator"
**Тип:** Unit | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. "open calculator".
**Ожидаемый результат:**
- Intent: `app.open`. Entity: `app_name = "calculator"`. Confidence > 0.9.
**Автоматизация:** автоматический.

### TC-29-002: NLU — "создай проект Альфа"
**Тип:** Unit | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. "создай проект Альфа".
**Ожидаемый результат:**
- Intent: `project.create`. Entity: `project_name = "Альфа"`.
**Автоматизация:** автоматический.

### TC-29-003: NLU — "show me the weather in Tokyo"
**Тип:** Unit | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. "show me the weather in Tokyo".
**Ожидаемый результат:**
- Intent: `weather.show`. Entity: `city = "Tokyo"`.
**Автоматизация:** автоматический.

### TC-29-004: NLU — ambiguous "run"
**Тип:** Unit | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. "run".
**Ожидаемый результат:**
- Top 3 intents. Disambiguation.
**Автоматизация:** автоматический.

### TC-29-005: NLU — unknown
**Тип:** Unit | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. "asdfghjkl".
**Ожидаемый результат:**
- Confidence < 0.3. "I don't understand".
**Автоматизация:** автоматический.

### TC-29-006: NLU — typo tolerance
**Тип:** Unit | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P1
**Шаги:**
1. "opn calcultor".
**Ожидаемый результат:**
- Intent: `app.open`. Entity: "calculator".
**Автоматизация:** автоматический.

### TC-29-007: Smart Scheduler — "remind me tomorrow"
**Тип:** Integration | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. "remind me to call mom tomorrow at 3 PM".
**Ожидаемый результат:**
- Reminder создан. Time: tomorrow 15:00.
**Автоматизация:** автоматический.

### TC-29-008: Smart Scheduler — "next Monday"
**Тип:** Integration | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. "remind me next Monday".
**Ожидаемый результат:**
- Date = next Monday.
**Автоматизация:** автоматический.

### TC-29-009: Smart Scheduler — recurring weekly
**Тип:** Integration | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. "every Monday at 9 AM standup".
**Ожидаемый результат:**
- Recurring: weekly, Monday, 9:00.
**Автоматизация:** автоматический.

### TC-29-010: Smart Scheduler — recurring daily
**Тип:** Integration | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P1
**Шаги:**
1. "every day at 8 AM".
**Ожидаемый результат:**
- Daily, 8:00.
**Автоматизация:** автоматический.

### TC-29-011: Smart Scheduler — conflict detection
**Тип:** Integration | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P1
**Шаги:**
1. "schedule meeting at 3 PM". 2. Already busy at 3 PM.
**Ожидаемый результат:**
- Warning: "You have a conflict".
**Автоматизация:** автоматический.

### TC-29-012: Generative UI — "show weather"
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. "show weather in Moscow".
**Ожидаемый результат:**
- UI: temp, icon, forecast. Layout корректен.
**Автоматизация:** автоматический.

### TC-29-013: Generative UI — "create dashboard"
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. "create dashboard with sales chart and recent orders".
**Ожидаемый результат:**
- Dashboard: chart + table. Component types корректны.
**Автоматизация:** автоматический.

### TC-29-014: Generative UI — invalid request
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P1
**Шаги:**
1. "create something impossible".
**Ожидаемый результат:**
- Graceful fallback. Default layout.
**Автоматизация:** автоматический.

### TC-29-015: Cloud Bridge — GPT-4 online
**Тип:** Integration | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Online. 2. Запрос к GPT-4.
**Ожидаемый результат:**
- Ответ. Detailed. Quality high.
**Автоматизация:** автоматический.

### TC-29-016: Cloud Bridge — SLM offline
**Тип:** Integration | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Offline. 2. Запрос.
**Ожидаемый результат:**
- SLM отвечает. Быстро. Простой ответ.
**Автоматизация:** автоматический.

### TC-29-017: Cloud Bridge — fallback GPT-4→SLM
**Тип:** Integration | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Online. 2. GPT-4 timeout. 3. Fallback SLM.
**Ожидаемый результат:**
- SLM отвечает. "(offline mode)" indicator.
**Автоматизация:** автоматический.

### TC-29-018: Cloud Bridge — cache
**Тип:** Integration | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P1
**Шаги:**
1. Запрос "weather Moscow". 2. Повторный запрос.
**Ожидаемый результат:**
- Второй: cached. < 10 мс.
**Автоматизация:** автоматический.

### TC-29-019: Privacy — prompt не логируется
**Тип:** Integration | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Send prompt. 2. Check logs.
**Ожидаемый результат:**
- Prompt не в логах. Anonymized ID only.
**Автоматизация:** автоматический.

### TC-29-020: Privacy — local SLM
**Тип:** Integration | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Offline mode. 2. tcpdump.
**Ожидаемый результат:**
- Нет исходящего трафика.
**Автоматизация:** автоматический.

### TC-29-021: Performance — 100 intents/sec
**Тип:** Stress | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. 100 intents/sec, 10 сек.
**Ожидаемый результат:**
- Latency < 100 мс/intent. Нет потерь.
**Автоматизация:** автоматический.

### TC-29-022: Performance — Generative UI < 2 sec
**Тип:** Performance | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. "create dashboard".
**Ожидаемый результат:**
- UI generated < 2 сек.
**Автоматизация:** автоматический.

### TC-29-023: Context — remember previous
**Тип:** Integration | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P1
**Шаги:**
1. "show weather in Moscow". 2. "what about tomorrow?".
**Ожидаемый результат:**
- Context: Moscow. Forecast tomorrow.
**Автоматизация:** автоматический.

### TC-29-024: Context — reset
**Тип:** Integration | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P1
**Шаги:**
1. "reset context". 2. "what about tomorrow?".
**Ожидаемый результат:**
- "Please specify city".
**Автоматизация:** автоматический.
