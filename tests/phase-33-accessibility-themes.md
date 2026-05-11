# Тесты: Этап 33 — Accessibility & Themes

> High contrast, font scaling, reduced motion, color blindness filters, screen reader, theme engine, custom themes. Все тесты на реальном UI.

---

### TC-33-001: High contrast — on
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. High contrast on. 2. Open Command Bar.
**Ожидаемый результат:**
- Contrast > 7:1 (WCAG AAA).
**Автоматизация:** автоматический.

### TC-33-002: High contrast — off
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. High contrast off.
**Ожидаемый результат:**
- Normal contrast.
**Автоматизация:** автоматический.

### TC-33-003: Font scaling — 100%
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Scale = 1.0.
**Ожидаемый результат:**
- Base size.
**Автоматизация:** автоматический.

### TC-33-004: Font scaling — 150%
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Scale = 1.5.
**Ожидаемый результат:**
- 1.5x size. Layout intact.
**Автоматизация:** автоматический.

### TC-33-005: Font scaling — 200%
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Scale = 2.0.
**Ожидаемый результат:**
- 2x size. No clipping.
**Автоматизация:** автоматический.

### TC-33-006: Reduced motion — on
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Reduced motion on. 2. Open Command Bar.
**Ожидаемый результат:**
- Instant. No animation.
**Автоматизация:** автоматический.

### TC-33-007: Reduced motion — off
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Reduced motion off.
**Ожидаемый результат:**
- Animations present.
**Автоматизация:** автоматический.

### TC-33-008: Color blindness — deuteranopia
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Deuteranopia filter. 2. Red/green UI.
**Ожидаемый результат:**
- Red/green различимы.
**Автоматизация:** автоматический.

### TC-33-009: Color blindness — protanopia
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P1
**Шаги:**
1. Protanopia filter.
**Ожидаемый результат:**
- Colors adjusted.
**Автоматизация:** автоматический.

### TC-33-010: Color blindness — tritanopia
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P1
**Шаги:**
1. Tritanopia filter.
**Ожидаемый результат:**
- Blue/yellow различимы.
**Автоматизация:** автоматический.

### TC-33-011: Screen reader — focusable elements
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Tab через все элементы.
**Ожидаемый результат:**
- All announced. Focus order logical.
**Автоматизация:** полуавтоматический.

### TC-33-012: Screen reader — labels
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Check all buttons have labels.
**Ожидаемый результат:**
- No unlabeled buttons.
**Автоматизация:** автоматический.

### TC-33-013: Screen reader — dynamic content
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P1
**Шаги:**
1. New notification arrives.
**Ожидаемый результат:**
- Announced.
**Автоматизация:** полуавтоматический.

### TC-33-014: Theme engine — load custom
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Load custom theme JSON.
**Ожидаемый результат:**
- All UI elements: new colors.
**Автоматизация:** автоматический.

### TC-33-015: Theme engine — hot reload
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P1
**Шаги:**
1. Edit theme file. 2. Wait.
**Ожидаемый результат:**
- Auto reload. No restart.
**Автоматизация:** автоматический.

### TC-33-016: Theme engine — invalid theme
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P1
**Шаги:**
1. Invalid JSON.
**Ожидаемый результат:**
- Error logged. Fallback theme.
**Автоматизация:** автоматический.

### TC-33-017: Keyboard navigation — arrow keys
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Arrow keys в списке.
**Ожидаемый результат:**
- Navigation works.
**Автоматизация:** автоматический.

### TC-33-018: Keyboard navigation — Enter/Space
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Enter на кнопке.
**Ожидаемый результат:**
- Action triggered.
**Автоматизация:** автоматический.

### TC-33-019: Keyboard navigation — Escape
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Escape в диалоге.
**Ожидаемый результат:**
- Dialog closed.
**Автоматизация:** автоматический.

### TC-33-020: Accessibility — system follows
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P1
**Шаги:**
1. System high contrast on.
**Ожидаемый результат:**
- CORE follows.
**Автоматизация:** автоматический.
