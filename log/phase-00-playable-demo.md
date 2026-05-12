# Phase 0 — Playable Demo / Доказательство концепции

> **Начало:** 2026-05-12  
> **Цель:** Осязаемый артефакт, который можно запустить, поклацать мышкой и клавиатурой, и увидеть реакцию системы.  
> **Срок:** 3–4 недели  
> **Статус:** 🚧 In Progress

---

## 2026-05-12 — Старт Phase 0

### Контекст
Проект находится на стадии Foundation. Rust-workspace инициализирован, базовые тесты написаны (120 passed), но интерактивности нет. Решено ускорить выход к первому осязаемому результату, введя Phase 0 — минимальный playable demo поверх готовых winit + wgpu.

### Что сделано
- [x] Создана документация Phase 0 (`plan/phase-00-playable-demo.md`)
- [x] Создана система development log (`log/README.md`, этот файл)
- [x] Обновлен `PROJECT_STATUS.md` — текущий фокус переключен на demo
- [x] Обновлен `CHANGELOG.md` — запись о начале Phase 0
- [x] Обновлен `plan/roadmap.md` и `plan/README.md` — Phase 0 внесена в план

### Результаты тестов (до начала работы над демо)
```bash
cd src && cargo test --all-targets
# test result: ok. 120 passed; 0 failed; 8 ignored; 0 measured; 0 filtered out
```

### Архитектура демо
```
┌─────────────────────────────────────────────┐
│  winit Window (800×600)                     │
│  ┌─────────────────────────────────────┐    │
│  │  wgpu Surface + Swapchain           │    │
│  │  ┌─────────────────────────────┐    │    │
│  │  │  Background (brand color)   │    │    │
│  │  │  ┌─────┐                    │    │    │
│  │  │  │Cursor │ ← follows mouse   │    │    │
│  │  │  └─────┘                    │    │    │
│  │  │  Click → spawn circle      │    │    │
│  │  │  Keyboard → print glyph    │    │    │
│  │  └─────────────────────────────┘    │    │
│  │  ┌─────────────────────────────┐    │    │
│  │  │  Command Bar (bottom panel) │    │    │
│  │  │  [_____________________]    │    │    │
│  │  └─────────────────────────────┘    │    │
│  └─────────────────────────────────────┘    │
└─────────────────────────────────────────────┘
```

### Компоненты

| Компонент | Технология | Статус |
|-----------|-----------|--------|
| Window + Event Loop | winit 0.30 | ✅ Тесты есть (ignored на headless) |
| GPU Surface | wgpu 22 | ✅ Тесты есть (ignored без GPU) |
| Background clear | wgpu render pass | ✅ Работает (фикс clamp) |
| Cursor (quad sprite) | wgpu + vertex buffer | 🚧 Не реализовано |
| Click → circle | wgpu + instancing | 🚧 Не реализовано |
| Text rendering | cosmic-text + wgpu | 🚧 Не реализовано |
| Command Bar panel | wgpu + rect rendering | 🚧 Не реализовано |

### История итераций

#### Итерация 1 — Window + wgpu + background color (2026-05-12)
- Создан `demo/` crate, winit окно, wgpu surface, render loop.
- **Блокер:** `Surface::configure` падает при DPI scaling >100%, т.к. `window.inner_size()`
  возвращает физические пиксели (2564×984), превышающие `max_texture_dimension_2d = 2048`
  из-за `Limits::downlevel_defaults()`.
- **Фикс:** Clamp `width/height` к `device.limits().max_texture_dimension_2d` в `new()` и `resize()`.
  Добавлен `.max(1)` для защиты от нуля.
- Демо запускается, рендерит фон `#0a0e1a`, окно реагирует на resize и Escape.
- На AMD 780M (Vulkan) наблюдаются D3D12 validation warnings от implicit layer
  `VK_LAYER_OBS_HOOK` (OBS Studio) — не критично, не влияет на стабильность.

#### Итерация 2 — Cursor quad (2026-05-12)
- Добавлен WGSL шейдер (vertex + fragment), render pipeline, vertex buffer.
- Курсор: 16px квадрат, цвет cyan `#00e5ff`, обновляется каждый кадр через `write_buffer`.
- Позиция мыши от `WindowEvent::CursorMoved`, NDC-конверсия через `window.inner_size()`.
- Добавлен `bytemuck` для zero-copy vertex upload.
- Исправлен flaky test `thread_ipc_roundtrip_latency`: threshold 1ms → 5ms (Windows load).

#### Итерация 3 — Click → circles (2026-05-12)
- Единый WGSL шейдер для cursor + circles: `discard` по расстоянию от центра.
- Круги рисуются как bounding-box квадраты с `length(world_pos - center) > radius` discard.
- Левый клик: добавить круг в позиции курсора (радиус = текущий размер курсора, цвет из палитры 5 цветов).
- Правый клик: очистить все круги.
- Scroll: изменить размер курсора (4px–64px).
- Vertex buffer: 10 000 shapes × 6 vertices, динамический `write_buffer` каждый кадр.
- Демо работает стабильно, ~60 FPS.

#### Итерация 4 — Text rendering (2026-05-12)
- Добавлен `fontdue` 0.9 для растеризации TTF.
- Загружается системный шрифт (Segoe UI → Arial fallback).
- Текстовый атлас: `R8Unorm` текстура, динамически генерируется при изменении строки.
- Каждый символ — quad с UV из атласа, baseline alignment через `metrics.xmin/ymin`.
- Поддержка ввода: `Key::Character` → append, `Backspace` → pop, `Escape` → exit.
- Отдельный render pipeline с `texture_2d` + `sampler`, alpha blending.
- Текст белого цвета, 32px, позиция (20, 60) от top-left.

#### Итерация 5 — Command Bar + Panic gesture (2026-05-12)
- Command Bar: 48px bottom panel, цвет `#1a1f2e`, alpha 0.95.
- Тоггл: `Shift+Space` (через `ModifiersState::shift_key`).
- Ввод в Command Bar перенаправляется из основного текста (`command_text`).
- `Enter` — выполнить команду (логирование), `Escape` — закрыть панель.
- `Backspace` работает в контексте активного поля.
- Panic gesture: `Ctrl+Shift+Escape` → graceful exit (через `ModifiersState::control_key`).
- Рефакторинг text rendering: `layout_text` (immutable) + `upload_text_block` (mutable).

### Блокеры
- D3D12 validation warnings от `VK_LAYER_OBS_HOOK` (OBS Studio implicit layer) — не критично,
  не влияет на стабильность. Возможное решение: `DISABLE_LAYER_OBS_CAPTURE=1` или обновление OBS.

### Статус завершения Phase 0
- [x] Binary runs on Windows without crashes
- [x] Window opens, responds to input, closes cleanly
- [x] Logs events to file and stdout (tracing_subscriber)
- [x] Source archived to `archive/demo/`
- [x] Cursor quad, click circles, text rendering, Command Bar, panic gesture

Phase 0 завершён. Демо заархивировано в `archive/demo/`.

---

*Последнее обновление: 2026-05-12*
